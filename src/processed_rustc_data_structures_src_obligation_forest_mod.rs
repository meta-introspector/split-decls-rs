/* FP:mod.rs-0001 */ // The `ObligationForest` is a utility data structure used in trait
/* FP:mod.rs-0002 */ // matching to track the set of outstanding obligations (those not yet
/* FP:mod.rs-0003 */ // resolved to success or error). It also tracks the "backtrace" of each
/* FP:mod.rs-0004 */ // pending obligation (why we are trying to figure this out in the first
/* FP:mod.rs-0005 */ // place).
/* FP:mod.rs-0006 */ //
/* FP:mod.rs-0007 */ // ### External view
/* FP:mod.rs-0008 */ //
/* FP:mod.rs-0009 */ // `ObligationForest` supports two main public operations (there are a
/* FP:mod.rs-0010 */ // few others not discussed here):
/* FP:mod.rs-0011 */ //
/* FP:mod.rs-0012 */ // 1. Add a new root obligations (`register_obligation`).
/* FP:mod.rs-0013 */ // 2. Process the pending obligations (`process_obligations`).
/* FP:mod.rs-0014 */ //
/* FP:mod.rs-0015 */ // When a new obligation `N` is added, it becomes the root of an
/* FP:mod.rs-0016 */ // obligation tree. This tree can also carry some per-tree state `T`,
/* FP:mod.rs-0017 */ // which is given at the same time. This tree is a singleton to start, so
/* FP:mod.rs-0018 */ // `N` is both the root and the only leaf. Each time the
/* FP:mod.rs-0019 */ // `process_obligations` method is called, it will invoke its callback
/* FP:mod.rs-0020 */ // with every pending obligation (so that will include `N`, the first
/* FP:mod.rs-0021 */ // time). The callback also receives a (mutable) reference to the
/* FP:mod.rs-0022 */ // per-tree state `T`. The callback should process the obligation `O`
/* FP:mod.rs-0023 */ // that it is given and return a `ProcessResult`:
/* FP:mod.rs-0024 */ //
/* FP:mod.rs-0025 */ // - `Unchanged` -> ambiguous result. Obligation was neither a success
/* FP:mod.rs-0026 */ //   nor a failure. It is assumed that further attempts to process the
/* FP:mod.rs-0027 */ //   obligation will yield the same result unless something in the
/* FP:mod.rs-0028 */ //   surrounding environment changes.
/* FP:mod.rs-0029 */ // - `Changed(C)` - the obligation was *shallowly successful*. The
/* FP:mod.rs-0030 */ //   vector `C` is a list of subobligations. The meaning of this is that
/* FP:mod.rs-0031 */ //   `O` was successful on the assumption that all the obligations in `C`
/* FP:mod.rs-0032 */ //   are also successful. Therefore, `O` is only considered a "true"
/* FP:mod.rs-0033 */ //   success if `C` is empty. Otherwise, `O` is put into a suspended
/* FP:mod.rs-0034 */ //   state and the obligations in `C` become the new pending
/* FP:mod.rs-0035 */ //   obligations. They will be processed the next time you call
/* FP:mod.rs-0036 */ //   `process_obligations`.
/* FP:mod.rs-0037 */ // - `Error(E)` -> obligation failed with error `E`. We will collect this
/* FP:mod.rs-0038 */ //   error and return it from `process_obligations`, along with the
/* FP:mod.rs-0039 */ //   "backtrace" of obligations (that is, the list of obligations up to
/* FP:mod.rs-0040 */ //   and including the root of the failed obligation). No further
/* FP:mod.rs-0041 */ //   obligations from that same tree will be processed, since the tree is
/* FP:mod.rs-0042 */ //   now considered to be in error.
/* FP:mod.rs-0043 */ //
/* FP:mod.rs-0044 */ // When the call to `process_obligations` completes, you get back an `Outcome`,
/* FP:mod.rs-0045 */ // which includes two bits of information:
/* FP:mod.rs-0046 */ //
/* FP:mod.rs-0047 */ // - `completed`: a list of obligations where processing was fully
/* FP:mod.rs-0048 */ //   completed without error (meaning that all transitive subobligations
/* FP:mod.rs-0049 */ //   have also been completed). So, for example, if the callback from
/* FP:mod.rs-0050 */ //   `process_obligations` returns `Changed(C)` for some obligation `O`,
/* FP:mod.rs-0051 */ //   then `O` will be considered completed right away if `C` is the
/* FP:mod.rs-0052 */ //   empty vector. Otherwise it will only be considered completed once
/* FP:mod.rs-0053 */ //   all the obligations in `C` have been found completed.
/* FP:mod.rs-0054 */ // - `errors`: a list of errors that occurred and associated backtraces
/* FP:mod.rs-0055 */ //   at the time of error, which can be used to give context to the user.
/* FP:mod.rs-0056 */ //
/* FP:mod.rs-0057 */ // Upon completion, none of the existing obligations were *shallowly
/* FP:mod.rs-0058 */ // successful* (that is, no callback returned `Changed(_)`). This implies that
/* FP:mod.rs-0059 */ // all obligations were either errors or returned an ambiguous result.
/* FP:mod.rs-0060 */ //
/* FP:mod.rs-0061 */ // ### Implementation details
/* FP:mod.rs-0062 */ //
/* FP:mod.rs-0063 */ // For the most part, comments specific to the implementation are in the
/* FP:mod.rs-0064 */ // code. This file only contains a very high-level overview. Basically,
/* FP:mod.rs-0065 */ // the forest is stored in a vector. Each element of the vector is a node
/* FP:mod.rs-0066 */ // in some tree. Each node in the vector has the index of its dependents,
/* FP:mod.rs-0067 */ // including the first dependent which is known as the parent. It also
/* FP:mod.rs-0068 */ // has a current state, described by `NodeState`. After each processing
/* FP:mod.rs-0069 */ // step, we compress the vector to remove completed and error nodes, which
/* FP:mod.rs-0070 */ // aren't needed anymore.
/* FP:mod.rs-0071 */ 
/* FP:mod.rs-0072 */ use std::cell::Cell;
/* FP:mod.rs-0073 */ use std::collections::hash_map::Entry;
/* FP:mod.rs-0074 */ use std::fmt::Debug;
/* FP:mod.rs-0075 */ use std::hash;
/* FP:mod.rs-0076 */ use std::marker::PhantomData;
/* FP:mod.rs-0077 */ 
/* FP:mod.rs-0078 */ use thin_vec::ThinVec;
/* FP:mod.rs-0079 */ use tracing::debug;
/* FP:mod.rs-0080 */ 
/* FP:mod.rs-0081 */ use crate::fx::{FxHashMap, FxHashSet};
/* FP:mod.rs-0082 */ 
/* FP:mod.rs-0084 */ 
/* FP:mod.rs-0085 */ #[cfg(test)]
/* FP:mod.rs-0087 */ 
/* FP:mod.rs-0088 */ pub trait ForestObligation: Clone + Debug {
/* FP:mod.rs-0089 */     type CacheKey: Clone + hash::Hash + Eq + Debug;
/* FP:mod.rs-0090 */ 
/* FP:mod.rs-0091 */     /// Converts this `ForestObligation` suitable for use as a cache key.
/* FP:mod.rs-0092 */     /// If two distinct `ForestObligations`s return the same cache key,
/* FP:mod.rs-0093 */     /// then it must be sound to use the result of processing one obligation
/* FP:mod.rs-0094 */     /// (e.g. success for error) for the other obligation
/* FP:mod.rs-0095 */     fn as_cache_key(&self) -> Self::CacheKey;
/* FP:mod.rs-0096 */ }
/* FP:mod.rs-0097 */ 
/* FP:mod.rs-0098 */ pub trait ObligationProcessor {
/* FP:mod.rs-0099 */     type Obligation: ForestObligation;
/* FP:mod.rs-0100 */     type Error: Debug;
/* FP:mod.rs-0101 */     type OUT: OutcomeTrait<Obligation = Self::Obligation, Error = Error<Self::Obligation, Self::Error>>;
/* FP:mod.rs-0102 */ 
/* FP:mod.rs-0103 */     /// Implementations can provide a fast-path to obligation-processing
/* FP:mod.rs-0104 */     /// by counting the prefix of the passed iterator for which
/* FP:mod.rs-0105 */     /// `needs_process_obligation` would return false.
/* FP:mod.rs-0106 */     fn skippable_obligations<'a>(
/* FP:mod.rs-0107 */         &'a self,
/* FP:mod.rs-0108 */         _it: impl Iterator<Item = &'a Self::Obligation>,
/* FP:mod.rs-0109 */     ) -> usize {
/* FP:mod.rs-0110 */         0
/* FP:mod.rs-0111 */     }
/* FP:mod.rs-0112 */ 
/* FP:mod.rs-0113 */     fn needs_process_obligation(&self, _obligation: &Self::Obligation) -> bool;
/* FP:mod.rs-0114 */ 
/* FP:mod.rs-0115 */     fn process_obligation(
/* FP:mod.rs-0116 */         &mut self,
/* FP:mod.rs-0117 */         obligation: &mut Self::Obligation,
/* FP:mod.rs-0118 */     ) -> ProcessResult<Self::Obligation, Self::Error>;
/* FP:mod.rs-0119 */ 
/* FP:mod.rs-0120 */     /// As we do the cycle check, we invoke this callback when we
/* FP:mod.rs-0121 */     /// encounter an actual cycle. `cycle` is an iterator that starts
/* FP:mod.rs-0122 */     /// at the start of the cycle in the stack and walks **toward the
/* FP:mod.rs-0123 */     /// top**.
/* FP:mod.rs-0124 */     ///
/* FP:mod.rs-0125 */     /// In other words, if we had O1 which required O2 which required
/* FP:mod.rs-0126 */     /// O3 which required O1, we would give an iterator yielding O1,
/* FP:mod.rs-0127 */     /// O2, O3 (O1 is not yielded twice).
/* FP:mod.rs-0128 */     fn process_backedge<'c, I>(
/* FP:mod.rs-0129 */         &mut self,
/* FP:mod.rs-0130 */         cycle: I,
/* FP:mod.rs-0131 */         _marker: PhantomData<&'c Self::Obligation>,
/* FP:mod.rs-0132 */     ) -> Result<(), Self::Error>
/* FP:mod.rs-0133 */     where
/* FP:mod.rs-0134 */         I: Clone + Iterator<Item = &'c Self::Obligation>;
/* FP:mod.rs-0135 */ }
/* FP:mod.rs-0136 */ 
/* FP:mod.rs-0137 */ /// The result type used by `process_obligation`.
/* FP:mod.rs-0138 */ // `repr(C)` to inhibit the niche filling optimization. Otherwise, the `match` appearing
/* FP:mod.rs-0139 */ // in `process_obligations` is significantly slower, which can substantially affect
/* FP:mod.rs-0140 */ // benchmarks like `rustc-perf`'s inflate and keccak.
/* FP:mod.rs-0141 */ #[repr(C)]
/* FP:mod.rs-0142 */ #[derive(Debug)]
/* FP:mod.rs-0143 */ pub enum ProcessResult<O, E> {
/* FP:mod.rs-0144 */     Unchanged,
/* FP:mod.rs-0145 */     Changed(ThinVec<O>),
/* FP:mod.rs-0146 */     Error(E),
/* FP:mod.rs-0147 */ }
/* FP:mod.rs-0148 */ 
/* FP:mod.rs-0149 */ #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/* FP:mod.rs-0150 */ struct ObligationTreeId(usize);
/* FP:mod.rs-0151 */ 
/* FP:mod.rs-0152 */ pub struct ObligationForest<O: ForestObligation> {
/* FP:mod.rs-0153 */     /// The list of obligations. In between calls to [Self::process_obligations],
/* FP:mod.rs-0154 */     /// this list only contains nodes in the `Pending` or `Waiting` state.
/* FP:mod.rs-0155 */     ///
/* FP:mod.rs-0156 */     /// `usize` indices are used here and throughout this module, rather than
/* FP:mod.rs-0157 */     /// [`crate::rustc_index::newtype_index!`] indices, because this code is hot enough
/* FP:mod.rs-0158 */     /// that the `u32`-to-`usize` conversions that would be required are
/* FP:mod.rs-0159 */     /// significant, and space considerations are not important.
/* FP:mod.rs-0160 */     nodes: Vec<Node<O>>,
/* FP:mod.rs-0161 */ 
/* FP:mod.rs-0162 */     /// A cache of predicates that have been successfully completed.
/* FP:mod.rs-0163 */     done_cache: FxHashSet<O::CacheKey>,
/* FP:mod.rs-0164 */ 
/* FP:mod.rs-0165 */     /// A cache of the nodes in `nodes`, indexed by predicate. Unfortunately,
/* FP:mod.rs-0166 */     /// its contents are not guaranteed to match those of `nodes`. See the
/* FP:mod.rs-0167 */     /// comments in `Self::process_obligation` for details.
/* FP:mod.rs-0168 */     active_cache: FxHashMap<O::CacheKey, usize>,
/* FP:mod.rs-0169 */ 
/* FP:mod.rs-0170 */     /// A vector reused in [Self::compress()] and [Self::find_cycles_from_node()],
/* FP:mod.rs-0171 */     /// to avoid allocating new vectors.
/* FP:mod.rs-0172 */     reused_node_vec: Vec<usize>,
/* FP:mod.rs-0173 */ 
/* FP:mod.rs-0174 */     obligation_tree_id_generator: ObligationTreeIdGenerator,
/* FP:mod.rs-0175 */ 
/* FP:mod.rs-0176 */     /// Per tree error cache. This is used to deduplicate errors,
/* FP:mod.rs-0177 */     /// which is necessary to avoid trait resolution overflow in
/* FP:mod.rs-0178 */     /// some cases.
/* FP:mod.rs-0179 */     ///
/* FP:mod.rs-0180 */     /// See [this][details] for details.
/* FP:mod.rs-0181 */     ///
/* FP:mod.rs-0182 */     /// [details]: https://github.com/rust-lang/rust/pull/53255#issuecomment-421184780
/* FP:mod.rs-0183 */     error_cache: FxHashMap<ObligationTreeId, FxHashSet<O::CacheKey>>,
/* FP:mod.rs-0184 */ }
/* FP:mod.rs-0185 */ 
/* FP:mod.rs-0186 */ #[derive(Debug)]
/* FP:mod.rs-0187 */ struct Node<O> {
/* FP:mod.rs-0188 */     obligation: O,
/* FP:mod.rs-0189 */     state: Cell<NodeState>,
/* FP:mod.rs-0190 */ 
/* FP:mod.rs-0191 */     /// Obligations that depend on this obligation for their completion. They
/* FP:mod.rs-0192 */     /// must all be in a non-pending state.
/* FP:mod.rs-0193 */     dependents: Vec<usize>,
/* FP:mod.rs-0194 */ 
/* FP:mod.rs-0195 */     /// If true, `dependents[0]` points to a "parent" node, which requires
/* FP:mod.rs-0196 */     /// special treatment upon error but is otherwise treated the same.
/* FP:mod.rs-0197 */     /// (It would be more idiomatic to store the parent node in a separate
/* FP:mod.rs-0198 */     /// `Option<usize>` field, but that slows down the common case of
/* FP:mod.rs-0199 */     /// iterating over the parent and other descendants together.)
/* FP:mod.rs-0200 */     has_parent: bool,
/* FP:mod.rs-0201 */ 
/* FP:mod.rs-0202 */     /// Identifier of the obligation tree to which this node belongs.
/* FP:mod.rs-0203 */     obligation_tree_id: ObligationTreeId,
/* FP:mod.rs-0204 */ }
/* FP:mod.rs-0205 */ 
/* FP:mod.rs-0206 */ impl<O> Node<O> {
/* FP:mod.rs-0207 */     fn new(parent: Option<usize>, obligation: O, obligation_tree_id: ObligationTreeId) -> Node<O> {
/* FP:mod.rs-0208 */         Node {
/* FP:mod.rs-0209 */             obligation,
/* FP:mod.rs-0210 */             state: Cell::new(NodeState::Pending),
/* FP:mod.rs-0211 */             dependents: if let Some(parent_index) = parent { vec![parent_index] } else { vec![] },
/* FP:mod.rs-0212 */             has_parent: parent.is_some(),
/* FP:mod.rs-0213 */             obligation_tree_id,
/* FP:mod.rs-0214 */         }
/* FP:mod.rs-0215 */     }
/* FP:mod.rs-0216 */ }
/* FP:mod.rs-0217 */ 
/* FP:mod.rs-0218 */ /// The state of one node in some tree within the forest. This represents the
/* FP:mod.rs-0219 */ /// current state of processing for the obligation (of type `O`) associated
/* FP:mod.rs-0220 */ /// with this node.
/* FP:mod.rs-0221 */ ///
/* FP:mod.rs-0222 */ /// The non-`Error` state transitions are as follows.
/* FP:mod.rs-0223 */ /// ```text
/* FP:mod.rs-0224 */ /// (Pre-creation)
/* FP:mod.rs-0225 */ ///  |
/* FP:mod.rs-0226 */ ///  |     register_obligation_at() (called by process_obligations() and
/* FP:mod.rs-0227 */ ///  v                               from outside the crate)
/* FP:mod.rs-0228 */ /// Pending
/* FP:mod.rs-0229 */ ///  |
/* FP:mod.rs-0230 */ ///  |     process_obligations()
/* FP:mod.rs-0231 */ ///  v
/* FP:mod.rs-0232 */ /// Success
/* FP:mod.rs-0233 */ ///  |  ^
/* FP:mod.rs-0234 */ ///  |  |  mark_successes()
/* FP:mod.rs-0235 */ ///  |  v
/* FP:mod.rs-0236 */ ///  |  Waiting
/* FP:mod.rs-0237 */ ///  |
/* FP:mod.rs-0238 */ ///  |     process_cycles()
/* FP:mod.rs-0239 */ ///  v
/* FP:mod.rs-0240 */ /// Done
/* FP:mod.rs-0241 */ ///  |
/* FP:mod.rs-0242 */ ///  |     compress()
/* FP:mod.rs-0243 */ ///  v
/* FP:mod.rs-0244 */ /// (Removed)
/* FP:mod.rs-0245 */ /// ```
/* FP:mod.rs-0246 */ /// The `Error` state can be introduced in several places, via `error_at()`.
/* FP:mod.rs-0247 */ ///
/* FP:mod.rs-0248 */ /// Outside of `ObligationForest` methods, nodes should be either `Pending` or
/* FP:mod.rs-0249 */ /// `Waiting`.
/* FP:mod.rs-0250 */ #[derive(Debug, Copy, Clone, PartialEq, Eq)]
/* FP:mod.rs-0251 */ enum NodeState {
/* FP:mod.rs-0252 */     /// This obligation has not yet been selected successfully. Cannot have
/* FP:mod.rs-0253 */     /// subobligations.
/* FP:mod.rs-0254 */     Pending,
/* FP:mod.rs-0255 */ 
/* FP:mod.rs-0256 */     /// This obligation was selected successfully, but may or may not have
/* FP:mod.rs-0257 */     /// subobligations.
/* FP:mod.rs-0258 */     Success,
/* FP:mod.rs-0259 */ 
/* FP:mod.rs-0260 */     /// This obligation was selected successfully, but it has a pending
/* FP:mod.rs-0261 */     /// subobligation.
/* FP:mod.rs-0262 */     Waiting,
/* FP:mod.rs-0263 */ 
/* FP:mod.rs-0264 */     /// This obligation, along with its subobligations, are complete, and will
/* FP:mod.rs-0265 */     /// be removed in the next collection.
/* FP:mod.rs-0266 */     Done,
/* FP:mod.rs-0267 */ 
/* FP:mod.rs-0268 */     /// This obligation was resolved to an error. It will be removed by the
/* FP:mod.rs-0269 */     /// next compression step.
/* FP:mod.rs-0270 */     Error,
/* FP:mod.rs-0271 */ }
/* FP:mod.rs-0272 */ 
/* FP:mod.rs-0273 */ /// This trait allows us to have two different Outcome types:
/* FP:mod.rs-0274 */ ///  - the normal one that does as little as possible
/* FP:mod.rs-0275 */ ///  - one for tests that does some additional work and checking
/* FP:mod.rs-0276 */ pub trait OutcomeTrait {
/* FP:mod.rs-0277 */     type Error;
/* FP:mod.rs-0278 */     type Obligation;
/* FP:mod.rs-0279 */ 
/* FP:mod.rs-0280 */     fn new() -> Self;
/* FP:mod.rs-0281 */     fn record_completed(&mut self, outcome: &Self::Obligation);
/* FP:mod.rs-0282 */     fn record_error(&mut self, error: Self::Error);
/* FP:mod.rs-0283 */ }
/* FP:mod.rs-0284 */ 
/* FP:mod.rs-0285 */ #[derive(Debug)]
/* FP:mod.rs-0286 */ pub struct Outcome<O, E> {
/* FP:mod.rs-0287 */     /// Backtrace of obligations that were found to be in error.
/* FP:mod.rs-0288 */     pub errors: Vec<Error<O, E>>,
/* FP:mod.rs-0289 */ }
/* FP:mod.rs-0290 */ 
/* FP:mod.rs-0291 */ impl<O, E> OutcomeTrait for Outcome<O, E> {
/* FP:mod.rs-0292 */     type Error = Error<O, E>;
/* FP:mod.rs-0293 */     type Obligation = O;
/* FP:mod.rs-0294 */ 
/* FP:mod.rs-0295 */     fn new() -> Self {
/* FP:mod.rs-0296 */         Self { errors: vec![] }
/* FP:mod.rs-0297 */     }
/* FP:mod.rs-0298 */ 
/* FP:mod.rs-0299 */     fn record_completed(&mut self, _outcome: &Self::Obligation) {
/* FP:mod.rs-0300 */         // do nothing
/* FP:mod.rs-0301 */     }
/* FP:mod.rs-0302 */ 
/* FP:mod.rs-0303 */     fn record_error(&mut self, error: Self::Error) {
/* FP:mod.rs-0304 */         self.errors.push(error)
/* FP:mod.rs-0305 */     }
/* FP:mod.rs-0306 */ }
/* FP:mod.rs-0307 */ 
/* FP:mod.rs-0308 */ #[derive(Debug, PartialEq, Eq)]
/* FP:mod.rs-0309 */ pub struct Error<O, E> {
/* FP:mod.rs-0310 */     pub error: E,
/* FP:mod.rs-0311 */     pub backtrace: Vec<O>,
/* FP:mod.rs-0312 */ }
/* FP:mod.rs-0313 */ 
/* FP:mod.rs-0314 */ mod helper {
/* FP:mod.rs-0315 */     use super::*;
/* FP:mod.rs-0316 */     pub(super) type ObligationTreeIdGenerator = impl Iterator<Item = ObligationTreeId>;
/* FP:mod.rs-0317 */     impl<O: ForestObligation> ObligationForest<O> {
/* FP:mod.rs-0318 */         #[define_opaque(ObligationTreeIdGenerator)]
/* FP:mod.rs-0319 */         pub fn new() -> ObligationForest<O> {
/* FP:mod.rs-0320 */             ObligationForest {
/* FP:mod.rs-0321 */                 nodes: vec![],
/* FP:mod.rs-0322 */                 done_cache: Default::default(),
/* FP:mod.rs-0323 */                 active_cache: Default::default(),
/* FP:mod.rs-0324 */                 reused_node_vec: vec![],
/* FP:mod.rs-0325 */                 obligation_tree_id_generator: (0..).map(ObligationTreeId),
/* FP:mod.rs-0326 */                 error_cache: Default::default(),
/* FP:mod.rs-0327 */             }
/* FP:mod.rs-0328 */         }
/* FP:mod.rs-0329 */     }
/* FP:mod.rs-0330 */ }
/* FP:mod.rs-0331 */ use helper::*;
/* FP:mod.rs-0332 */ 
/* FP:mod.rs-0333 */ impl<O: ForestObligation> ObligationForest<O> {
/* FP:mod.rs-0334 */     /// Returns the total number of nodes in the forest that have not
/* FP:mod.rs-0335 */     /// yet been fully resolved.
/* FP:mod.rs-0336 */     pub fn len(&self) -> usize {
/* FP:mod.rs-0337 */         self.nodes.len()
/* FP:mod.rs-0338 */     }
/* FP:mod.rs-0339 */ 
/* FP:mod.rs-0340 */     /// Registers an obligation.
/* FP:mod.rs-0341 */     pub fn register_obligation(&mut self, obligation: O) {
/* FP:mod.rs-0342 */         // Ignore errors here - there is no guarantee of success.
/* FP:mod.rs-0343 */         let _ = self.register_obligation_at(obligation, None);
/* FP:mod.rs-0344 */     }
/* FP:mod.rs-0345 */ 
/* FP:mod.rs-0346 */     // Returns Err(()) if we already know this obligation failed.
/* FP:mod.rs-0347 */     fn register_obligation_at(&mut self, obligation: O, parent: Option<usize>) -> Result<(), ()> {
/* FP:mod.rs-0348 */         let cache_key = obligation.as_cache_key();
/* FP:mod.rs-0349 */         if self.done_cache.contains(&cache_key) {
/* FP:mod.rs-0350 */             debug!("register_obligation_at: ignoring already done obligation: {:?}", obligation);
/* FP:mod.rs-0351 */             return Ok(());
/* FP:mod.rs-0352 */         }
/* FP:mod.rs-0353 */ 
/* FP:mod.rs-0354 */         match self.active_cache.entry(cache_key) {
/* FP:mod.rs-0355 */             Entry::Occupied(o) => {
/* FP:mod.rs-0356 */                 let node = &mut self.nodes[*o.get()];
/* FP:mod.rs-0357 */                 if let Some(parent_index) = parent {
/* FP:mod.rs-0358 */                     // If the node is already in `active_cache`, it has already
/* FP:mod.rs-0359 */                     // had its chance to be marked with a parent. So if it's
/* FP:mod.rs-0360 */                     // not already present, just dump `parent` into the
/* FP:mod.rs-0361 */                     // dependents as a non-parent.
/* FP:mod.rs-0362 */                     if !node.dependents.contains(&parent_index) {
/* FP:mod.rs-0363 */                         node.dependents.push(parent_index);
/* FP:mod.rs-0364 */                     }
/* FP:mod.rs-0365 */                 }
/* FP:mod.rs-0366 */                 if let NodeState::Error = node.state.get() { Err(()) } else { Ok(()) }
/* FP:mod.rs-0367 */             }
/* FP:mod.rs-0368 */             Entry::Vacant(v) => {
/* FP:mod.rs-0369 */                 let obligation_tree_id = match parent {
/* FP:mod.rs-0370 */                     Some(parent_index) => self.nodes[parent_index].obligation_tree_id,
/* FP:mod.rs-0371 */                     None => self.obligation_tree_id_generator.next().unwrap(),
/* FP:mod.rs-0372 */                 };
/* FP:mod.rs-0373 */ 
/* FP:mod.rs-0374 */                 let already_failed = parent.is_some()
/* FP:mod.rs-0375 */                     && self
/* FP:mod.rs-0376 */                         .error_cache
/* FP:mod.rs-0377 */                         .get(&obligation_tree_id)
/* FP:mod.rs-0378 */                         .is_some_and(|errors| errors.contains(v.key()));
/* FP:mod.rs-0379 */ 
/* FP:mod.rs-0380 */                 if already_failed {
/* FP:mod.rs-0381 */                     Err(())
/* FP:mod.rs-0382 */                 } else {
/* FP:mod.rs-0383 */                     let new_index = self.nodes.len();
/* FP:mod.rs-0384 */                     v.insert(new_index);
/* FP:mod.rs-0385 */                     self.nodes.push(Node::new(parent, obligation, obligation_tree_id));
/* FP:mod.rs-0386 */                     Ok(())
/* FP:mod.rs-0387 */                 }
/* FP:mod.rs-0388 */             }
/* FP:mod.rs-0389 */         }
/* FP:mod.rs-0390 */     }
/* FP:mod.rs-0391 */ 
/* FP:mod.rs-0392 */     /// Converts all remaining obligations to the given error.
/* FP:mod.rs-0393 */     pub fn to_errors<E: Clone>(&mut self, error: E) -> Vec<Error<O, E>> {
/* FP:mod.rs-0394 */         let errors = self
/* FP:mod.rs-0395 */             .nodes
/* FP:mod.rs-0396 */             .iter()
/* FP:mod.rs-0397 */             .enumerate()
/* FP:mod.rs-0398 */             .filter(|(_index, node)| node.state.get() == NodeState::Pending)
/* FP:mod.rs-0399 */             .map(|(index, _node)| Error { error: error.clone(), backtrace: self.error_at(index) })
/* FP:mod.rs-0400 */             .collect();
/* FP:mod.rs-0401 */ 
/* FP:mod.rs-0402 */         self.compress(|_| assert!(false));
/* FP:mod.rs-0403 */         errors
/* FP:mod.rs-0404 */     }
/* FP:mod.rs-0405 */ 
/* FP:mod.rs-0406 */     /// Returns the set of obligations that are in a pending state.
/* FP:mod.rs-0407 */     pub fn map_pending_obligations<P, F, R>(&self, f: F) -> R
/* FP:mod.rs-0408 */     where
/* FP:mod.rs-0409 */         F: Fn(&O) -> P,
/* FP:mod.rs-0410 */         R: FromIterator<P>,
/* FP:mod.rs-0411 */     {
/* FP:mod.rs-0412 */         self.nodes
/* FP:mod.rs-0413 */             .iter()
/* FP:mod.rs-0414 */             .filter(|node| node.state.get() == NodeState::Pending)
/* FP:mod.rs-0415 */             .map(|node| f(&node.obligation))
/* FP:mod.rs-0416 */             .collect()
/* FP:mod.rs-0417 */     }
/* FP:mod.rs-0418 */ 
/* FP:mod.rs-0419 */     pub fn has_pending_obligations(&self) -> bool {
/* FP:mod.rs-0420 */         self.nodes.iter().any(|node| node.state.get() == NodeState::Pending)
/* FP:mod.rs-0421 */     }
/* FP:mod.rs-0422 */ 
/* FP:mod.rs-0423 */     fn insert_into_error_cache(&mut self, index: usize) {
/* FP:mod.rs-0424 */         let node = &self.nodes[index];
/* FP:mod.rs-0425 */         self.error_cache
/* FP:mod.rs-0426 */             .entry(node.obligation_tree_id)
/* FP:mod.rs-0427 */             .or_default()
/* FP:mod.rs-0428 */             .insert(node.obligation.as_cache_key());
/* FP:mod.rs-0429 */     }
/* FP:mod.rs-0430 */ 
/* FP:mod.rs-0431 */     /// Performs a fixpoint computation over the obligation list.
/* FP:mod.rs-0432 */     #[inline(never)]
/* FP:mod.rs-0433 */     pub fn process_obligations<P>(&mut self, processor: &mut P) -> P::OUT
/* FP:mod.rs-0434 */     where
/* FP:mod.rs-0435 */         P: ObligationProcessor<Obligation = O>,
/* FP:mod.rs-0436 */     {
/* FP:mod.rs-0437 */         let mut outcome = P::OUT::new();
/* FP:mod.rs-0438 */ 
/* FP:mod.rs-0439 */         // Fixpoint computation: we repeat until the inner loop stalls.
/* FP:mod.rs-0440 */         loop {
/* FP:mod.rs-0441 */             let mut has_changed = false;
/* FP:mod.rs-0442 */ 
/* FP:mod.rs-0443 */             // This is the super fast path for cheap-to-check conditions.
/* FP:mod.rs-0444 */             let mut index =
/* FP:mod.rs-0445 */                 processor.skippable_obligations(self.nodes.iter().map(|n| &n.obligation));
/* FP:mod.rs-0446 */ 
/* FP:mod.rs-0447 */             // Note that the loop body can append new nodes, and those new nodes
/* FP:mod.rs-0448 */             // will then be processed by subsequent iterations of the loop.
/* FP:mod.rs-0449 */             //
/* FP:mod.rs-0450 */             // We can't use an iterator for the loop because `self.nodes` is
/* FP:mod.rs-0451 */             // appended to and the borrow checker would complain. We also can't use
/* FP:mod.rs-0452 */             // `for index in 0..self.nodes.len() { ... }` because the range would
/* FP:mod.rs-0453 */             // be computed with the initial length, and we would miss the appended
/* FP:mod.rs-0454 */             // nodes. Therefore we use a `while` loop.
/* FP:mod.rs-0455 */             while let Some(node) = self.nodes.get_mut(index) {
/* FP:mod.rs-0456 */                 // This is the moderately fast path when the prefix skipping above didn't work out.
/* FP:mod.rs-0457 */                 if node.state.get() != NodeState::Pending
/* FP:mod.rs-0458 */                     || !processor.needs_process_obligation(&node.obligation)
/* FP:mod.rs-0459 */                 {
/* FP:mod.rs-0460 */                     index += 1;
/* FP:mod.rs-0461 */                     continue;
/* FP:mod.rs-0462 */                 }
/* FP:mod.rs-0463 */ 
/* FP:mod.rs-0464 */                 // `processor.process_obligation` can modify the predicate within
/* FP:mod.rs-0465 */                 // `node.obligation`, and that predicate is the key used for
/* FP:mod.rs-0466 */                 // `self.active_cache`. This means that `self.active_cache` can get
/* FP:mod.rs-0467 */                 // out of sync with `nodes`. It's not very common, but it does
/* FP:mod.rs-0468 */                 // happen, and code in `compress` has to allow for it.
/* FP:mod.rs-0469 */ 
/* FP:mod.rs-0470 */                 // This code is much less hot.
/* FP:mod.rs-0471 */                 match processor.process_obligation(&mut node.obligation) {
/* FP:mod.rs-0472 */                     ProcessResult::Unchanged => {
/* FP:mod.rs-0473 */                         // No change in state.
/* FP:mod.rs-0474 */                     }
/* FP:mod.rs-0475 */                     ProcessResult::Changed(children) => {
/* FP:mod.rs-0476 */                         // We are not (yet) stalled.
/* FP:mod.rs-0477 */                         has_changed = true;
/* FP:mod.rs-0478 */                         node.state.set(NodeState::Success);
/* FP:mod.rs-0479 */ 
/* FP:mod.rs-0480 */                         for child in children {
/* FP:mod.rs-0481 */                             let st = self.register_obligation_at(child, Some(index));
/* FP:mod.rs-0482 */                             if let Err(()) = st {
/* FP:mod.rs-0483 */                                 // Error already reported - propagate it
/* FP:mod.rs-0484 */                                 // to our node.
/* FP:mod.rs-0485 */                                 self.error_at(index);
/* FP:mod.rs-0486 */                             }
/* FP:mod.rs-0487 */                         }
/* FP:mod.rs-0488 */                     }
/* FP:mod.rs-0489 */                     ProcessResult::Error(err) => {
/* FP:mod.rs-0490 */                         has_changed = true;
/* FP:mod.rs-0491 */                         outcome.record_error(Error { error: err, backtrace: self.error_at(index) });
/* FP:mod.rs-0492 */                     }
/* FP:mod.rs-0493 */                 }
/* FP:mod.rs-0494 */                 index += 1;
/* FP:mod.rs-0495 */             }
/* FP:mod.rs-0496 */ 
/* FP:mod.rs-0497 */             // If unchanged, then we saw no successful obligations, which means
/* FP:mod.rs-0498 */             // there is no point in further iteration. This is based on the
/* FP:mod.rs-0499 */             // assumption that when trait matching returns `Error` or
/* FP:mod.rs-0500 */             // `Unchanged`, those results do not affect environmental inference
/* FP:mod.rs-0501 */             // state. (Note that this will occur if we invoke
/* FP:mod.rs-0502 */             // `process_obligations` with no pending obligations.)
/* FP:mod.rs-0503 */             if !has_changed {
/* FP:mod.rs-0504 */                 break;
/* FP:mod.rs-0505 */             }
/* FP:mod.rs-0506 */ 
/* FP:mod.rs-0507 */             self.mark_successes();
/* FP:mod.rs-0508 */             self.process_cycles(processor, &mut outcome);
/* FP:mod.rs-0509 */             self.compress(|obl| outcome.record_completed(obl));
/* FP:mod.rs-0510 */         }
/* FP:mod.rs-0511 */ 
/* FP:mod.rs-0512 */         outcome
/* FP:mod.rs-0513 */     }
/* FP:mod.rs-0514 */ 
/* FP:mod.rs-0515 */     /// Returns a vector of obligations for `p` and all of its
/* FP:mod.rs-0516 */     /// ancestors, putting them into the error state in the process.
/* FP:mod.rs-0517 */     fn error_at(&self, mut index: usize) -> Vec<O> {
/* FP:mod.rs-0518 */         let mut error_stack: Vec<usize> = vec![];
/* FP:mod.rs-0519 */         let mut trace = vec![];
/* FP:mod.rs-0520 */ 
/* FP:mod.rs-0521 */         loop {
/* FP:mod.rs-0522 */             let node = &self.nodes[index];
/* FP:mod.rs-0523 */             node.state.set(NodeState::Error);
/* FP:mod.rs-0524 */             trace.push(node.obligation.clone());
/* FP:mod.rs-0525 */             if node.has_parent {
/* FP:mod.rs-0526 */                 // The first dependent is the parent, which is treated
/* FP:mod.rs-0527 */                 // specially.
/* FP:mod.rs-0528 */                 error_stack.extend(node.dependents.iter().skip(1));
/* FP:mod.rs-0529 */                 index = node.dependents[0];
/* FP:mod.rs-0530 */             } else {
/* FP:mod.rs-0531 */                 // No parent; treat all dependents non-specially.
/* FP:mod.rs-0532 */                 error_stack.extend(node.dependents.iter());
/* FP:mod.rs-0533 */                 break;
/* FP:mod.rs-0534 */             }
/* FP:mod.rs-0535 */         }
/* FP:mod.rs-0536 */ 
/* FP:mod.rs-0537 */         while let Some(index) = error_stack.pop() {
/* FP:mod.rs-0538 */             let node = &self.nodes[index];
/* FP:mod.rs-0539 */             if node.state.get() != NodeState::Error {
/* FP:mod.rs-0540 */                 node.state.set(NodeState::Error);
/* FP:mod.rs-0541 */                 error_stack.extend(node.dependents.iter());
/* FP:mod.rs-0542 */             }
/* FP:mod.rs-0543 */         }
/* FP:mod.rs-0544 */ 
/* FP:mod.rs-0545 */         trace
/* FP:mod.rs-0546 */     }
/* FP:mod.rs-0547 */ 
/* FP:mod.rs-0548 */     /// Mark all `Waiting` nodes as `Success`, except those that depend on a
/* FP:mod.rs-0549 */     /// pending node.
/* FP:mod.rs-0550 */     fn mark_successes(&self) {
/* FP:mod.rs-0551 */         // Convert all `Waiting` nodes to `Success`.
/* FP:mod.rs-0552 */         for node in &self.nodes {
/* FP:mod.rs-0553 */             if node.state.get() == NodeState::Waiting {
/* FP:mod.rs-0554 */                 node.state.set(NodeState::Success);
/* FP:mod.rs-0555 */             }
/* FP:mod.rs-0556 */         }
/* FP:mod.rs-0557 */ 
/* FP:mod.rs-0558 */         // Convert `Success` nodes that depend on a pending node back to
/* FP:mod.rs-0559 */         // `Waiting`.
/* FP:mod.rs-0560 */         for node in &self.nodes {
/* FP:mod.rs-0561 */             if node.state.get() == NodeState::Pending {
/* FP:mod.rs-0562 */                 // This call site is hot.
/* FP:mod.rs-0563 */                 self.inlined_mark_dependents_as_waiting(node);
/* FP:mod.rs-0564 */             }
/* FP:mod.rs-0565 */         }
/* FP:mod.rs-0566 */     }
/* FP:mod.rs-0567 */ 
/* FP:mod.rs-0568 */     // This always-inlined function is for the hot call site.
/* FP:mod.rs-0569 */     #[inline(always)]
/* FP:mod.rs-0570 */     fn inlined_mark_dependents_as_waiting(&self, node: &Node<O>) {
/* FP:mod.rs-0571 */         for &index in node.dependents.iter() {
/* FP:mod.rs-0572 */             let node = &self.nodes[index];
/* FP:mod.rs-0573 */             let state = node.state.get();
/* FP:mod.rs-0574 */             if state == NodeState::Success {
/* FP:mod.rs-0575 */                 // This call site is cold.
/* FP:mod.rs-0576 */                 self.uninlined_mark_dependents_as_waiting(node);
/* FP:mod.rs-0577 */             } else {
/* FP:mod.rs-0578 */                 debug_assert!(state == NodeState::Waiting || state == NodeState::Error)
/* FP:mod.rs-0579 */             }
/* FP:mod.rs-0580 */         }
/* FP:mod.rs-0581 */     }
/* FP:mod.rs-0582 */ 
/* FP:mod.rs-0583 */     // This never-inlined function is for the cold call site.
/* FP:mod.rs-0584 */     #[inline(never)]
/* FP:mod.rs-0585 */     fn uninlined_mark_dependents_as_waiting(&self, node: &Node<O>) {
/* FP:mod.rs-0586 */         // Mark node Waiting in the cold uninlined code instead of the hot inlined
/* FP:mod.rs-0587 */         node.state.set(NodeState::Waiting);
/* FP:mod.rs-0588 */         self.inlined_mark_dependents_as_waiting(node)
/* FP:mod.rs-0589 */     }
/* FP:mod.rs-0590 */ 
/* FP:mod.rs-0591 */     /// Report cycles between all `Success` nodes, and convert all `Success`
/* FP:mod.rs-0592 */     /// nodes to `Done`. This must be called after `mark_successes`.
/* FP:mod.rs-0593 */     fn process_cycles<P>(&mut self, processor: &mut P, outcome: &mut P::OUT)
/* FP:mod.rs-0594 */     where
/* FP:mod.rs-0595 */         P: ObligationProcessor<Obligation = O>,
/* FP:mod.rs-0596 */     {
/* FP:mod.rs-0597 */         let mut stack = std::mem::take(&mut self.reused_node_vec);
/* FP:mod.rs-0598 */         for (index, node) in self.nodes.iter().enumerate() {
/* FP:mod.rs-0599 */             // For some benchmarks this state test is extremely hot. It's a win
/* FP:mod.rs-0600 */             // to handle the no-op cases immediately to avoid the cost of the
/* FP:mod.rs-0601 */             // function call.
/* FP:mod.rs-0602 */             if node.state.get() == NodeState::Success {
/* FP:mod.rs-0603 */                 self.find_cycles_from_node(&mut stack, processor, index, outcome);
/* FP:mod.rs-0604 */             }
/* FP:mod.rs-0605 */         }
/* FP:mod.rs-0606 */ 
/* FP:mod.rs-0607 */         debug_assert!(stack.is_empty());
/* FP:mod.rs-0608 */         self.reused_node_vec = stack;
/* FP:mod.rs-0609 */     }
/* FP:mod.rs-0610 */ 
/* FP:mod.rs-0611 */     fn find_cycles_from_node<P>(
/* FP:mod.rs-0612 */         &self,
/* FP:mod.rs-0613 */         stack: &mut Vec<usize>,
/* FP:mod.rs-0614 */         processor: &mut P,
/* FP:mod.rs-0615 */         index: usize,
/* FP:mod.rs-0616 */         outcome: &mut P::OUT,
/* FP:mod.rs-0617 */     ) where
/* FP:mod.rs-0618 */         P: ObligationProcessor<Obligation = O>,
/* FP:mod.rs-0619 */     {
/* FP:mod.rs-0620 */         let node = &self.nodes[index];
/* FP:mod.rs-0621 */         if node.state.get() == NodeState::Success {
/* FP:mod.rs-0622 */             match stack.iter().rposition(|&n| n == index) {
/* FP:mod.rs-0623 */                 None => {
/* FP:mod.rs-0624 */                     stack.push(index);
/* FP:mod.rs-0625 */                     for &dep_index in node.dependents.iter() {
/* FP:mod.rs-0626 */                         self.find_cycles_from_node(stack, processor, dep_index, outcome);
/* FP:mod.rs-0627 */                     }
/* FP:mod.rs-0628 */                     stack.pop();
/* FP:mod.rs-0629 */                     node.state.set(NodeState::Done);
/* FP:mod.rs-0630 */                 }
/* FP:mod.rs-0631 */                 Some(rpos) => {
/* FP:mod.rs-0632 */                     // Cycle detected.
/* FP:mod.rs-0633 */                     let result = processor.process_backedge(
/* FP:mod.rs-0634 */                         stack[rpos..].iter().map(|&i| &self.nodes[i].obligation),
/* FP:mod.rs-0635 */                         PhantomData,
/* FP:mod.rs-0636 */                     );
/* FP:mod.rs-0637 */                     if let Err(err) = result {
/* FP:mod.rs-0638 */                         outcome.record_error(Error { error: err, backtrace: self.error_at(index) });
/* FP:mod.rs-0639 */                     }
/* FP:mod.rs-0640 */                 }
/* FP:mod.rs-0641 */             }
/* FP:mod.rs-0642 */         }
/* FP:mod.rs-0643 */     }
/* FP:mod.rs-0644 */ 
/* FP:mod.rs-0645 */     /// Compresses the vector, removing all popped nodes. This adjusts the
/* FP:mod.rs-0646 */     /// indices and hence invalidates any outstanding indices. `process_cycles`
/* FP:mod.rs-0647 */     /// must be run beforehand to remove any cycles on `Success` nodes.
/* FP:mod.rs-0648 */     #[inline(never)]
/* FP:mod.rs-0649 */     fn compress(&mut self, mut outcome_cb: impl FnMut(&O)) {
/* FP:mod.rs-0650 */         let orig_nodes_len = self.nodes.len();
/* FP:mod.rs-0651 */         let mut node_rewrites: Vec<_> = std::mem::take(&mut self.reused_node_vec);
/* FP:mod.rs-0652 */         debug_assert!(node_rewrites.is_empty());
/* FP:mod.rs-0653 */         node_rewrites.extend(0..orig_nodes_len);
/* FP:mod.rs-0654 */         let mut dead_nodes = 0;
/* FP:mod.rs-0655 */ 
/* FP:mod.rs-0656 */         // Move removable nodes to the end, preserving the order of the
/* FP:mod.rs-0657 */         // remaining nodes.
/* FP:mod.rs-0658 */         //
/* FP:mod.rs-0659 */         // LOOP INVARIANT:
/* FP:mod.rs-0660 */         //     self.nodes[0..index - dead_nodes] are the first remaining nodes
/* FP:mod.rs-0661 */         //     self.nodes[index - dead_nodes..index] are all dead
/* FP:mod.rs-0662 */         //     self.nodes[index..] are unchanged
/* FP:mod.rs-0663 */         for index in 0..orig_nodes_len {
/* FP:mod.rs-0664 */             let node = &self.nodes[index];
/* FP:mod.rs-0665 */             match node.state.get() {
/* FP:mod.rs-0666 */                 NodeState::Pending | NodeState::Waiting => {
/* FP:mod.rs-0667 */                     if dead_nodes > 0 {
/* FP:mod.rs-0668 */                         self.nodes.swap(index, index - dead_nodes);
/* FP:mod.rs-0669 */                         node_rewrites[index] -= dead_nodes;
/* FP:mod.rs-0670 */                     }
/* FP:mod.rs-0671 */                 }
/* FP:mod.rs-0672 */                 NodeState::Done => {
/* FP:mod.rs-0673 */                     // The removal lookup might fail because the contents of
/* FP:mod.rs-0674 */                     // `self.active_cache` are not guaranteed to match those of
/* FP:mod.rs-0675 */                     // `self.nodes`. See the comment in `process_obligation`
/* FP:mod.rs-0676 */                     // for more details.
/* FP:mod.rs-0677 */                     let cache_key = node.obligation.as_cache_key();
/* FP:mod.rs-0678 */                     self.active_cache.remove(&cache_key);
/* FP:mod.rs-0679 */                     self.done_cache.insert(cache_key);
/* FP:mod.rs-0680 */ 
/* FP:mod.rs-0681 */                     // Extract the success stories.
/* FP:mod.rs-0682 */                     outcome_cb(&node.obligation);
/* FP:mod.rs-0683 */                     node_rewrites[index] = orig_nodes_len;
/* FP:mod.rs-0684 */                     dead_nodes += 1;
/* FP:mod.rs-0685 */                 }
/* FP:mod.rs-0686 */                 NodeState::Error => {
/* FP:mod.rs-0687 */                     // We *intentionally* remove the node from the cache at this point. Otherwise
/* FP:mod.rs-0688 */                     // tests must come up with a different type on every type error they
/* FP:mod.rs-0689 */                     // check against.
/* FP:mod.rs-0690 */                     self.active_cache.remove(&node.obligation.as_cache_key());
/* FP:mod.rs-0691 */                     self.insert_into_error_cache(index);
/* FP:mod.rs-0692 */                     node_rewrites[index] = orig_nodes_len;
/* FP:mod.rs-0693 */                     dead_nodes += 1;
/* FP:mod.rs-0694 */                 }
/* FP:mod.rs-0695 */                 NodeState::Success => unreachable!(),
/* FP:mod.rs-0696 */             }
/* FP:mod.rs-0697 */         }
/* FP:mod.rs-0698 */ 
/* FP:mod.rs-0699 */         if dead_nodes > 0 {
/* FP:mod.rs-0700 */             // Remove the dead nodes and rewrite indices.
/* FP:mod.rs-0701 */             self.nodes.truncate(orig_nodes_len - dead_nodes);
/* FP:mod.rs-0702 */             self.apply_rewrites(&node_rewrites);
/* FP:mod.rs-0703 */         }
/* FP:mod.rs-0704 */ 
/* FP:mod.rs-0705 */         node_rewrites.truncate(0);
/* FP:mod.rs-0706 */         self.reused_node_vec = node_rewrites;
/* FP:mod.rs-0707 */     }
/* FP:mod.rs-0708 */ 
/* FP:mod.rs-0709 */     #[inline(never)]
/* FP:mod.rs-0710 */     fn apply_rewrites(&mut self, node_rewrites: &[usize]) {
/* FP:mod.rs-0711 */         let orig_nodes_len = node_rewrites.len();
/* FP:mod.rs-0712 */ 
/* FP:mod.rs-0713 */         for node in &mut self.nodes {
/* FP:mod.rs-0714 */             let mut i = 0;
/* FP:mod.rs-0715 */             while let Some(dependent) = node.dependents.get_mut(i) {
/* FP:mod.rs-0716 */                 let new_index = node_rewrites[*dependent];
/* FP:mod.rs-0717 */                 if new_index >= orig_nodes_len {
/* FP:mod.rs-0718 */                     node.dependents.swap_remove(i);
/* FP:mod.rs-0719 */                     if i == 0 && node.has_parent {
/* FP:mod.rs-0720 */                         // We just removed the parent.
/* FP:mod.rs-0721 */                         node.has_parent = false;
/* FP:mod.rs-0722 */                     }
/* FP:mod.rs-0723 */                 } else {
/* FP:mod.rs-0724 */                     *dependent = new_index;
/* FP:mod.rs-0725 */                     i += 1;
/* FP:mod.rs-0726 */                 }
/* FP:mod.rs-0727 */             }
/* FP:mod.rs-0728 */         }
/* FP:mod.rs-0729 */ 
/* FP:mod.rs-0730 */         // This updating of `self.active_cache` is necessary because the
/* FP:mod.rs-0731 */         // removal of nodes within `compress` can fail. See above.
/* FP:mod.rs-0732 */         self.active_cache.retain(|_predicate, index| {
/* FP:mod.rs-0733 */             let new_index = node_rewrites[*index];
/* FP:mod.rs-0734 */             if new_index >= orig_nodes_len {
/* FP:mod.rs-0735 */                 false
/* FP:mod.rs-0736 */             } else {
/* FP:mod.rs-0737 */                 *index = new_index;
/* FP:mod.rs-0738 */                 true
/* FP:mod.rs-0739 */             }
/* FP:mod.rs-0740 */         });
/* FP:mod.rs-0741 */     }
/* FP:mod.rs-0742 */ }