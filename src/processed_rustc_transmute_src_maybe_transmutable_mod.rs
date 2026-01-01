/* FP:mod.rs-0001 */ use crate::rustc_data_structures::stack::ensure_sufficient_stack;
/* FP:mod.rs-0002 */ use tracing::{debug, instrument, trace};
/* FP:mod.rs-0003 */ 
/* FP:mod.rs-0005 */ #[cfg(test)]
/* FP:mod.rs-0007 */ 
/* FP:mod.rs-0008 */ use crate::layout::{self, Def, Dfa, Reference, Tree, dfa, union};
/* FP:mod.rs-0009 */ use crate::maybe_transmutable::query_context::QueryContext;
/* FP:mod.rs-0010 */ use crate::{Answer, Condition, Map, Reason};
/* FP:mod.rs-0011 */ 
/* FP:mod.rs-0012 */ pub(crate) struct MaybeTransmutableQuery<L, C>
/* FP:mod.rs-0013 */ where
/* FP:mod.rs-0014 */     C: QueryContext,
/* FP:mod.rs-0015 */ {
/* FP:mod.rs-0016 */     src: L,
/* FP:mod.rs-0017 */     dst: L,
/* FP:mod.rs-0018 */     assume: crate::Assume,
/* FP:mod.rs-0019 */     context: C,
/* FP:mod.rs-0020 */ }
/* FP:mod.rs-0021 */ 
/* FP:mod.rs-0022 */ impl<L, C> MaybeTransmutableQuery<L, C>
/* FP:mod.rs-0023 */ where
/* FP:mod.rs-0024 */     C: QueryContext,
/* FP:mod.rs-0025 */ {
/* FP:mod.rs-0026 */     pub(crate) fn new(src: L, dst: L, assume: crate::Assume, context: C) -> Self {
/* FP:mod.rs-0027 */         Self { src, dst, assume, context }
/* FP:mod.rs-0028 */     }
/* FP:mod.rs-0029 */ }
/* FP:mod.rs-0030 */ 
/* FP:mod.rs-0031 */ // FIXME: Nix this cfg, so we can write unit tests independently of rustc
/* FP:mod.rs-0032 */ #[cfg(feature = "rustc")]
/* FP:mod.rs-0033 */ mod rustc {
/* FP:mod.rs-0034 */     use crate::rustc_complete::ty::layout::LayoutCx;
/* FP:mod.rs-0035 */     use crate::rustc_complete::ty::{Ty, TyCtxt, TypingEnv};
/* FP:mod.rs-0036 */ 
/* FP:mod.rs-0037 */     use super::*;
/* FP:mod.rs-0038 */     use crate::layout::tree::rustc::Err;
/* FP:mod.rs-0039 */ 
/* FP:mod.rs-0040 */     impl<'tcx> MaybeTransmutableQuery<Ty<'tcx>, TyCtxt<'tcx>> {
/* FP:mod.rs-0041 */         /// This method begins by converting `src` and `dst` from `Ty`s to `Tree`s,
/* FP:mod.rs-0042 */         /// then computes an answer using those trees.
/* FP:mod.rs-0043 */         #[instrument(level = "debug", skip(self), fields(src = ?self.src, dst = ?self.dst))]
/* FP:mod.rs-0044 */         pub(crate) fn answer(
/* FP:mod.rs-0045 */             self,
/* FP:mod.rs-0046 */         ) -> Answer<<TyCtxt<'tcx> as QueryContext>::Region, <TyCtxt<'tcx> as QueryContext>::Type>
/* FP:mod.rs-0047 */         {
/* FP:mod.rs-0048 */             let Self { src, dst, assume, context } = self;
/* FP:mod.rs-0049 */ 
/* FP:mod.rs-0050 */             let layout_cx = LayoutCx::new(context, TypingEnv::fully_monomorphized());
/* FP:mod.rs-0051 */ 
/* FP:mod.rs-0052 */             // Convert `src` and `dst` from their rustc representations, to `Tree`-based
/* FP:mod.rs-0053 */             // representations.
/* FP:mod.rs-0054 */             let src = Tree::from_ty(src, layout_cx);
/* FP:mod.rs-0055 */             let dst = Tree::from_ty(dst, layout_cx);
/* FP:mod.rs-0056 */ 
/* FP:mod.rs-0057 */             match (src, dst) {
/* FP:mod.rs-0058 */                 (Err(Err::TypeError(_)), _) | (_, Err(Err::TypeError(_))) => {
/* FP:mod.rs-0059 */                     Answer::No(Reason::TypeError)
/* FP:mod.rs-0060 */                 }
/* FP:mod.rs-0061 */                 (Err(Err::UnknownLayout), _) => Answer::No(Reason::SrcLayoutUnknown),
/* FP:mod.rs-0062 */                 (_, Err(Err::UnknownLayout)) => Answer::No(Reason::DstLayoutUnknown),
/* FP:mod.rs-0063 */                 (Err(Err::NotYetSupported), _) => Answer::No(Reason::SrcIsNotYetSupported),
/* FP:mod.rs-0064 */                 (_, Err(Err::NotYetSupported)) => Answer::No(Reason::DstIsNotYetSupported),
/* FP:mod.rs-0065 */                 (Err(Err::SizeOverflow), _) => Answer::No(Reason::SrcSizeOverflow),
/* FP:mod.rs-0066 */                 (_, Err(Err::SizeOverflow)) => Answer::No(Reason::DstSizeOverflow),
/* FP:mod.rs-0067 */                 (Ok(src), Ok(dst)) => MaybeTransmutableQuery { src, dst, assume, context }.answer(),
/* FP:mod.rs-0068 */             }
/* FP:mod.rs-0069 */         }
/* FP:mod.rs-0070 */     }
/* FP:mod.rs-0071 */ }
/* FP:mod.rs-0072 */ 
/* FP:mod.rs-0073 */ impl<C>
/* FP:mod.rs-0074 */     MaybeTransmutableQuery<
/* FP:mod.rs-0075 */         Tree<<C as QueryContext>::Def, <C as QueryContext>::Region, <C as QueryContext>::Type>,
/* FP:mod.rs-0076 */         C,
/* FP:mod.rs-0077 */     >
/* FP:mod.rs-0078 */ where
/* FP:mod.rs-0079 */     C: QueryContext,
/* FP:mod.rs-0080 */ {
/* FP:mod.rs-0081 */     /// Answers whether a `Tree` is transmutable into another `Tree`.
/* FP:mod.rs-0082 */     ///
/* FP:mod.rs-0083 */     /// This method begins by de-def'ing `src` and `dst`, and prunes private paths from `dst`,
/* FP:mod.rs-0084 */     /// then converts `src` and `dst` to `Dfa`s, and computes an answer using those DFAs.
/* FP:mod.rs-0085 */     #[inline(always)]
/* FP:mod.rs-0086 */     #[instrument(level = "debug", skip(self), fields(src = ?self.src, dst = ?self.dst))]
/* FP:mod.rs-0087 */     pub(crate) fn answer(self) -> Answer<<C as QueryContext>::Region, <C as QueryContext>::Type> {
/* FP:mod.rs-0088 */         let Self { src, dst, assume, context } = self;
/* FP:mod.rs-0089 */ 
/* FP:mod.rs-0090 */         // Unconditionally remove all `Def` nodes from `src`, without pruning away the
/* FP:mod.rs-0091 */         // branches they appear in. This is valid to do for value-to-value
/* FP:mod.rs-0092 */         // transmutations, but not for `&mut T` to `&mut U`; we will need to be
/* FP:mod.rs-0093 */         // more sophisticated to handle transmutations between mutable
/* FP:mod.rs-0094 */         // references.
/* FP:mod.rs-0095 */         let src = src.prune(&|_def| false);
/* FP:mod.rs-0096 */ 
/* FP:mod.rs-0097 */         if src.is_inhabited() && !dst.is_inhabited() {
/* FP:mod.rs-0098 */             return Answer::No(Reason::DstUninhabited);
/* FP:mod.rs-0099 */         }
/* FP:mod.rs-0100 */ 
/* FP:mod.rs-0101 */         trace!(?src, "pruned src");
/* FP:mod.rs-0102 */ 
/* FP:mod.rs-0103 */         // Remove all `Def` nodes from `dst`, additionally...
/* FP:mod.rs-0104 */         let dst = if assume.safety {
/* FP:mod.rs-0105 */             // ...if safety is assumed, don't check if they carry safety
/* FP:mod.rs-0106 */             // invariants; retain all paths.
/* FP:mod.rs-0107 */             dst.prune(&|_def| false)
/* FP:mod.rs-0108 */         } else {
/* FP:mod.rs-0109 */             // ...otherwise, prune away all paths with safety invariants from
/* FP:mod.rs-0110 */             // the `Dst` layout.
/* FP:mod.rs-0111 */             dst.prune(&|def| def.has_safety_invariants())
/* FP:mod.rs-0112 */         };
/* FP:mod.rs-0113 */ 
/* FP:mod.rs-0114 */         trace!(?dst, "pruned dst");
/* FP:mod.rs-0115 */ 
/* FP:mod.rs-0116 */         // Convert `src` from a tree-based representation to an DFA-based
/* FP:mod.rs-0117 */         // representation. If the conversion fails because `src` is uninhabited,
/* FP:mod.rs-0118 */         // conclude that the transmutation is acceptable, because instances of
/* FP:mod.rs-0119 */         // the `src` type do not exist.
/* FP:mod.rs-0120 */         let src = match Dfa::from_tree(src) {
/* FP:mod.rs-0121 */             Ok(src) => src,
/* FP:mod.rs-0122 */             Err(layout::Uninhabited) => return Answer::Yes,
/* FP:mod.rs-0123 */         };
/* FP:mod.rs-0124 */ 
/* FP:mod.rs-0125 */         // Convert `dst` from a tree-based representation to an DFA-based
/* FP:mod.rs-0126 */         // representation. If the conversion fails because `src` is uninhabited,
/* FP:mod.rs-0127 */         // conclude that the transmutation is unacceptable. Valid instances of
/* FP:mod.rs-0128 */         // the `dst` type do not exist, either because it's genuinely
/* FP:mod.rs-0129 */         // uninhabited, or because there are no branches of the tree that are
/* FP:mod.rs-0130 */         // free of safety invariants.
/* FP:mod.rs-0131 */         let dst = match Dfa::from_tree(dst) {
/* FP:mod.rs-0132 */             Ok(dst) => dst,
/* FP:mod.rs-0133 */             Err(layout::Uninhabited) => return Answer::No(Reason::DstMayHaveSafetyInvariants),
/* FP:mod.rs-0134 */         };
/* FP:mod.rs-0135 */ 
/* FP:mod.rs-0136 */         MaybeTransmutableQuery { src, dst, assume, context }.answer()
/* FP:mod.rs-0137 */     }
/* FP:mod.rs-0138 */ }
/* FP:mod.rs-0139 */ 
/* FP:mod.rs-0140 */ impl<C> MaybeTransmutableQuery<Dfa<<C as QueryContext>::Region, <C as QueryContext>::Type>, C>
/* FP:mod.rs-0141 */ where
/* FP:mod.rs-0142 */     C: QueryContext,
/* FP:mod.rs-0143 */ {
/* FP:mod.rs-0144 */     /// Answers whether a `Dfa` is transmutable into another `Dfa`.
/* FP:mod.rs-0145 */     pub(crate) fn answer(self) -> Answer<<C as QueryContext>::Region, <C as QueryContext>::Type> {
/* FP:mod.rs-0146 */         self.answer_memo(&mut Map::default(), self.src.start, self.dst.start)
/* FP:mod.rs-0147 */     }
/* FP:mod.rs-0148 */ 
/* FP:mod.rs-0149 */     #[inline(always)]
/* FP:mod.rs-0150 */     #[instrument(level = "debug", skip(self))]
/* FP:mod.rs-0151 */     fn answer_memo(
/* FP:mod.rs-0152 */         &self,
/* FP:mod.rs-0153 */         cache: &mut Map<
/* FP:mod.rs-0154 */             (dfa::State, dfa::State),
/* FP:mod.rs-0155 */             Answer<<C as QueryContext>::Region, <C as QueryContext>::Type>,
/* FP:mod.rs-0156 */         >,
/* FP:mod.rs-0157 */         src_state: dfa::State,
/* FP:mod.rs-0158 */         dst_state: dfa::State,
/* FP:mod.rs-0159 */     ) -> Answer<<C as QueryContext>::Region, <C as QueryContext>::Type> {
/* FP:mod.rs-0160 */         if let Some(answer) = cache.get(&(src_state, dst_state)) {
/* FP:mod.rs-0161 */             answer.clone()
/* FP:mod.rs-0162 */         } else {
/* FP:mod.rs-0163 */             let answer = ensure_sufficient_stack(|| self.answer_impl(cache, src_state, dst_state));
/* FP:mod.rs-0164 */             if let Some(..) = cache.insert((src_state, dst_state), answer.clone()) {
/* FP:mod.rs-0165 */                 panic!("failed to correctly cache transmutability")
/* FP:mod.rs-0166 */             }
/* FP:mod.rs-0167 */             answer
/* FP:mod.rs-0168 */         }
/* FP:mod.rs-0169 */     }
/* FP:mod.rs-0170 */ 
/* FP:mod.rs-0171 */     fn answer_impl(
/* FP:mod.rs-0172 */         &self,
/* FP:mod.rs-0173 */         cache: &mut Map<
/* FP:mod.rs-0174 */             (dfa::State, dfa::State),
/* FP:mod.rs-0175 */             Answer<<C as QueryContext>::Region, <C as QueryContext>::Type>,
/* FP:mod.rs-0176 */         >,
/* FP:mod.rs-0177 */         src_state: dfa::State,
/* FP:mod.rs-0178 */         dst_state: dfa::State,
/* FP:mod.rs-0179 */     ) -> Answer<<C as QueryContext>::Region, <C as QueryContext>::Type> {
/* FP:mod.rs-0180 */         debug!(?src_state, ?dst_state);
/* FP:mod.rs-0181 */         debug!(src = ?self.src);
/* FP:mod.rs-0182 */         debug!(dst = ?self.dst);
/* FP:mod.rs-0183 */         debug!(
/* FP:mod.rs-0184 */             src_transitions_len = self.src.transitions.len(),
/* FP:mod.rs-0185 */             dst_transitions_len = self.dst.transitions.len()
/* FP:mod.rs-0186 */         );
/* FP:mod.rs-0187 */         if dst_state == self.dst.accept {
/* FP:mod.rs-0188 */             // truncation: `size_of(Src) >= size_of(Dst)`
/* FP:mod.rs-0189 */             //
/* FP:mod.rs-0190 */             // Why is truncation OK to do? Because even though the Src is bigger, all we care about
/* FP:mod.rs-0191 */             // is whether we have enough data for the Dst to be valid in accordance with what its
/* FP:mod.rs-0192 */             // type dictates.
/* FP:mod.rs-0193 */             // For example, in a u8 to `()` transmutation, we have enough data available from the u8
/* FP:mod.rs-0194 */             // to transmute it to a `()` (though in this case does `()` really need any data to
/* FP:mod.rs-0195 */             // begin with? It doesn't). Same thing with u8 to fieldless struct.
/* FP:mod.rs-0196 */             // Now then, why is something like u8 to bool not allowed? That is not because the bool
/* FP:mod.rs-0197 */             // is smaller in size, but rather because those 2 bits that we are re-interpreting from
/* FP:mod.rs-0198 */             // the u8 could introduce invalid states for the bool type.
/* FP:mod.rs-0199 */             //
/* FP:mod.rs-0200 */             // So, if it's possible to transmute to a smaller Dst by truncating, and we can guarantee
/* FP:mod.rs-0201 */             // that none of the actually-used data can introduce an invalid state for Dst's type, we
/* FP:mod.rs-0202 */             // are able to safely transmute, even with truncation.
/* FP:mod.rs-0203 */             Answer::Yes
/* FP:mod.rs-0204 */         } else if src_state == self.src.accept {
/* FP:mod.rs-0205 */             // extension: `size_of(Src) <= size_of(Dst)`
/* FP:mod.rs-0206 */             if let Some(dst_state_prime) = self.dst.get_uninit_edge_dst(dst_state) {
/* FP:mod.rs-0207 */                 self.answer_memo(cache, src_state, dst_state_prime)
/* FP:mod.rs-0208 */             } else {
/* FP:mod.rs-0209 */                 Answer::No(Reason::DstIsTooBig)
/* FP:mod.rs-0210 */             }
/* FP:mod.rs-0211 */         } else {
/* FP:mod.rs-0212 */             let src_quantifier = if self.assume.validity {
/* FP:mod.rs-0213 */                 // if the compiler may assume that the programmer is doing additional validity checks,
/* FP:mod.rs-0214 */                 // (e.g.: that `src != 3u8` when the destination type is `bool`)
/* FP:mod.rs-0215 */                 // then there must exist at least one transition out of `src_state` such that the transmute is viable...
/* FP:mod.rs-0216 */                 Quantifier::ThereExists
/* FP:mod.rs-0217 */             } else {
/* FP:mod.rs-0218 */                 // if the compiler cannot assume that the programmer is doing additional validity checks,
/* FP:mod.rs-0219 */                 // then for all transitions out of `src_state`, such that the transmute is viable...
/* FP:mod.rs-0220 */                 // then there must exist at least one transition out of `dst_state` such that the transmute is viable...
/* FP:mod.rs-0221 */                 Quantifier::ForAll
/* FP:mod.rs-0222 */             };
/* FP:mod.rs-0223 */ 
/* FP:mod.rs-0224 */             let bytes_answer = src_quantifier.apply(
/* FP:mod.rs-0225 */                 union(self.src.bytes_from(src_state), self.dst.bytes_from(dst_state)).filter_map(
/* FP:mod.rs-0226 */                     |(_range, (src_state_prime, dst_state_prime))| {
/* FP:mod.rs-0227 */                         match (src_state_prime, dst_state_prime) {
/* FP:mod.rs-0228 */                             // No matching transitions in `src`. Skip.
/* FP:mod.rs-0229 */                             (None, _) => None,
/* FP:mod.rs-0230 */                             // No matching transitions in `dst`. Fail.
/* FP:mod.rs-0231 */                             (Some(_), None) => Some(Answer::No(Reason::DstIsBitIncompatible)),
/* FP:mod.rs-0232 */                             // Matching transitions. Continue with successor states.
/* FP:mod.rs-0233 */                             (Some(src_state_prime), Some(dst_state_prime)) => {
/* FP:mod.rs-0234 */                                 Some(self.answer_memo(cache, src_state_prime, dst_state_prime))
/* FP:mod.rs-0235 */                             }
/* FP:mod.rs-0236 */                         }
/* FP:mod.rs-0237 */                     },
/* FP:mod.rs-0238 */                 ),
/* FP:mod.rs-0239 */             );
/* FP:mod.rs-0240 */ 
/* FP:mod.rs-0241 */             // The below early returns reflect how this code would behave:
/* FP:mod.rs-0242 */             //   if self.assume.validity {
/* FP:mod.rs-0243 */             //       or(bytes_answer, refs_answer)
/* FP:mod.rs-0244 */             //   } else {
/* FP:mod.rs-0245 */             //       and(bytes_answer, refs_answer)
/* FP:mod.rs-0246 */             //   }
/* FP:mod.rs-0247 */             // ...if `refs_answer` was computed lazily. The below early
/* FP:mod.rs-0248 */             // returns can be deleted without impacting the correctness of
/* FP:mod.rs-0249 */             // the algorithm; only its performance.
/* FP:mod.rs-0250 */             debug!(?bytes_answer);
/* FP:mod.rs-0251 */             match bytes_answer {
/* FP:mod.rs-0252 */                 Answer::No(_) if !self.assume.validity => return bytes_answer,
/* FP:mod.rs-0253 */                 Answer::Yes if self.assume.validity => return bytes_answer,
/* FP:mod.rs-0254 */                 _ => {}
/* FP:mod.rs-0255 */             };
/* FP:mod.rs-0256 */ 
/* FP:mod.rs-0257 */             let refs_answer = src_quantifier.apply(
/* FP:mod.rs-0258 */                 // for each reference transition out of `src_state`...
/* FP:mod.rs-0259 */                 self.src.refs_from(src_state).map(|(src_ref, src_state_prime)| {
/* FP:mod.rs-0260 */                     // ...there exists a reference transition out of `dst_state`...
/* FP:mod.rs-0261 */                     Quantifier::ThereExists.apply(self.dst.refs_from(dst_state).map(
/* FP:mod.rs-0262 */                         |(dst_ref, dst_state_prime)| {
/* FP:mod.rs-0263 */                             if !src_ref.is_mut && dst_ref.is_mut {
/* FP:mod.rs-0264 */                                 Answer::No(Reason::DstIsMoreUnique)
/* FP:mod.rs-0265 */                             } else if !self.assume.alignment
/* FP:mod.rs-0266 */                                 && src_ref.referent_align < dst_ref.referent_align
/* FP:mod.rs-0267 */                             {
/* FP:mod.rs-0268 */                                 Answer::No(Reason::DstHasStricterAlignment {
/* FP:mod.rs-0269 */                                     src_min_align: src_ref.referent_align,
/* FP:mod.rs-0270 */                                     dst_min_align: dst_ref.referent_align,
/* FP:mod.rs-0271 */                                 })
/* FP:mod.rs-0272 */                             } else if dst_ref.referent_size > src_ref.referent_size {
/* FP:mod.rs-0273 */                                 Answer::No(Reason::DstRefIsTooBig {
/* FP:mod.rs-0274 */                                     src: src_ref.referent,
/* FP:mod.rs-0275 */                                     src_size: src_ref.referent_size,
/* FP:mod.rs-0276 */                                     dst: dst_ref.referent,
/* FP:mod.rs-0277 */                                     dst_size: dst_ref.referent_size,
/* FP:mod.rs-0278 */                                 })
/* FP:mod.rs-0279 */                             } else {
/* FP:mod.rs-0280 */                                 let mut conditions = Vec::with_capacity(4);
/* FP:mod.rs-0281 */                                 let mut is_transmutable =
/* FP:mod.rs-0282 */                                     |src: Reference<_, _>, dst: Reference<_, _>| {
/* FP:mod.rs-0283 */                                         conditions.push(Condition::Transmutable {
/* FP:mod.rs-0284 */                                             src: src.referent,
/* FP:mod.rs-0285 */                                             dst: dst.referent,
/* FP:mod.rs-0286 */                                         });
/* FP:mod.rs-0287 */                                         if !self.assume.lifetimes {
/* FP:mod.rs-0288 */                                             conditions.push(Condition::Outlives {
/* FP:mod.rs-0289 */                                                 long: src.region,
/* FP:mod.rs-0290 */                                                 short: dst.region,
/* FP:mod.rs-0291 */                                             });
/* FP:mod.rs-0292 */                                         }
/* FP:mod.rs-0293 */                                     };
/* FP:mod.rs-0294 */ 
/* FP:mod.rs-0295 */                                 is_transmutable(src_ref, dst_ref);
/* FP:mod.rs-0296 */ 
/* FP:mod.rs-0297 */                                 if dst_ref.is_mut {
/* FP:mod.rs-0298 */                                     is_transmutable(dst_ref, src_ref);
/* FP:mod.rs-0299 */                                 } else {
/* FP:mod.rs-0300 */                                     conditions.push(Condition::Immutable { ty: dst_ref.referent });
/* FP:mod.rs-0301 */                                 }
/* FP:mod.rs-0302 */ 
/* FP:mod.rs-0303 */                                 Answer::If(Condition::IfAll(conditions)).and(self.answer_memo(
/* FP:mod.rs-0304 */                                     cache,
/* FP:mod.rs-0305 */                                     src_state_prime,
/* FP:mod.rs-0306 */                                     dst_state_prime,
/* FP:mod.rs-0307 */                                 ))
/* FP:mod.rs-0308 */                             }
/* FP:mod.rs-0309 */                         },
/* FP:mod.rs-0310 */                     ))
/* FP:mod.rs-0311 */                 }),
/* FP:mod.rs-0312 */             );
/* FP:mod.rs-0313 */ 
/* FP:mod.rs-0314 */             if self.assume.validity {
/* FP:mod.rs-0315 */                 bytes_answer.or(refs_answer)
/* FP:mod.rs-0316 */             } else {
/* FP:mod.rs-0317 */                 bytes_answer.and(refs_answer)
/* FP:mod.rs-0318 */             }
/* FP:mod.rs-0319 */         }
/* FP:mod.rs-0320 */     }
/* FP:mod.rs-0321 */ }
/* FP:mod.rs-0322 */ 
/* FP:mod.rs-0323 */ impl<R, T> Answer<R, T> {
/* FP:mod.rs-0324 */     fn and(self, rhs: Answer<R, T>) -> Answer<R, T> {
/* FP:mod.rs-0325 */         let lhs = self;
/* FP:mod.rs-0326 */         match (lhs, rhs) {
/* FP:mod.rs-0327 */             // If both are errors, then we should return the more specific one
/* FP:mod.rs-0328 */             (Answer::No(Reason::DstIsBitIncompatible), Answer::No(reason))
/* FP:mod.rs-0329 */             | (Answer::No(reason), Answer::No(_))
/* FP:mod.rs-0330 */             // If either is an error, return it
/* FP:mod.rs-0331 */             | (Answer::No(reason), _) | (_, Answer::No(reason)) => Answer::No(reason),
/* FP:mod.rs-0332 */             // If only one side has a condition, pass it along
/* FP:mod.rs-0333 */             | (Answer::Yes, other) | (other, Answer::Yes) => other,
/* FP:mod.rs-0334 */             // If both sides have IfAll conditions, merge them
/* FP:mod.rs-0335 */             (Answer::If(Condition::IfAll(mut lhs)), Answer::If(Condition::IfAll(ref mut rhs))) => {
/* FP:mod.rs-0336 */                 lhs.append(rhs);
/* FP:mod.rs-0337 */                 Answer::If(Condition::IfAll(lhs))
/* FP:mod.rs-0338 */             }
/* FP:mod.rs-0339 */             // If only one side is an IfAll, add the other Condition to it
/* FP:mod.rs-0340 */             (Answer::If(cond), Answer::If(Condition::IfAll(mut conds)))
/* FP:mod.rs-0341 */             | (Answer::If(Condition::IfAll(mut conds)), Answer::If(cond)) => {
/* FP:mod.rs-0342 */                 conds.push(cond);
/* FP:mod.rs-0343 */                 Answer::If(Condition::IfAll(conds))
/* FP:mod.rs-0344 */             }
/* FP:mod.rs-0345 */             // Otherwise, both lhs and rhs conditions can be combined in a parent IfAll
/* FP:mod.rs-0346 */             (Answer::If(lhs), Answer::If(rhs)) => Answer::If(Condition::IfAll(vec![lhs, rhs])),
/* FP:mod.rs-0347 */         }
/* FP:mod.rs-0348 */     }
/* FP:mod.rs-0349 */ 
/* FP:mod.rs-0350 */     fn or(self, rhs: Answer<R, T>) -> Answer<R, T> {
/* FP:mod.rs-0351 */         let lhs = self;
/* FP:mod.rs-0352 */         match (lhs, rhs) {
/* FP:mod.rs-0353 */             // If both are errors, then we should return the more specific one
/* FP:mod.rs-0354 */             (Answer::No(Reason::DstIsBitIncompatible), Answer::No(reason))
/* FP:mod.rs-0355 */             | (Answer::No(reason), Answer::No(_)) => Answer::No(reason),
/* FP:mod.rs-0356 */             // Otherwise, errors can be ignored for the rest of the pattern matching
/* FP:mod.rs-0357 */             (Answer::No(_), other) | (other, Answer::No(_)) => other.or(Answer::Yes),
/* FP:mod.rs-0358 */             // If only one side has a condition, pass it along
/* FP:mod.rs-0359 */             (Answer::Yes, other) | (other, Answer::Yes) => other,
/* FP:mod.rs-0360 */             // If both sides have IfAny conditions, merge them
/* FP:mod.rs-0361 */             (Answer::If(Condition::IfAny(mut lhs)), Answer::If(Condition::IfAny(ref mut rhs))) => {
/* FP:mod.rs-0362 */                 lhs.append(rhs);
/* FP:mod.rs-0363 */                 Answer::If(Condition::IfAny(lhs))
/* FP:mod.rs-0364 */             }
/* FP:mod.rs-0365 */             // If only one side is an IfAny, add the other Condition to it
/* FP:mod.rs-0366 */             (Answer::If(cond), Answer::If(Condition::IfAny(mut conds)))
/* FP:mod.rs-0367 */             | (Answer::If(Condition::IfAny(mut conds)), Answer::If(cond)) => {
/* FP:mod.rs-0368 */                 conds.push(cond);
/* FP:mod.rs-0369 */                 Answer::If(Condition::IfAny(conds))
/* FP:mod.rs-0370 */             }
/* FP:mod.rs-0371 */             // Otherwise, both lhs and rhs conditions can be combined in a parent IfAny
/* FP:mod.rs-0372 */             (Answer::If(lhs), Answer::If(rhs)) => Answer::If(Condition::IfAny(vec![lhs, rhs])),
/* FP:mod.rs-0373 */         }
/* FP:mod.rs-0374 */     }
/* FP:mod.rs-0375 */ }
/* FP:mod.rs-0376 */ 
/* FP:mod.rs-0377 */ enum Quantifier {
/* FP:mod.rs-0378 */     ThereExists,
/* FP:mod.rs-0379 */     ForAll,
/* FP:mod.rs-0380 */ }
/* FP:mod.rs-0381 */ 
/* FP:mod.rs-0382 */ impl Quantifier {
/* FP:mod.rs-0383 */     fn apply<R, T, I>(&self, iter: I) -> Answer<R, T>
/* FP:mod.rs-0384 */     where
/* FP:mod.rs-0385 */         R: layout::Region,
/* FP:mod.rs-0386 */         T: layout::Type,
/* FP:mod.rs-0387 */         I: IntoIterator<Item = Answer<R, T>>,
/* FP:mod.rs-0388 */     {
/* FP:mod.rs-0389 */         use std::ops::ControlFlow::{Break, Continue};
/* FP:mod.rs-0390 */ 
/* FP:mod.rs-0391 */         let (init, try_fold_f): (_, fn(_, _) -> _) = match self {
/* FP:mod.rs-0392 */             Self::ThereExists => {
/* FP:mod.rs-0393 */                 (Answer::No(Reason::DstIsBitIncompatible), |accum: Answer<R, T>, next| match accum
/* FP:mod.rs-0394 */                     .or(next)
/* FP:mod.rs-0395 */                 {
/* FP:mod.rs-0396 */                     Answer::Yes => Break(Answer::Yes),
/* FP:mod.rs-0397 */                     maybe => Continue(maybe),
/* FP:mod.rs-0398 */                 })
/* FP:mod.rs-0399 */             }
/* FP:mod.rs-0400 */             Self::ForAll => (Answer::Yes, |accum: Answer<R, T>, next| {
/* FP:mod.rs-0401 */                 let answer = accum.and(next);
/* FP:mod.rs-0402 */                 match answer {
/* FP:mod.rs-0403 */                     Answer::No(_) => Break(answer),
/* FP:mod.rs-0404 */                     maybe => Continue(maybe),
/* FP:mod.rs-0405 */                 }
/* FP:mod.rs-0406 */             }),
/* FP:mod.rs-0407 */         };
/* FP:mod.rs-0408 */ 
/* FP:mod.rs-0409 */         let (Continue(result) | Break(result)) = iter.into_iter().try_fold(init, try_fold_f);
/* FP:mod.rs-0410 */         result
/* FP:mod.rs-0411 */     }
/* FP:mod.rs-0412 */ }