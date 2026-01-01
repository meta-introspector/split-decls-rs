/* FP:cursor.rs-0001 */ // Random access inspection of the results of a dataflow analysis.
/* FP:cursor.rs-0002 */ 
/* FP:cursor.rs-0003 */ use std::borrow::Cow;
/* FP:cursor.rs-0004 */ use std::cmp::Ordering;
/* FP:cursor.rs-0005 */ use std::ops::{Deref, DerefMut};
/* FP:cursor.rs-0006 */ 
/* FP:cursor.rs-0007 */ #[cfg(debug_assertions)]
/* FP:cursor.rs-0008 */ use crate::rustc_index::bit_set::DenseBitSet;
/* FP:cursor.rs-0009 */ use crate::rustc_complete::mir::{self, BasicBlock, Location};
/* FP:cursor.rs-0010 */ 
/* FP:cursor.rs-0011 */ use super::{Analysis, Direction, Effect, EffectIndex, Results};
/* FP:cursor.rs-0012 */ 
/* FP:cursor.rs-0013 */ /// Some `ResultsCursor`s want to own an `Analysis`, and some want to borrow an `Analysis`, either
/* FP:cursor.rs-0014 */ /// mutable or immutably. This type allows all of the above. It's similar to `Cow`, but `Cow`
/* FP:cursor.rs-0015 */ /// doesn't allow mutable borrowing.
/* FP:cursor.rs-0016 */ enum CowMut<'a, T> {
/* FP:cursor.rs-0017 */     BorrowedMut(&'a mut T),
/* FP:cursor.rs-0018 */     Owned(T),
/* FP:cursor.rs-0019 */ }
/* FP:cursor.rs-0020 */ 
/* FP:cursor.rs-0021 */ impl<T> Deref for CowMut<'_, T> {
/* FP:cursor.rs-0022 */     type Target = T;
/* FP:cursor.rs-0023 */ 
/* FP:cursor.rs-0024 */     fn deref(&self) -> &T {
/* FP:cursor.rs-0025 */         match self {
/* FP:cursor.rs-0026 */             CowMut::BorrowedMut(borrowed) => borrowed,
/* FP:cursor.rs-0027 */             CowMut::Owned(owned) => owned,
/* FP:cursor.rs-0028 */         }
/* FP:cursor.rs-0029 */     }
/* FP:cursor.rs-0030 */ }
/* FP:cursor.rs-0031 */ 
/* FP:cursor.rs-0032 */ impl<T> DerefMut for CowMut<'_, T> {
/* FP:cursor.rs-0033 */     fn deref_mut(&mut self) -> &mut T {
/* FP:cursor.rs-0034 */         match self {
/* FP:cursor.rs-0035 */             CowMut::BorrowedMut(borrowed) => borrowed,
/* FP:cursor.rs-0036 */             CowMut::Owned(owned) => owned,
/* FP:cursor.rs-0037 */         }
/* FP:cursor.rs-0038 */     }
/* FP:cursor.rs-0039 */ }
/* FP:cursor.rs-0040 */ 
/* FP:cursor.rs-0041 */ /// Allows random access inspection of the results of a dataflow analysis. Use this when you want
/* FP:cursor.rs-0042 */ /// to inspect domain values only in certain locations; use `ResultsVisitor` if you want to inspect
/* FP:cursor.rs-0043 */ /// domain values in many or all locations.
/* FP:cursor.rs-0044 */ ///
/* FP:cursor.rs-0045 */ /// Because `Results` only has domain values for the entry of each basic block, these inspections
/* FP:cursor.rs-0046 */ /// involve some amount of domain value recomputations. This cursor only has linear performance
/* FP:cursor.rs-0047 */ /// within a basic block when its statements are visited in the same order as the `DIRECTION` of
/* FP:cursor.rs-0048 */ /// the analysis. In the worst case—when statements are visited in *reverse* order—performance will
/* FP:cursor.rs-0049 */ /// be quadratic in the number of statements in the block. The order in which basic blocks are
/* FP:cursor.rs-0050 */ /// inspected has no impact on performance.
/* FP:cursor.rs-0051 */ pub struct ResultsCursor<'mir, 'tcx, A>
/* FP:cursor.rs-0052 */ where
/* FP:cursor.rs-0053 */     A: Analysis<'tcx>,
/* FP:cursor.rs-0054 */ {
/* FP:cursor.rs-0055 */     body: &'mir mir::Body<'tcx>,
/* FP:cursor.rs-0056 */     analysis: CowMut<'mir, A>,
/* FP:cursor.rs-0057 */     results: Cow<'mir, Results<A::Domain>>,
/* FP:cursor.rs-0058 */     state: A::Domain,
/* FP:cursor.rs-0059 */ 
/* FP:cursor.rs-0060 */     pos: CursorPosition,
/* FP:cursor.rs-0061 */ 
/* FP:cursor.rs-0062 */     /// Indicates that `state` has been modified with a custom effect.
/* FP:cursor.rs-0063 */     ///
/* FP:cursor.rs-0064 */     /// When this flag is set, we need to reset to an entry set before doing a seek.
/* FP:cursor.rs-0065 */     state_needs_reset: bool,
/* FP:cursor.rs-0066 */ 
/* FP:cursor.rs-0067 */     #[cfg(debug_assertions)]
/* FP:cursor.rs-0068 */     reachable_blocks: DenseBitSet<BasicBlock>,
/* FP:cursor.rs-0069 */ }
/* FP:cursor.rs-0070 */ 
/* FP:cursor.rs-0071 */ impl<'mir, 'tcx, A> ResultsCursor<'mir, 'tcx, A>
/* FP:cursor.rs-0072 */ where
/* FP:cursor.rs-0073 */     A: Analysis<'tcx>,
/* FP:cursor.rs-0074 */ {
/* FP:cursor.rs-0075 */     /// Returns the dataflow state at the current location.
/* FP:cursor.rs-0076 */     pub fn get(&self) -> &A::Domain {
/* FP:cursor.rs-0077 */         &self.state
/* FP:cursor.rs-0078 */     }
/* FP:cursor.rs-0079 */ 
/* FP:cursor.rs-0080 */     /// Returns the body this analysis was run on.
/* FP:cursor.rs-0081 */     pub fn body(&self) -> &'mir mir::Body<'tcx> {
/* FP:cursor.rs-0082 */         self.body
/* FP:cursor.rs-0083 */     }
/* FP:cursor.rs-0084 */ 
/* FP:cursor.rs-0085 */     fn new(
/* FP:cursor.rs-0086 */         body: &'mir mir::Body<'tcx>,
/* FP:cursor.rs-0087 */         analysis: CowMut<'mir, A>,
/* FP:cursor.rs-0088 */         results: Cow<'mir, Results<A::Domain>>,
/* FP:cursor.rs-0089 */     ) -> Self {
/* FP:cursor.rs-0090 */         let bottom_value = analysis.bottom_value(body);
/* FP:cursor.rs-0091 */         ResultsCursor {
/* FP:cursor.rs-0092 */             body,
/* FP:cursor.rs-0093 */             analysis,
/* FP:cursor.rs-0094 */             results,
/* FP:cursor.rs-0095 */ 
/* FP:cursor.rs-0096 */             // Initialize to the `bottom_value` and set `state_needs_reset` to tell the cursor that
/* FP:cursor.rs-0097 */             // it needs to reset to block entry before the first seek. The cursor position is
/* FP:cursor.rs-0098 */             // immaterial.
/* FP:cursor.rs-0099 */             state_needs_reset: true,
/* FP:cursor.rs-0100 */             state: bottom_value,
/* FP:cursor.rs-0101 */             pos: CursorPosition::block_entry(mir::START_BLOCK),
/* FP:cursor.rs-0102 */ 
/* FP:cursor.rs-0103 */             #[cfg(debug_assertions)]
/* FP:cursor.rs-0104 */             reachable_blocks: mir::traversal::reachable_as_bitset(body),
/* FP:cursor.rs-0105 */         }
/* FP:cursor.rs-0106 */     }
/* FP:cursor.rs-0107 */ 
/* FP:cursor.rs-0108 */     /// Returns a new cursor that takes ownership of and inspects analysis results.
/* FP:cursor.rs-0109 */     pub fn new_owning(
/* FP:cursor.rs-0110 */         body: &'mir mir::Body<'tcx>,
/* FP:cursor.rs-0111 */         analysis: A,
/* FP:cursor.rs-0112 */         results: Results<A::Domain>,
/* FP:cursor.rs-0113 */     ) -> Self {
/* FP:cursor.rs-0114 */         Self::new(body, CowMut::Owned(analysis), Cow::Owned(results))
/* FP:cursor.rs-0115 */     }
/* FP:cursor.rs-0116 */ 
/* FP:cursor.rs-0117 */     /// Returns a new cursor that borrows and inspects analysis results.
/* FP:cursor.rs-0118 */     pub fn new_borrowing(
/* FP:cursor.rs-0119 */         body: &'mir mir::Body<'tcx>,
/* FP:cursor.rs-0120 */         analysis: &'mir mut A,
/* FP:cursor.rs-0121 */         results: &'mir Results<A::Domain>,
/* FP:cursor.rs-0122 */     ) -> Self {
/* FP:cursor.rs-0123 */         Self::new(body, CowMut::BorrowedMut(analysis), Cow::Borrowed(results))
/* FP:cursor.rs-0124 */     }
/* FP:cursor.rs-0125 */ 
/* FP:cursor.rs-0126 */     /// Allows inspection of unreachable basic blocks even with `debug_assertions` enabled.
/* FP:cursor.rs-0127 */     #[cfg(test)]
/* FP:cursor.rs-0128 */     pub(crate) fn allow_unreachable(&mut self) {
/* FP:cursor.rs-0129 */         #[cfg(debug_assertions)]
/* FP:cursor.rs-0130 */         self.reachable_blocks.insert_all()
/* FP:cursor.rs-0131 */     }
/* FP:cursor.rs-0132 */ 
/* FP:cursor.rs-0133 */     /// Returns the `Analysis` used to generate the underlying `Results`.
/* FP:cursor.rs-0134 */     pub fn analysis(&self) -> &A {
/* FP:cursor.rs-0135 */         &self.analysis
/* FP:cursor.rs-0136 */     }
/* FP:cursor.rs-0137 */ 
/* FP:cursor.rs-0138 */     /// Resets the cursor to hold the entry set for the given basic block.
/* FP:cursor.rs-0139 */     ///
/* FP:cursor.rs-0140 */     /// For forward dataflow analyses, this is the dataflow state prior to the first statement.
/* FP:cursor.rs-0141 */     ///
/* FP:cursor.rs-0142 */     /// For backward dataflow analyses, this is the dataflow state after the terminator.
/* FP:cursor.rs-0143 */     pub(super) fn seek_to_block_entry(&mut self, block: BasicBlock) {
/* FP:cursor.rs-0144 */         #[cfg(debug_assertions)]
/* FP:cursor.rs-0145 */         assert!(self.reachable_blocks.contains(block));
/* FP:cursor.rs-0146 */ 
/* FP:cursor.rs-0147 */         self.state.clone_from(&self.results[block]);
/* FP:cursor.rs-0148 */         self.pos = CursorPosition::block_entry(block);
/* FP:cursor.rs-0149 */         self.state_needs_reset = false;
/* FP:cursor.rs-0150 */     }
/* FP:cursor.rs-0151 */ 
/* FP:cursor.rs-0152 */     /// Resets the cursor to hold the state prior to the first statement in a basic block.
/* FP:cursor.rs-0153 */     ///
/* FP:cursor.rs-0154 */     /// For forward analyses, this is the entry set for the given block.
/* FP:cursor.rs-0155 */     ///
/* FP:cursor.rs-0156 */     /// For backward analyses, this is the state that will be propagated to its
/* FP:cursor.rs-0157 */     /// predecessors (ignoring edge-specific effects).
/* FP:cursor.rs-0158 */     pub fn seek_to_block_start(&mut self, block: BasicBlock) {
/* FP:cursor.rs-0159 */         if A::Direction::IS_FORWARD {
/* FP:cursor.rs-0160 */             self.seek_to_block_entry(block)
/* FP:cursor.rs-0161 */         } else {
/* FP:cursor.rs-0162 */             self.seek_after(Location { block, statement_index: 0 }, Effect::Primary)
/* FP:cursor.rs-0163 */         }
/* FP:cursor.rs-0164 */     }
/* FP:cursor.rs-0165 */ 
/* FP:cursor.rs-0166 */     /// Resets the cursor to hold the state after the terminator in a basic block.
/* FP:cursor.rs-0167 */     ///
/* FP:cursor.rs-0168 */     /// For backward analyses, this is the entry set for the given block.
/* FP:cursor.rs-0169 */     ///
/* FP:cursor.rs-0170 */     /// For forward analyses, this is the state that will be propagated to its
/* FP:cursor.rs-0171 */     /// successors (ignoring edge-specific effects).
/* FP:cursor.rs-0172 */     pub fn seek_to_block_end(&mut self, block: BasicBlock) {
/* FP:cursor.rs-0173 */         if A::Direction::IS_BACKWARD {
/* FP:cursor.rs-0174 */             self.seek_to_block_entry(block)
/* FP:cursor.rs-0175 */         } else {
/* FP:cursor.rs-0176 */             self.seek_after(self.body.terminator_loc(block), Effect::Primary)
/* FP:cursor.rs-0177 */         }
/* FP:cursor.rs-0178 */     }
/* FP:cursor.rs-0179 */ 
/* FP:cursor.rs-0180 */     /// Advances the cursor to hold the dataflow state at `target` before its "primary" effect is
/* FP:cursor.rs-0181 */     /// applied.
/* FP:cursor.rs-0182 */     ///
/* FP:cursor.rs-0183 */     /// The "early" effect at the target location *will be* applied.
/* FP:cursor.rs-0184 */     pub fn seek_before_primary_effect(&mut self, target: Location) {
/* FP:cursor.rs-0185 */         self.seek_after(target, Effect::Early)
/* FP:cursor.rs-0186 */     }
/* FP:cursor.rs-0187 */ 
/* FP:cursor.rs-0188 */     /// Advances the cursor to hold the dataflow state at `target` after its "primary" effect is
/* FP:cursor.rs-0189 */     /// applied.
/* FP:cursor.rs-0190 */     ///
/* FP:cursor.rs-0191 */     /// The "early" effect at the target location will be applied as well.
/* FP:cursor.rs-0192 */     pub fn seek_after_primary_effect(&mut self, target: Location) {
/* FP:cursor.rs-0193 */         self.seek_after(target, Effect::Primary)
/* FP:cursor.rs-0194 */     }
/* FP:cursor.rs-0195 */ 
/* FP:cursor.rs-0196 */     fn seek_after(&mut self, target: Location, effect: Effect) {
/* FP:cursor.rs-0197 */         assert!(target <= self.body.terminator_loc(target.block));
/* FP:cursor.rs-0198 */ 
/* FP:cursor.rs-0199 */         // Reset to the entry of the target block if any of the following are true:
/* FP:cursor.rs-0200 */         //   - A custom effect has been applied to the cursor state.
/* FP:cursor.rs-0201 */         //   - We are in a different block than the target.
/* FP:cursor.rs-0202 */         //   - We are in the same block but have advanced past the target effect.
/* FP:cursor.rs-0203 */         if self.state_needs_reset || self.pos.block != target.block {
/* FP:cursor.rs-0204 */             self.seek_to_block_entry(target.block);
/* FP:cursor.rs-0205 */         } else if let Some(curr_effect) = self.pos.curr_effect_index {
/* FP:cursor.rs-0206 */             let mut ord = curr_effect.statement_index.cmp(&target.statement_index);
/* FP:cursor.rs-0207 */             if A::Direction::IS_BACKWARD {
/* FP:cursor.rs-0208 */                 ord = ord.reverse()
/* FP:cursor.rs-0209 */             }
/* FP:cursor.rs-0210 */ 
/* FP:cursor.rs-0211 */             match ord.then_with(|| curr_effect.effect.cmp(&effect)) {
/* FP:cursor.rs-0212 */                 Ordering::Equal => return,
/* FP:cursor.rs-0213 */                 Ordering::Greater => self.seek_to_block_entry(target.block),
/* FP:cursor.rs-0214 */                 Ordering::Less => {}
/* FP:cursor.rs-0215 */             }
/* FP:cursor.rs-0216 */         }
/* FP:cursor.rs-0217 */ 
/* FP:cursor.rs-0218 */         // At this point, the cursor is in the same block as the target location at an earlier
/* FP:cursor.rs-0219 */         // statement.
/* FP:cursor.rs-0220 */         debug_assert_eq!(target.block, self.pos.block);
/* FP:cursor.rs-0221 */ 
/* FP:cursor.rs-0222 */         let block_data = &self.body[target.block];
/* FP:cursor.rs-0223 */         #[rustfmt::skip]
/* FP:cursor.rs-0224 */         let next_effect = if A::Direction::IS_FORWARD {
/* FP:cursor.rs-0225 */             self.pos.curr_effect_index.map_or_else(
/* FP:cursor.rs-0226 */                 || Effect::Early.at_index(0),
/* FP:cursor.rs-0227 */                 EffectIndex::next_in_forward_order,
/* FP:cursor.rs-0228 */             )
/* FP:cursor.rs-0229 */         } else {
/* FP:cursor.rs-0230 */             self.pos.curr_effect_index.map_or_else(
/* FP:cursor.rs-0231 */                 || Effect::Early.at_index(block_data.statements.len()),
/* FP:cursor.rs-0232 */                 EffectIndex::next_in_backward_order,
/* FP:cursor.rs-0233 */             )
/* FP:cursor.rs-0234 */         };
/* FP:cursor.rs-0235 */ 
/* FP:cursor.rs-0236 */         let target_effect_index = effect.at_index(target.statement_index);
/* FP:cursor.rs-0237 */ 
/* FP:cursor.rs-0238 */         A::Direction::apply_effects_in_range(
/* FP:cursor.rs-0239 */             &mut *self.analysis,
/* FP:cursor.rs-0240 */             &mut self.state,
/* FP:cursor.rs-0241 */             target.block,
/* FP:cursor.rs-0242 */             block_data,
/* FP:cursor.rs-0243 */             next_effect..=target_effect_index,
/* FP:cursor.rs-0244 */         );
/* FP:cursor.rs-0245 */ 
/* FP:cursor.rs-0246 */         self.pos =
/* FP:cursor.rs-0247 */             CursorPosition { block: target.block, curr_effect_index: Some(target_effect_index) };
/* FP:cursor.rs-0248 */     }
/* FP:cursor.rs-0249 */ 
/* FP:cursor.rs-0250 */     /// Applies `f` to the cursor's internal state.
/* FP:cursor.rs-0251 */     ///
/* FP:cursor.rs-0252 */     /// This can be used, e.g., to apply the call return effect directly to the cursor without
/* FP:cursor.rs-0253 */     /// creating an extra copy of the dataflow state.
/* FP:cursor.rs-0254 */     pub fn apply_custom_effect(&mut self, f: impl FnOnce(&mut A, &mut A::Domain)) {
/* FP:cursor.rs-0255 */         f(&mut self.analysis, &mut self.state);
/* FP:cursor.rs-0256 */         self.state_needs_reset = true;
/* FP:cursor.rs-0257 */     }
/* FP:cursor.rs-0258 */ }
/* FP:cursor.rs-0259 */ 
/* FP:cursor.rs-0260 */ #[derive(Clone, Copy, Debug)]
/* FP:cursor.rs-0261 */ struct CursorPosition {
/* FP:cursor.rs-0262 */     block: BasicBlock,
/* FP:cursor.rs-0263 */     curr_effect_index: Option<EffectIndex>,
/* FP:cursor.rs-0264 */ }
/* FP:cursor.rs-0265 */ 
/* FP:cursor.rs-0266 */ impl CursorPosition {
/* FP:cursor.rs-0267 */     fn block_entry(block: BasicBlock) -> CursorPosition {
/* FP:cursor.rs-0268 */         CursorPosition { block, curr_effect_index: None }
/* FP:cursor.rs-0269 */     }
/* FP:cursor.rs-0270 */ }