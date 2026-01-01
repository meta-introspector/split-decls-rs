/* FP:mod.rs-0001 */ // A framework that can express both [gen-kill] and generic dataflow problems.
/* FP:mod.rs-0002 */ //
/* FP:mod.rs-0003 */ // To use this framework, implement the [`Analysis`] trait. There used to be a `GenKillAnalysis`
/* FP:mod.rs-0004 */ // alternative trait for gen-kill analyses that would pre-compute the transfer function for each
/* FP:mod.rs-0005 */ // block. It was intended as an optimization, but it ended up not being any faster than
/* FP:mod.rs-0006 */ // `Analysis`.
/* FP:mod.rs-0007 */ //
/* FP:mod.rs-0008 */ // The `impls` module contains several examples of dataflow analyses.
/* FP:mod.rs-0009 */ //
/* FP:mod.rs-0010 */ // Then call `iterate_to_fixpoint` on your type that impls `Analysis` to get a `Results`. From
/* FP:mod.rs-0011 */ // there, you can use a `ResultsCursor` to inspect the fixpoint solution to your dataflow problem
/* FP:mod.rs-0012 */ // (good for inspecting a small number of locations), or implement the `ResultsVisitor` interface
/* FP:mod.rs-0013 */ // and use `visit_results` (good for inspecting many or all locations). The following example uses
/* FP:mod.rs-0014 */ // the `ResultsCursor` approach.
/* FP:mod.rs-0015 */ //
/* FP:mod.rs-0016 */ // ```ignore (cross-crate-imports)
/* FP:mod.rs-0017 */ // use rustc_const_eval::dataflow::Analysis; // Makes `iterate_to_fixpoint` available.
/* FP:mod.rs-0018 */ //
/* FP:mod.rs-0019 */ // fn do_my_analysis(tcx: TyCtxt<'tcx>, body: &mir::Body<'tcx>) {
/* FP:mod.rs-0020 */ //     let analysis = MyAnalysis::new()
/* FP:mod.rs-0021 */ //         .iterate_to_fixpoint(tcx, body, None)
/* FP:mod.rs-0022 */ //         .into_results_cursor(body);
/* FP:mod.rs-0023 */ //
/* FP:mod.rs-0024 */ //     // Print the dataflow state *after* each statement in the start block.
/* FP:mod.rs-0025 */ //     for (_, statement_index) in body.block_data[START_BLOCK].statements.iter_enumerated() {
/* FP:mod.rs-0026 */ //         cursor.seek_after(Location { block: START_BLOCK, statement_index });
/* FP:mod.rs-0027 */ //         let state = cursor.get();
/* FP:mod.rs-0028 */ //         println!("{:?}", state);
/* FP:mod.rs-0029 */ //     }
/* FP:mod.rs-0030 */ // }
/* FP:mod.rs-0031 */ // ```
/* FP:mod.rs-0032 */ //
/* FP:mod.rs-0033 */ // [gen-kill]: https://en.wikipedia.org/wiki/Data-flow_analysis#Bit_vector_problems
/* FP:mod.rs-0034 */ 
/* FP:mod.rs-0035 */ use std::cmp::Ordering;
/* FP:mod.rs-0036 */ 
/* FP:mod.rs-0037 */ use crate::rustc_data_structures::work_queue::WorkQueue;
/* FP:mod.rs-0038 */ use crate::rustc_index::bit_set::{DenseBitSet, MixedBitSet};
/* FP:mod.rs-0039 */ use crate::rustc_index::{Idx, IndexVec};
/* FP:mod.rs-0040 */ use crate::rustc_complete::bug;
/* FP:mod.rs-0041 */ use crate::rustc_complete::mir::{
/* FP:mod.rs-0042 */     self, BasicBlock, CallReturnPlaces, Location, SwitchTargetValue, TerminatorEdges, traversal,
/* FP:mod.rs-0043 */ };
/* FP:mod.rs-0044 */ use crate::rustc_complete::ty::TyCtxt;
/* FP:mod.rs-0045 */ use tracing::error;
/* FP:mod.rs-0046 */ 
/* FP:mod.rs-0047 */ use self::graphviz::write_graphviz_results;
/* FP:mod.rs-0048 */ use super::fmt::DebugWithContext;
/* FP:mod.rs-0049 */ 
/* FP:mod.rs-0057 */ 
/* FP:mod.rs-0058 */ pub use self::cursor::ResultsCursor;
/* FP:mod.rs-0059 */ pub use self::direction::{Backward, Direction, Forward};
/* FP:mod.rs-0060 */ pub use self::lattice::{JoinSemiLattice, MaybeReachable};
/* FP:mod.rs-0061 */ pub(crate) use self::results::AnalysisAndResults;
/* FP:mod.rs-0062 */ pub use self::results::Results;
/* FP:mod.rs-0063 */ pub use self::visitor::{ResultsVisitor, visit_reachable_results, visit_results};
/* FP:mod.rs-0064 */ 
/* FP:mod.rs-0065 */ /// Analysis domains are all bitsets of various kinds. This trait holds
/* FP:mod.rs-0066 */ /// operations needed by all of them.
/* FP:mod.rs-0067 */ pub trait BitSetExt<T> {
/* FP:mod.rs-0068 */     fn contains(&self, elem: T) -> bool;
/* FP:mod.rs-0069 */ }
/* FP:mod.rs-0070 */ 
/* FP:mod.rs-0071 */ impl<T: Idx> BitSetExt<T> for DenseBitSet<T> {
/* FP:mod.rs-0072 */     fn contains(&self, elem: T) -> bool {
/* FP:mod.rs-0073 */         self.contains(elem)
/* FP:mod.rs-0074 */     }
/* FP:mod.rs-0075 */ }
/* FP:mod.rs-0076 */ 
/* FP:mod.rs-0077 */ impl<T: Idx> BitSetExt<T> for MixedBitSet<T> {
/* FP:mod.rs-0078 */     fn contains(&self, elem: T) -> bool {
/* FP:mod.rs-0079 */         self.contains(elem)
/* FP:mod.rs-0080 */     }
/* FP:mod.rs-0081 */ }
/* FP:mod.rs-0082 */ 
/* FP:mod.rs-0083 */ /// A dataflow problem with an arbitrarily complex transfer function.
/* FP:mod.rs-0084 */ ///
/* FP:mod.rs-0085 */ /// This trait specifies the lattice on which this analysis operates (the domain), its
/* FP:mod.rs-0086 */ /// initial value at the entry point of each basic block, and various operations.
/* FP:mod.rs-0087 */ ///
/* FP:mod.rs-0088 */ /// # Convergence
/* FP:mod.rs-0089 */ ///
/* FP:mod.rs-0090 */ /// When implementing this trait it's possible to choose a transfer function such that the analysis
/* FP:mod.rs-0091 */ /// does not reach fixpoint. To guarantee convergence, your transfer functions must maintain the
/* FP:mod.rs-0092 */ /// following invariant:
/* FP:mod.rs-0093 */ ///
/* FP:mod.rs-0094 */ /// > If the dataflow state **before** some point in the program changes to be greater
/* FP:mod.rs-0095 */ /// than the prior state **before** that point, the dataflow state **after** that point must
/* FP:mod.rs-0096 */ /// also change to be greater than the prior state **after** that point.
/* FP:mod.rs-0097 */ ///
/* FP:mod.rs-0098 */ /// This invariant guarantees that the dataflow state at a given point in the program increases
/* FP:mod.rs-0099 */ /// monotonically until fixpoint is reached. Note that this monotonicity requirement only applies
/* FP:mod.rs-0100 */ /// to the same point in the program at different points in time. The dataflow state at a given
/* FP:mod.rs-0101 */ /// point in the program may or may not be greater than the state at any preceding point.
/* FP:mod.rs-0102 */ pub trait Analysis<'tcx> {
/* FP:mod.rs-0103 */     /// The type that holds the dataflow state at any given point in the program.
/* FP:mod.rs-0104 */     type Domain: Clone + JoinSemiLattice;
/* FP:mod.rs-0105 */ 
/* FP:mod.rs-0106 */     /// The direction of this analysis. Either `Forward` or `Backward`.
/* FP:mod.rs-0107 */     type Direction: Direction = Forward;
/* FP:mod.rs-0108 */ 
/* FP:mod.rs-0109 */     /// Auxiliary data used for analyzing `SwitchInt` terminators, if necessary.
/* FP:mod.rs-0110 */     type SwitchIntData = !;
/* FP:mod.rs-0111 */ 
/* FP:mod.rs-0112 */     /// A descriptive name for this analysis. Used only for debugging.
/* FP:mod.rs-0113 */     ///
/* FP:mod.rs-0114 */     /// This name should be brief and contain no spaces, periods or other characters that are not
/* FP:mod.rs-0115 */     /// suitable as part of a filename.
/* FP:mod.rs-0116 */     const NAME: &'static str;
/* FP:mod.rs-0117 */ 
/* FP:mod.rs-0118 */     /// Returns the initial value of the dataflow state upon entry to each basic block.
/* FP:mod.rs-0119 */     fn bottom_value(&self, body: &mir::Body<'tcx>) -> Self::Domain;
/* FP:mod.rs-0120 */ 
/* FP:mod.rs-0121 */     /// Mutates the initial value of the dataflow state upon entry to the `START_BLOCK`.
/* FP:mod.rs-0122 */     ///
/* FP:mod.rs-0123 */     /// For backward analyses, initial state (besides the bottom value) is not yet supported. Trying
/* FP:mod.rs-0124 */     /// to mutate the initial state will result in a panic.
/* FP:mod.rs-0125 */     //
/* FP:mod.rs-0126 */     // FIXME: For backward dataflow analyses, the initial state should be applied to every basic
/* FP:mod.rs-0127 */     // block where control flow could exit the MIR body (e.g., those terminated with `return` or
/* FP:mod.rs-0128 */     // `resume`). It's not obvious how to handle `yield` points in coroutines, however.
/* FP:mod.rs-0129 */     fn initialize_start_block(&self, body: &mir::Body<'tcx>, state: &mut Self::Domain);
/* FP:mod.rs-0130 */ 
/* FP:mod.rs-0131 */     /// Updates the current dataflow state with an "early" effect, i.e. one
/* FP:mod.rs-0132 */     /// that occurs immediately before the given statement.
/* FP:mod.rs-0133 */     ///
/* FP:mod.rs-0134 */     /// This method is useful if the consumer of the results of this analysis only needs to observe
/* FP:mod.rs-0135 */     /// *part* of the effect of a statement (e.g. for two-phase borrows). As a general rule,
/* FP:mod.rs-0136 */     /// analyses should not implement this without also implementing
/* FP:mod.rs-0137 */     /// `apply_primary_statement_effect`.
/* FP:mod.rs-0138 */     fn apply_early_statement_effect(
/* FP:mod.rs-0139 */         &mut self,
/* FP:mod.rs-0140 */         _state: &mut Self::Domain,
/* FP:mod.rs-0141 */         _statement: &mir::Statement<'tcx>,
/* FP:mod.rs-0142 */         _location: Location,
/* FP:mod.rs-0143 */     ) {
/* FP:mod.rs-0144 */     }
/* FP:mod.rs-0145 */ 
/* FP:mod.rs-0146 */     /// Updates the current dataflow state with the effect of evaluating a statement.
/* FP:mod.rs-0147 */     fn apply_primary_statement_effect(
/* FP:mod.rs-0148 */         &mut self,
/* FP:mod.rs-0149 */         state: &mut Self::Domain,
/* FP:mod.rs-0150 */         statement: &mir::Statement<'tcx>,
/* FP:mod.rs-0151 */         location: Location,
/* FP:mod.rs-0152 */     );
/* FP:mod.rs-0153 */ 
/* FP:mod.rs-0154 */     /// Updates the current dataflow state with an effect that occurs immediately *before* the
/* FP:mod.rs-0155 */     /// given terminator.
/* FP:mod.rs-0156 */     ///
/* FP:mod.rs-0157 */     /// This method is useful if the consumer of the results of this analysis needs only to observe
/* FP:mod.rs-0158 */     /// *part* of the effect of a terminator (e.g. for two-phase borrows). As a general rule,
/* FP:mod.rs-0159 */     /// analyses should not implement this without also implementing
/* FP:mod.rs-0160 */     /// `apply_primary_terminator_effect`.
/* FP:mod.rs-0161 */     fn apply_early_terminator_effect(
/* FP:mod.rs-0162 */         &mut self,
/* FP:mod.rs-0163 */         _state: &mut Self::Domain,
/* FP:mod.rs-0164 */         _terminator: &mir::Terminator<'tcx>,
/* FP:mod.rs-0165 */         _location: Location,
/* FP:mod.rs-0166 */     ) {
/* FP:mod.rs-0167 */     }
/* FP:mod.rs-0168 */ 
/* FP:mod.rs-0169 */     /// Updates the current dataflow state with the effect of evaluating a terminator.
/* FP:mod.rs-0170 */     ///
/* FP:mod.rs-0171 */     /// The effect of a successful return from a `Call` terminator should **not** be accounted for
/* FP:mod.rs-0172 */     /// in this function. That should go in `apply_call_return_effect`. For example, in the
/* FP:mod.rs-0173 */     /// `InitializedPlaces` analyses, the return place for a function call is not marked as
/* FP:mod.rs-0174 */     /// initialized here.
/* FP:mod.rs-0175 */     fn apply_primary_terminator_effect<'mir>(
/* FP:mod.rs-0176 */         &mut self,
/* FP:mod.rs-0177 */         _state: &mut Self::Domain,
/* FP:mod.rs-0178 */         terminator: &'mir mir::Terminator<'tcx>,
/* FP:mod.rs-0179 */         _location: Location,
/* FP:mod.rs-0180 */     ) -> TerminatorEdges<'mir, 'tcx> {
/* FP:mod.rs-0181 */         terminator.edges()
/* FP:mod.rs-0182 */     }
/* FP:mod.rs-0183 */ 
/* FP:mod.rs-0184 */     /* Edge-specific effects */
/* FP:mod.rs-0185 */ 
/* FP:mod.rs-0186 */     /// Updates the current dataflow state with the effect of a successful return from a `Call`
/* FP:mod.rs-0187 */     /// terminator.
/* FP:mod.rs-0188 */     ///
/* FP:mod.rs-0189 */     /// This is separate from `apply_primary_terminator_effect` to properly track state across
/* FP:mod.rs-0190 */     /// unwind edges.
/* FP:mod.rs-0191 */     fn apply_call_return_effect(
/* FP:mod.rs-0192 */         &mut self,
/* FP:mod.rs-0193 */         _state: &mut Self::Domain,
/* FP:mod.rs-0194 */         _block: BasicBlock,
/* FP:mod.rs-0195 */         _return_places: CallReturnPlaces<'_, 'tcx>,
/* FP:mod.rs-0196 */     ) {
/* FP:mod.rs-0197 */     }
/* FP:mod.rs-0198 */ 
/* FP:mod.rs-0199 */     /// Used to update the current dataflow state with the effect of taking a particular branch in
/* FP:mod.rs-0200 */     /// a `SwitchInt` terminator.
/* FP:mod.rs-0201 */     ///
/* FP:mod.rs-0202 */     /// Unlike the other edge-specific effects, which are allowed to mutate `Self::Domain`
/* FP:mod.rs-0203 */     /// directly, overriders of this method must return a `Self::SwitchIntData` value (wrapped in
/* FP:mod.rs-0204 */     /// `Some`). The `apply_switch_int_edge_effect` method will then be called once for each
/* FP:mod.rs-0205 */     /// outgoing edge and will have access to the dataflow state that will be propagated along that
/* FP:mod.rs-0206 */     /// edge, and also the `Self::SwitchIntData` value.
/* FP:mod.rs-0207 */     ///
/* FP:mod.rs-0208 */     /// This interface is somewhat more complex than the other visitor-like "effect" methods.
/* FP:mod.rs-0209 */     /// However, it is both more ergonomic—callers don't need to recompute or cache information
/* FP:mod.rs-0210 */     /// about a given `SwitchInt` terminator for each one of its edges—and more efficient—the
/* FP:mod.rs-0211 */     /// engine doesn't need to clone the exit state for a block unless
/* FP:mod.rs-0212 */     /// `get_switch_int_data` is actually called.
/* FP:mod.rs-0213 */     fn get_switch_int_data(
/* FP:mod.rs-0214 */         &mut self,
/* FP:mod.rs-0215 */         _block: mir::BasicBlock,
/* FP:mod.rs-0216 */         _discr: &mir::Operand<'tcx>,
/* FP:mod.rs-0217 */     ) -> Option<Self::SwitchIntData> {
/* FP:mod.rs-0218 */         None
/* FP:mod.rs-0219 */     }
/* FP:mod.rs-0220 */ 
/* FP:mod.rs-0221 */     /// See comments on `get_switch_int_data`.
/* FP:mod.rs-0222 */     fn apply_switch_int_edge_effect(
/* FP:mod.rs-0223 */         &mut self,
/* FP:mod.rs-0224 */         _data: &mut Self::SwitchIntData,
/* FP:mod.rs-0225 */         _state: &mut Self::Domain,
/* FP:mod.rs-0226 */         _value: SwitchTargetValue,
/* FP:mod.rs-0227 */         _targets: &mir::SwitchTargets,
/* FP:mod.rs-0228 */     ) {
/* FP:mod.rs-0229 */         unreachable!();
/* FP:mod.rs-0230 */     }
/* FP:mod.rs-0231 */ 
/* FP:mod.rs-0232 */     /* Extension methods */
/* FP:mod.rs-0233 */ 
/* FP:mod.rs-0234 */     /// Finds the fixpoint for this dataflow problem.
/* FP:mod.rs-0235 */     ///
/* FP:mod.rs-0236 */     /// You shouldn't need to override this. Its purpose is to enable method chaining like so:
/* FP:mod.rs-0237 */     ///
/* FP:mod.rs-0238 */     /// ```ignore (cross-crate-imports)
/* FP:mod.rs-0239 */     /// let results = MyAnalysis::new(tcx, body)
/* FP:mod.rs-0240 */     ///     .iterate_to_fixpoint(tcx, body, None)
/* FP:mod.rs-0241 */     ///     .into_results_cursor(body);
/* FP:mod.rs-0242 */     /// ```
/* FP:mod.rs-0243 */     /// You can optionally add a `pass_name` to the graphviz output for this particular run of a
/* FP:mod.rs-0244 */     /// dataflow analysis. Some analyses are run multiple times in the compilation pipeline.
/* FP:mod.rs-0245 */     /// Without a `pass_name` to differentiates them, only the results for the latest run will be
/* FP:mod.rs-0246 */     /// saved.
/* FP:mod.rs-0247 */     fn iterate_to_fixpoint<'mir>(
/* FP:mod.rs-0248 */         mut self,
/* FP:mod.rs-0249 */         tcx: TyCtxt<'tcx>,
/* FP:mod.rs-0250 */         body: &'mir mir::Body<'tcx>,
/* FP:mod.rs-0251 */         pass_name: Option<&'static str>,
/* FP:mod.rs-0252 */     ) -> AnalysisAndResults<'tcx, Self>
/* FP:mod.rs-0253 */     where
/* FP:mod.rs-0254 */         Self: Sized,
/* FP:mod.rs-0255 */         Self::Domain: DebugWithContext<Self>,
/* FP:mod.rs-0256 */     {
/* FP:mod.rs-0257 */         let mut results = IndexVec::from_fn_n(|_| self.bottom_value(body), body.basic_blocks.len());
/* FP:mod.rs-0258 */         self.initialize_start_block(body, &mut results[mir::START_BLOCK]);
/* FP:mod.rs-0259 */ 
/* FP:mod.rs-0260 */         if Self::Direction::IS_BACKWARD && results[mir::START_BLOCK] != self.bottom_value(body) {
/* FP:mod.rs-0261 */             bug!("`initialize_start_block` is not yet supported for backward dataflow analyses");
/* FP:mod.rs-0262 */         }
/* FP:mod.rs-0263 */ 
/* FP:mod.rs-0264 */         let mut dirty_queue: WorkQueue<BasicBlock> = WorkQueue::with_none(body.basic_blocks.len());
/* FP:mod.rs-0265 */ 
/* FP:mod.rs-0266 */         if Self::Direction::IS_FORWARD {
/* FP:mod.rs-0267 */             for (bb, _) in traversal::reverse_postorder(body) {
/* FP:mod.rs-0268 */                 dirty_queue.insert(bb);
/* FP:mod.rs-0269 */             }
/* FP:mod.rs-0270 */         } else {
/* FP:mod.rs-0271 */             // Reverse post-order on the reverse CFG may generate a better iteration order for
/* FP:mod.rs-0272 */             // backward dataflow analyses, but probably not enough to matter.
/* FP:mod.rs-0273 */             for (bb, _) in traversal::postorder(body) {
/* FP:mod.rs-0274 */                 dirty_queue.insert(bb);
/* FP:mod.rs-0275 */             }
/* FP:mod.rs-0276 */         }
/* FP:mod.rs-0277 */ 
/* FP:mod.rs-0278 */         // `state` is not actually used between iterations;
/* FP:mod.rs-0279 */         // this is just an optimization to avoid reallocating
/* FP:mod.rs-0280 */         // every iteration.
/* FP:mod.rs-0281 */         let mut state = self.bottom_value(body);
/* FP:mod.rs-0282 */         while let Some(bb) = dirty_queue.pop() {
/* FP:mod.rs-0283 */             // Set the state to the entry state of the block. This is equivalent to `state =
/* FP:mod.rs-0284 */             // results[bb].clone()`, but it saves an allocation, thus improving compile times.
/* FP:mod.rs-0285 */             state.clone_from(&results[bb]);
/* FP:mod.rs-0286 */ 
/* FP:mod.rs-0287 */             Self::Direction::apply_effects_in_block(
/* FP:mod.rs-0288 */                 &mut self,
/* FP:mod.rs-0289 */                 body,
/* FP:mod.rs-0290 */                 &mut state,
/* FP:mod.rs-0291 */                 bb,
/* FP:mod.rs-0292 */                 &body[bb],
/* FP:mod.rs-0293 */                 |target: BasicBlock, state: &Self::Domain| {
/* FP:mod.rs-0294 */                     let set_changed = results[target].join(state);
/* FP:mod.rs-0295 */                     if set_changed {
/* FP:mod.rs-0296 */                         dirty_queue.insert(target);
/* FP:mod.rs-0297 */                     }
/* FP:mod.rs-0298 */                 },
/* FP:mod.rs-0299 */             );
/* FP:mod.rs-0300 */         }
/* FP:mod.rs-0301 */ 
/* FP:mod.rs-0302 */         if tcx.sess.opts.unstable_opts.dump_mir_dataflow {
/* FP:mod.rs-0303 */             let res = write_graphviz_results(tcx, body, &mut self, &results, pass_name);
/* FP:mod.rs-0304 */             if let Err(e) = res {
/* FP:mod.rs-0305 */                 error!("Failed to write graphviz dataflow results: {}", e);
/* FP:mod.rs-0306 */             }
/* FP:mod.rs-0307 */         }
/* FP:mod.rs-0308 */ 
/* FP:mod.rs-0309 */         AnalysisAndResults { analysis: self, results }
/* FP:mod.rs-0310 */     }
/* FP:mod.rs-0311 */ }
/* FP:mod.rs-0312 */ 
/* FP:mod.rs-0313 */ /// The legal operations for a transfer function in a gen/kill problem.
/* FP:mod.rs-0314 */ pub trait GenKill<T> {
/* FP:mod.rs-0315 */     /// Inserts `elem` into the state vector.
/* FP:mod.rs-0316 */     fn gen_(&mut self, elem: T);
/* FP:mod.rs-0317 */ 
/* FP:mod.rs-0318 */     /// Removes `elem` from the state vector.
/* FP:mod.rs-0319 */     fn kill(&mut self, elem: T);
/* FP:mod.rs-0320 */ 
/* FP:mod.rs-0321 */     /// Calls `gen` for each element in `elems`.
/* FP:mod.rs-0322 */     fn gen_all(&mut self, elems: impl IntoIterator<Item = T>) {
/* FP:mod.rs-0323 */         for elem in elems {
/* FP:mod.rs-0324 */             self.gen_(elem);
/* FP:mod.rs-0325 */         }
/* FP:mod.rs-0326 */     }
/* FP:mod.rs-0327 */ 
/* FP:mod.rs-0328 */     /// Calls `kill` for each element in `elems`.
/* FP:mod.rs-0329 */     fn kill_all(&mut self, elems: impl IntoIterator<Item = T>) {
/* FP:mod.rs-0330 */         for elem in elems {
/* FP:mod.rs-0331 */             self.kill(elem);
/* FP:mod.rs-0332 */         }
/* FP:mod.rs-0333 */     }
/* FP:mod.rs-0334 */ }
/* FP:mod.rs-0335 */ 
/* FP:mod.rs-0336 */ impl<T: Idx> GenKill<T> for DenseBitSet<T> {
/* FP:mod.rs-0337 */     fn gen_(&mut self, elem: T) {
/* FP:mod.rs-0338 */         self.insert(elem);
/* FP:mod.rs-0339 */     }
/* FP:mod.rs-0340 */ 
/* FP:mod.rs-0341 */     fn kill(&mut self, elem: T) {
/* FP:mod.rs-0342 */         self.remove(elem);
/* FP:mod.rs-0343 */     }
/* FP:mod.rs-0344 */ }
/* FP:mod.rs-0345 */ 
/* FP:mod.rs-0346 */ impl<T: Idx> GenKill<T> for MixedBitSet<T> {
/* FP:mod.rs-0347 */     fn gen_(&mut self, elem: T) {
/* FP:mod.rs-0348 */         self.insert(elem);
/* FP:mod.rs-0349 */     }
/* FP:mod.rs-0350 */ 
/* FP:mod.rs-0351 */     fn kill(&mut self, elem: T) {
/* FP:mod.rs-0352 */         self.remove(elem);
/* FP:mod.rs-0353 */     }
/* FP:mod.rs-0354 */ }
/* FP:mod.rs-0355 */ 
/* FP:mod.rs-0356 */ impl<T, S: GenKill<T>> GenKill<T> for MaybeReachable<S> {
/* FP:mod.rs-0357 */     fn gen_(&mut self, elem: T) {
/* FP:mod.rs-0358 */         match self {
/* FP:mod.rs-0359 */             // If the state is not reachable, adding an element does nothing.
/* FP:mod.rs-0360 */             MaybeReachable::Unreachable => {}
/* FP:mod.rs-0361 */             MaybeReachable::Reachable(set) => set.gen_(elem),
/* FP:mod.rs-0362 */         }
/* FP:mod.rs-0363 */     }
/* FP:mod.rs-0364 */ 
/* FP:mod.rs-0365 */     fn kill(&mut self, elem: T) {
/* FP:mod.rs-0366 */         match self {
/* FP:mod.rs-0367 */             // If the state is not reachable, killing an element does nothing.
/* FP:mod.rs-0368 */             MaybeReachable::Unreachable => {}
/* FP:mod.rs-0369 */             MaybeReachable::Reachable(set) => set.kill(elem),
/* FP:mod.rs-0370 */         }
/* FP:mod.rs-0371 */     }
/* FP:mod.rs-0372 */ }
/* FP:mod.rs-0373 */ 
/* FP:mod.rs-0374 */ // NOTE: DO NOT CHANGE VARIANT ORDER. The derived `Ord` impls rely on the current order.
/* FP:mod.rs-0375 */ #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/* FP:mod.rs-0376 */ enum Effect {
/* FP:mod.rs-0377 */     /// The "early" effect (e.g., `apply_early_statement_effect`) for a statement/terminator.
/* FP:mod.rs-0378 */     Early,
/* FP:mod.rs-0379 */ 
/* FP:mod.rs-0380 */     /// The "primary" effect (e.g., `apply_primary_statement_effect`) for a statement/terminator.
/* FP:mod.rs-0381 */     Primary,
/* FP:mod.rs-0382 */ }
/* FP:mod.rs-0383 */ 
/* FP:mod.rs-0384 */ impl Effect {
/* FP:mod.rs-0385 */     const fn at_index(self, statement_index: usize) -> EffectIndex {
/* FP:mod.rs-0386 */         EffectIndex { effect: self, statement_index }
/* FP:mod.rs-0387 */     }
/* FP:mod.rs-0388 */ }
/* FP:mod.rs-0389 */ 
/* FP:mod.rs-0390 */ #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/* FP:mod.rs-0391 */ pub struct EffectIndex {
/* FP:mod.rs-0392 */     statement_index: usize,
/* FP:mod.rs-0393 */     effect: Effect,
/* FP:mod.rs-0394 */ }
/* FP:mod.rs-0395 */ 
/* FP:mod.rs-0396 */ impl EffectIndex {
/* FP:mod.rs-0397 */     fn next_in_forward_order(self) -> Self {
/* FP:mod.rs-0398 */         match self.effect {
/* FP:mod.rs-0399 */             Effect::Early => Effect::Primary.at_index(self.statement_index),
/* FP:mod.rs-0400 */             Effect::Primary => Effect::Early.at_index(self.statement_index + 1),
/* FP:mod.rs-0401 */         }
/* FP:mod.rs-0402 */     }
/* FP:mod.rs-0403 */ 
/* FP:mod.rs-0404 */     fn next_in_backward_order(self) -> Self {
/* FP:mod.rs-0405 */         match self.effect {
/* FP:mod.rs-0406 */             Effect::Early => Effect::Primary.at_index(self.statement_index),
/* FP:mod.rs-0407 */             Effect::Primary => Effect::Early.at_index(self.statement_index - 1),
/* FP:mod.rs-0408 */         }
/* FP:mod.rs-0409 */     }
/* FP:mod.rs-0410 */ 
/* FP:mod.rs-0411 */     /// Returns `true` if the effect at `self` should be applied earlier than the effect at `other`
/* FP:mod.rs-0412 */     /// in forward order.
/* FP:mod.rs-0413 */     fn precedes_in_forward_order(self, other: Self) -> bool {
/* FP:mod.rs-0414 */         let ord = self
/* FP:mod.rs-0415 */             .statement_index
/* FP:mod.rs-0416 */             .cmp(&other.statement_index)
/* FP:mod.rs-0417 */             .then_with(|| self.effect.cmp(&other.effect));
/* FP:mod.rs-0418 */         ord == Ordering::Less
/* FP:mod.rs-0419 */     }
/* FP:mod.rs-0420 */ 
/* FP:mod.rs-0421 */     /// Returns `true` if the effect at `self` should be applied earlier than the effect at `other`
/* FP:mod.rs-0422 */     /// in backward order.
/* FP:mod.rs-0423 */     fn precedes_in_backward_order(self, other: Self) -> bool {
/* FP:mod.rs-0424 */         let ord = other
/* FP:mod.rs-0425 */             .statement_index
/* FP:mod.rs-0426 */             .cmp(&self.statement_index)
/* FP:mod.rs-0427 */             .then_with(|| self.effect.cmp(&other.effect));
/* FP:mod.rs-0428 */         ord == Ordering::Less
/* FP:mod.rs-0429 */     }
/* FP:mod.rs-0430 */ }
/* FP:mod.rs-0431 */ 
/* FP:mod.rs-0432 */ #[cfg(test)]