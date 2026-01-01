/* FP:suggestions.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_traits_suggestions_UNPARSEABLE_0001
/* FP:suggestions.rs-0002 */ // ignore-tidy-filelength
/* FP:suggestions.rs-0003 */ 
/* FP:suggestions.rs-0004 */ use std::assert_matches::debug_assert_matches;
/* FP:suggestions.rs-0005 */ use std::borrow::Cow;
/* FP:suggestions.rs-0006 */ use std::iter;
/* FP:suggestions.rs-0007 */ use std::path::PathBuf;
/* FP:suggestions.rs-0008 */ 
/* FP:suggestions.rs-0009 */ use itertools::{EitherOrBoth, Itertools};
/* FP:suggestions.rs-0010 */ use crate::rustc_abi::ExternAbi;
/* FP:suggestions.rs-0011 */ use crate::rustc_data_structures::fx::FxHashSet;
/* FP:suggestions.rs-0012 */ use crate::rustc_data_structures::stack::ensure_sufficient_stack;
/* FP:suggestions.rs-0013 */ use crate::rustc_complete::codes::*;
/* FP:suggestions.rs-0014 */ use crate::rustc_complete::{
/* FP:suggestions.rs-0015 */     Applicability, Diag, EmissionGuarantee, MultiSpan, Style, SuggestionStyle, pluralize,
/* FP:suggestions.rs-0016 */     struct_span_code_err,
/* FP:suggestions.rs-0017 */ };
/* FP:suggestions.rs-0018 */ use crate::rustc_complete::def::{CtorOf, DefKind, Res};
/* FP:suggestions.rs-0019 */ use crate::rustc_complete::def_id::DefId;
/* FP:suggestions.rs-0020 */ use crate::rustc_complete::intravisit::{Visitor, VisitorExt};
/* FP:suggestions.rs-0021 */ use crate::rustc_complete::lang_items::LangItem;
/* FP:suggestions.rs-0022 */ use crate::rustc_complete::{
/* FP:suggestions.rs-0023 */     self as hir, AmbigArg, CoroutineDesugaring, CoroutineKind, CoroutineSource, Expr, HirId, Node,
/* FP:suggestions.rs-0024 */     expr_needs_parens, is_range_literal,
/* FP:suggestions.rs-0025 */ };
/* FP:suggestions.rs-0026 */ use crate::rustc_infer::infer::{BoundRegionConversionTime, DefineOpaqueTypes, InferCtxt, InferOk};
/* FP:suggestions.rs-0027 */ use crate::rustc_complete::middle::privacy::Level;
/* FP:suggestions.rs-0028 */ use crate::rustc_complete::traits::IsConstable;
/* FP:suggestions.rs-0029 */ use crate::rustc_complete::ty::error::TypeError;
/* FP:suggestions.rs-0030 */ use crate::rustc_complete::ty::print::{
/* FP:suggestions.rs-0031 */     PrintPolyTraitPredicateExt as _, PrintPolyTraitRefExt, PrintTraitPredicateExt as _,
/* FP:suggestions.rs-0032 */     with_forced_trimmed_paths, with_no_trimmed_paths, with_types_for_suggestion,
/* FP:suggestions.rs-0033 */ };
/* FP:suggestions.rs-0034 */ use crate::rustc_complete::ty::{
/* FP:suggestions.rs-0035 */     self, AdtKind, GenericArgs, InferTy, IsSuggestable, Ty, TyCtxt, TypeFoldable, TypeFolder,
/* FP:suggestions.rs-0036 */     TypeSuperFoldable, TypeSuperVisitable, TypeVisitableExt, TypeVisitor, TypeckResults, Upcast,
/* FP:suggestions.rs-0037 */     suggest_arbitrary_trait_bound, suggest_constraining_type_param,
/* FP:suggestions.rs-0038 */ };
/* FP:suggestions.rs-0039 */ use crate::rustc_complete::{bug, span_bug};
/* FP:suggestions.rs-0040 */ use crate::rustc_complete::def_id::LocalDefId;
/* FP:suggestions.rs-0041 */ use crate::rustc_complete::{
/* FP:suggestions.rs-0042 */     BytePos, DUMMY_SP, DesugaringKind, ExpnKind, Ident, MacroKind, Span, Symbol, kw, sym,
/* FP:suggestions.rs-0043 */ };
/* FP:suggestions.rs-0044 */ use tracing::{debug, instrument};
/* FP:suggestions.rs-0045 */ 
/* FP:suggestions.rs-0046 */ use super::{
/* FP:suggestions.rs-0047 */     DefIdOrName, FindExprBySpan, ImplCandidate, Obligation, ObligationCause, ObligationCauseCode,
/* FP:suggestions.rs-0048 */     PredicateObligation,
/* FP:suggestions.rs-0049 */ };
/* FP:suggestions.rs-0050 */ use crate::error_reporting::TypeErrCtxt;
/* FP:suggestions.rs-0051 */ use crate::errors;
/* FP:suggestions.rs-0052 */ use crate::infer::InferCtxtExt as _;
/* FP:suggestions.rs-0053 */ use crate::traits::query::evaluate_obligation::InferCtxtExt as _;
/* FP:suggestions.rs-0054 */ use crate::traits::{ImplDerivedCause, NormalizeExt, ObligationCtxt};
/* FP:suggestions.rs-0055 */ 
/* FP:suggestions.rs-0056 */ #[derive(Debug)]
/* FP:suggestions.rs-0057 */ pub enum CoroutineInteriorOrUpvar {
/* FP:suggestions.rs-0058 */     // span of interior type
/* FP:suggestions.rs-0059 */     Interior(Span, Option<(Span, Option<Span>)>),
/* FP:suggestions.rs-0060 */     // span of upvar
/* FP:suggestions.rs-0061 */     Upvar(Span),
/* FP:suggestions.rs-0062 */ }
/* FP:suggestions.rs-0063 */ 
/* FP:suggestions.rs-0064 */ // This type provides a uniform interface to retrieve data on coroutines, whether it originated from
/* FP:suggestions.rs-0065 */ // the local crate being compiled or from a foreign crate.
/* FP:suggestions.rs-0066 */ #[derive(Debug)]
/* FP:suggestions.rs-0067 */ struct CoroutineData<'a, 'tcx>(&'a TypeckResults<'tcx>);
/* FP:suggestions.rs-0068 */ 
/* FP:suggestions.rs-0069 */ impl<'a, 'tcx> CoroutineData<'a, 'tcx> {
/* FP:suggestions.rs-0070 */     /// Try to get information about variables captured by the coroutine that matches a type we are
/* FP:suggestions.rs-0071 */     /// looking for with `ty_matches` function. We uses it to find upvar which causes a failure to
/* FP:suggestions.rs-0072 */     /// meet an obligation
/* FP:suggestions.rs-0073 */     fn try_get_upvar_span<F>(
/* FP:suggestions.rs-0074 */         &self,
/* FP:suggestions.rs-0075 */         infer_context: &InferCtxt<'tcx>,
/* FP:suggestions.rs-0076 */         coroutine_did: DefId,
/* FP:suggestions.rs-0077 */         ty_matches: F,
/* FP:suggestions.rs-0078 */     ) -> Option<CoroutineInteriorOrUpvar>
/* FP:suggestions.rs-0079 */     where
/* FP:suggestions.rs-0080 */         F: Fn(ty::Binder<'tcx, Ty<'tcx>>) -> bool,
/* FP:suggestions.rs-0081 */     {
/* FP:suggestions.rs-0082 */         infer_context.tcx.upvars_mentioned(coroutine_did).and_then(|upvars| {
/* FP:suggestions.rs-0083 */             upvars.iter().find_map(|(upvar_id, upvar)| {
/* FP:suggestions.rs-0084 */                 let upvar_ty = self.0.node_type(*upvar_id);
/* FP:suggestions.rs-0085 */                 let upvar_ty = infer_context.resolve_vars_if_possible(upvar_ty);
/* FP:suggestions.rs-0086 */                 ty_matches(ty::Binder::dummy(upvar_ty))
/* FP:suggestions.rs-0087 */                     .then(|| CoroutineInteriorOrUpvar::Upvar(upvar.span))
/* FP:suggestions.rs-0088 */             })
/* FP:suggestions.rs-0089 */         })
/* FP:suggestions.rs-0090 */     }
/* FP:suggestions.rs-0091 */ 
/* FP:suggestions.rs-0092 */     /// Try to get the span of a type being awaited on that matches the type we are looking with the
/* FP:suggestions.rs-0093 */     /// `ty_matches` function. We uses it to find awaited type which causes a failure to meet an
/* FP:suggestions.rs-0094 */     /// obligation
/* FP:suggestions.rs-0095 */     fn get_from_await_ty<F>(
/* FP:suggestions.rs-0096 */         &self,
/* FP:suggestions.rs-0097 */         visitor: AwaitsVisitor,
/* FP:suggestions.rs-0098 */         tcx: TyCtxt<'tcx>,
/* FP:suggestions.rs-0099 */         ty_matches: F,
/* FP:suggestions.rs-0100 */     ) -> Option<Span>
/* FP:suggestions.rs-0101 */     where
/* FP:suggestions.rs-0102 */         F: Fn(ty::Binder<'tcx, Ty<'tcx>>) -> bool,
/* FP:suggestions.rs-0103 */     {
/* FP:suggestions.rs-0104 */         visitor
/* FP:suggestions.rs-0105 */             .awaits
/* FP:suggestions.rs-0106 */             .into_iter()
/* FP:suggestions.rs-0107 */             .map(|id| tcx.hir_expect_expr(id))
/* FP:suggestions.rs-0108 */             .find(|await_expr| ty_matches(ty::Binder::dummy(self.0.expr_ty_adjusted(await_expr))))
/* FP:suggestions.rs-0109 */             .map(|expr| expr.span)
/* FP:suggestions.rs-0110 */     }
/* FP:suggestions.rs-0111 */ }
/* FP:suggestions.rs-0112 */ 
/* FP:suggestions.rs-0113 */ fn predicate_constraint(generics: &hir::Generics<'_>, pred: ty::Predicate<'_>) -> (Span, String) {
/* FP:suggestions.rs-0114 */     (
/* FP:suggestions.rs-0115 */         generics.tail_span_for_predicate_suggestion(),
/* FP:suggestions.rs-0116 */         with_types_for_suggestion!(format!("{} {}", generics.add_where_or_trailing_comma(), pred)),
/* FP:suggestions.rs-0117 */     )
/* FP:suggestions.rs-0118 */ }
/* FP:suggestions.rs-0119 */ 
/* FP:suggestions.rs-0120 */ /// Type parameter needs more bounds. The trivial case is `T` `where T: Bound`, but
/* FP:suggestions.rs-0121 */ /// it can also be an `impl Trait` param that needs to be decomposed to a type
/* FP:suggestions.rs-0122 */ /// param for cleaner code.
/* FP:suggestions.rs-0123 */ pub fn suggest_restriction<'tcx, G: EmissionGuarantee>(
/* FP:suggestions.rs-0124 */     tcx: TyCtxt<'tcx>,
/* FP:suggestions.rs-0125 */     item_id: LocalDefId,
/* FP:suggestions.rs-0126 */     hir_generics: &hir::Generics<'tcx>,
/* FP:suggestions.rs-0127 */     msg: &str,
/* FP:suggestions.rs-0128 */     err: &mut Diag<'_, G>,
/* FP:suggestions.rs-0129 */     fn_sig: Option<&hir::FnSig<'_>>,
/* FP:suggestions.rs-0130 */     projection: Option<ty::AliasTy<'_>>,
/* FP:suggestions.rs-0131 */     trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-0132 */     // When we are dealing with a trait, `super_traits` will be `Some`:
/* FP:suggestions.rs-0133 */     // Given `trait T: A + B + C {}`
/* FP:suggestions.rs-0134 */     //              -  ^^^^^^^^^ GenericBounds
/* FP:suggestions.rs-0135 */     //              |
/* FP:suggestions.rs-0136 */     //              &Ident
/* FP:suggestions.rs-0137 */     super_traits: Option<(&Ident, &hir::GenericBounds<'_>)>,
/* FP:suggestions.rs-0138 */ ) {
/* FP:suggestions.rs-0139 */     if hir_generics.where_clause_span.from_expansion()
/* FP:suggestions.rs-0140 */         || hir_generics.where_clause_span.desugaring_kind().is_some()
/* FP:suggestions.rs-0141 */         || projection.is_some_and(|projection| {
/* FP:suggestions.rs-0142 */             (tcx.is_impl_trait_in_trait(projection.def_id)
/* FP:suggestions.rs-0143 */                 && !tcx.features().return_type_notation())
/* FP:suggestions.rs-0144 */                 || tcx.lookup_stability(projection.def_id).is_some_and(|stab| stab.is_unstable())
/* FP:suggestions.rs-0145 */         })
/* FP:suggestions.rs-0146 */     {
/* FP:suggestions.rs-0147 */         return;
/* FP:suggestions.rs-0148 */     }
/* FP:suggestions.rs-0149 */     let generics = tcx.generics_of(item_id);
/* FP:suggestions.rs-0150 */     // Given `fn foo(t: impl Trait)` where `Trait` requires assoc type `A`...
/* FP:suggestions.rs-0151 */     if let Some((param, bound_str, fn_sig)) =
/* FP:suggestions.rs-0152 */         fn_sig.zip(projection).and_then(|(sig, p)| match *p.self_ty().kind() {
/* FP:suggestions.rs-0153 */             // Shenanigans to get the `Trait` from the `impl Trait`.
/* FP:suggestions.rs-0154 */             ty::Param(param) => {
/* FP:suggestions.rs-0155 */                 let param_def = generics.type_param(param, tcx);
/* FP:suggestions.rs-0156 */                 if param_def.kind.is_synthetic() {
/* FP:suggestions.rs-0157 */                     let bound_str =
/* FP:suggestions.rs-0158 */                         param_def.name.as_str().strip_prefix("impl ")?.trim_start().to_string();
/* FP:suggestions.rs-0159 */                     return Some((param_def, bound_str, sig));
/* FP:suggestions.rs-0160 */                 }
/* FP:suggestions.rs-0161 */                 None
/* FP:suggestions.rs-0162 */             }
/* FP:suggestions.rs-0163 */             _ => None,
/* FP:suggestions.rs-0164 */         })
/* FP:suggestions.rs-0165 */     {
/* FP:suggestions.rs-0166 */         let type_param_name = hir_generics.params.next_type_param_name(Some(&bound_str));
/* FP:suggestions.rs-0167 */         let trait_pred = trait_pred.fold_with(&mut ReplaceImplTraitFolder {
/* FP:suggestions.rs-0168 */             tcx,
/* FP:suggestions.rs-0169 */             param,
/* FP:suggestions.rs-0170 */             replace_ty: ty::ParamTy::new(generics.count() as u32, Symbol::intern(&type_param_name))
/* FP:suggestions.rs-0171 */                 .to_ty(tcx),
/* FP:suggestions.rs-0172 */         });
/* FP:suggestions.rs-0173 */         if !trait_pred.is_suggestable(tcx, false) {
/* FP:suggestions.rs-0174 */             return;
/* FP:suggestions.rs-0175 */         }
/* FP:suggestions.rs-0176 */         // We know we have an `impl Trait` that doesn't satisfy a required projection.
/* FP:suggestions.rs-0177 */ 
/* FP:suggestions.rs-0178 */         // Find all of the occurrences of `impl Trait` for `Trait` in the function arguments'
/* FP:suggestions.rs-0179 */         // types. There should be at least one, but there might be *more* than one. In that
/* FP:suggestions.rs-0180 */         // case we could just ignore it and try to identify which one needs the restriction,
/* FP:suggestions.rs-0181 */         // but instead we choose to suggest replacing all instances of `impl Trait` with `T`
/* FP:suggestions.rs-0182 */         // where `T: Trait`.
/* FP:suggestions.rs-0183 */         let mut ty_spans = vec![];
/* FP:suggestions.rs-0184 */         for input in fn_sig.decl.inputs {
/* FP:suggestions.rs-0185 */             ReplaceImplTraitVisitor { ty_spans: &mut ty_spans, param_did: param.def_id }
/* FP:suggestions.rs-0186 */                 .visit_ty_unambig(input);
/* FP:suggestions.rs-0187 */         }
/* FP:suggestions.rs-0188 */         // The type param `T: Trait` we will suggest to introduce.
/* FP:suggestions.rs-0189 */         let type_param = format!("{type_param_name}: {bound_str}");
/* FP:suggestions.rs-0190 */ 
/* FP:suggestions.rs-0191 */         let mut sugg = vec![
/* FP:suggestions.rs-0192 */             if let Some(span) = hir_generics.span_for_param_suggestion() {
/* FP:suggestions.rs-0193 */                 (span, format!(", {type_param}"))
/* FP:suggestions.rs-0194 */             } else {
/* FP:suggestions.rs-0195 */                 (hir_generics.span, format!("<{type_param}>"))
/* FP:suggestions.rs-0196 */             },
/* FP:suggestions.rs-0197 */             // `fn foo(t: impl Trait)`
/* FP:suggestions.rs-0198 */             //                       ^ suggest `where <T as Trait>::A: Bound`
/* FP:suggestions.rs-0199 */             predicate_constraint(hir_generics, trait_pred.upcast(tcx)),
/* FP:suggestions.rs-0200 */         ];
/* FP:suggestions.rs-0201 */         sugg.extend(ty_spans.into_iter().map(|s| (s, type_param_name.to_string())));
/* FP:suggestions.rs-0202 */ 
/* FP:suggestions.rs-0203 */         // Suggest `fn foo<T: Trait>(t: T) where <T as Trait>::A: Bound`.
/* FP:suggestions.rs-0204 */         // FIXME: we should suggest `fn foo(t: impl Trait<A: Bound>)` instead.
/* FP:suggestions.rs-0205 */         err.multipart_suggestion(
/* FP:suggestions.rs-0206 */             "introduce a type parameter with a trait bound instead of using `impl Trait`",
/* FP:suggestions.rs-0207 */             sugg,
/* FP:suggestions.rs-0208 */             Applicability::MaybeIncorrect,
/* FP:suggestions.rs-0209 */         );
/* FP:suggestions.rs-0210 */     } else {
/* FP:suggestions.rs-0211 */         if !trait_pred.is_suggestable(tcx, false) {
/* FP:suggestions.rs-0212 */             return;
/* FP:suggestions.rs-0213 */         }
/* FP:suggestions.rs-0214 */         // Trivial case: `T` needs an extra bound: `T: Bound`.
/* FP:suggestions.rs-0215 */         let (sp, suggestion) = match (
/* FP:suggestions.rs-0216 */             hir_generics
/* FP:suggestions.rs-0217 */                 .params
/* FP:suggestions.rs-0218 */                 .iter()
/* FP:suggestions.rs-0219 */                 .find(|p| !matches!(p.kind, hir::GenericParamKind::Type { synthetic: true, .. })),
/* FP:suggestions.rs-0220 */             super_traits,
/* FP:suggestions.rs-0221 */         ) {
/* FP:suggestions.rs-0222 */             (_, None) => predicate_constraint(hir_generics, trait_pred.upcast(tcx)),
/* FP:suggestions.rs-0223 */             (None, Some((ident, []))) => (
/* FP:suggestions.rs-0224 */                 ident.span.shrink_to_hi(),
/* FP:suggestions.rs-0225 */                 format!(": {}", trait_pred.print_modifiers_and_trait_path()),
/* FP:suggestions.rs-0226 */             ),
/* FP:suggestions.rs-0227 */             (_, Some((_, [.., bounds]))) => (
/* FP:suggestions.rs-0228 */                 bounds.span().shrink_to_hi(),
/* FP:suggestions.rs-0229 */                 format!(" + {}", trait_pred.print_modifiers_and_trait_path()),
/* FP:suggestions.rs-0230 */             ),
/* FP:suggestions.rs-0231 */             (Some(_), Some((_, []))) => (
/* FP:suggestions.rs-0232 */                 hir_generics.span.shrink_to_hi(),
/* FP:suggestions.rs-0233 */                 format!(": {}", trait_pred.print_modifiers_and_trait_path()),
/* FP:suggestions.rs-0234 */             ),
/* FP:suggestions.rs-0235 */         };
/* FP:suggestions.rs-0236 */ 
/* FP:suggestions.rs-0237 */         err.span_suggestion_verbose(
/* FP:suggestions.rs-0238 */             sp,
/* FP:suggestions.rs-0239 */             format!("consider further restricting {msg}"),
/* FP:suggestions.rs-0240 */             suggestion,
/* FP:suggestions.rs-0241 */             Applicability::MachineApplicable,
/* FP:suggestions.rs-0242 */         );
/* FP:suggestions.rs-0243 */     }
/* FP:suggestions.rs-0244 */ }
/* FP:suggestions.rs-0245 */ 
/* FP:suggestions.rs-0246 */ impl<'a, 'tcx> TypeErrCtxt<'a, 'tcx> {
/* FP:suggestions.rs-0247 */     pub fn suggest_restricting_param_bound(
/* FP:suggestions.rs-0248 */         &self,
/* FP:suggestions.rs-0249 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-0250 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-0251 */         associated_ty: Option<(&'static str, Ty<'tcx>)>,
/* FP:suggestions.rs-0252 */         mut body_id: LocalDefId,
/* FP:suggestions.rs-0253 */     ) {
/* FP:suggestions.rs-0254 */         if trait_pred.skip_binder().polarity != ty::PredicatePolarity::Positive {
/* FP:suggestions.rs-0255 */             return;
/* FP:suggestions.rs-0256 */         }
/* FP:suggestions.rs-0257 */ 
/* FP:suggestions.rs-0258 */         let trait_pred = self.resolve_numeric_literals_with_default(trait_pred);
/* FP:suggestions.rs-0259 */ 
/* FP:suggestions.rs-0260 */         let self_ty = trait_pred.skip_binder().self_ty();
/* FP:suggestions.rs-0261 */         let (param_ty, projection) = match *self_ty.kind() {
/* FP:suggestions.rs-0262 */             ty::Param(_) => (true, None),
/* FP:suggestions.rs-0263 */             ty::Alias(ty::Projection, projection) => (false, Some(projection)),
/* FP:suggestions.rs-0264 */             _ => (false, None),
/* FP:suggestions.rs-0265 */         };
/* FP:suggestions.rs-0266 */ 
/* FP:suggestions.rs-0267 */         let mut finder = ParamFinder { .. };
/* FP:suggestions.rs-0268 */         finder.visit_binder(&trait_pred);
/* FP:suggestions.rs-0269 */ 
/* FP:suggestions.rs-0270 */         // FIXME: Add check for trait bound that is already present, particularly `?Sized` so we
/* FP:suggestions.rs-0271 */         //        don't suggest `T: Sized + ?Sized`.
/* FP:suggestions.rs-0272 */         loop {
/* FP:suggestions.rs-0273 */             let node = self.tcx.hir_node_by_def_id(body_id);
/* FP:suggestions.rs-0274 */             match node {
/* FP:suggestions.rs-0275 */                 hir::Node::Item(hir::Item {
/* FP:suggestions.rs-0276 */                     kind: hir::ItemKind::Trait(_, _, _, ident, generics, bounds, _),
/* FP:suggestions.rs-0277 */                     ..
/* FP:suggestions.rs-0278 */                 }) if self_ty == self.tcx.types.self_param => {
/* FP:suggestions.rs-0279 */                     assert!(param_ty);
/* FP:suggestions.rs-0280 */                     // Restricting `Self` for a single method.
/* FP:suggestions.rs-0281 */                     suggest_restriction(
/* FP:suggestions.rs-0282 */                         self.tcx,
/* FP:suggestions.rs-0283 */                         body_id,
/* FP:suggestions.rs-0284 */                         generics,
/* FP:suggestions.rs-0285 */                         "`Self`",
/* FP:suggestions.rs-0286 */                         err,
/* FP:suggestions.rs-0287 */                         None,
/* FP:suggestions.rs-0288 */                         projection,
/* FP:suggestions.rs-0289 */                         trait_pred,
/* FP:suggestions.rs-0290 */                         Some((&ident, bounds)),
/* FP:suggestions.rs-0291 */                     );
/* FP:suggestions.rs-0292 */                     return;
/* FP:suggestions.rs-0293 */                 }
/* FP:suggestions.rs-0294 */ 
/* FP:suggestions.rs-0295 */                 hir::Node::TraitItem(hir::TraitItem {
/* FP:suggestions.rs-0296 */                     generics,
/* FP:suggestions.rs-0297 */                     kind: hir::TraitItemKind::Fn(..),
/* FP:suggestions.rs-0298 */                     ..
/* FP:suggestions.rs-0299 */                 }) if self_ty == self.tcx.types.self_param => {
/* FP:suggestions.rs-0300 */                     assert!(param_ty);
/* FP:suggestions.rs-0301 */                     // Restricting `Self` for a single method.
/* FP:suggestions.rs-0302 */                     suggest_restriction(
/* FP:suggestions.rs-0303 */                         self.tcx, body_id, generics, "`Self`", err, None, projection, trait_pred,
/* FP:suggestions.rs-0304 */                         None,
/* FP:suggestions.rs-0305 */                     );
/* FP:suggestions.rs-0306 */                     return;
/* FP:suggestions.rs-0307 */                 }
/* FP:suggestions.rs-0308 */ 
/* FP:suggestions.rs-0309 */                 hir::Node::TraitItem(hir::TraitItem {
/* FP:suggestions.rs-0310 */                     generics,
/* FP:suggestions.rs-0311 */                     kind: hir::TraitItemKind::Fn(fn_sig, ..),
/* FP:suggestions.rs-0312 */                     ..
/* FP:suggestions.rs-0313 */                 })
/* FP:suggestions.rs-0314 */                 | hir::Node::ImplItem(hir::ImplItem {
/* FP:suggestions.rs-0315 */                     generics,
/* FP:suggestions.rs-0316 */                     kind: hir::ImplItemKind::Fn(fn_sig, ..),
/* FP:suggestions.rs-0317 */                     ..
/* FP:suggestions.rs-0318 */                 })
/* FP:suggestions.rs-0319 */                 | hir::Node::Item(hir::Item {
/* FP:suggestions.rs-0320 */                     kind: hir::ItemKind::Fn { sig: fn_sig, generics, .. },
/* FP:suggestions.rs-0321 */                     ..
/* FP:suggestions.rs-0322 */                 }) if projection.is_some() => {
/* FP:suggestions.rs-0323 */                     // Missing restriction on associated type of type parameter (unmet projection).
/* FP:suggestions.rs-0324 */                     suggest_restriction(
/* FP:suggestions.rs-0325 */                         self.tcx,
/* FP:suggestions.rs-0326 */                         body_id,
/* FP:suggestions.rs-0327 */                         generics,
/* FP:suggestions.rs-0328 */                         "the associated type",
/* FP:suggestions.rs-0329 */                         err,
/* FP:suggestions.rs-0330 */                         Some(fn_sig),
/* FP:suggestions.rs-0331 */                         projection,
/* FP:suggestions.rs-0332 */                         trait_pred,
/* FP:suggestions.rs-0333 */                         None,
/* FP:suggestions.rs-0334 */                     );
/* FP:suggestions.rs-0335 */                     return;
/* FP:suggestions.rs-0336 */                 }
/* FP:suggestions.rs-0337 */                 hir::Node::Item(hir::Item {
/* FP:suggestions.rs-0338 */                     kind:
/* FP:suggestions.rs-0339 */                         hir::ItemKind::Trait(_, _, _, _, generics, ..)
/* FP:suggestions.rs-0340 */                         | hir::ItemKind::Impl(hir::Impl { generics, .. }),
/* FP:suggestions.rs-0341 */                     ..
/* FP:suggestions.rs-0342 */                 }) if projection.is_some() => {
/* FP:suggestions.rs-0343 */                     // Missing restriction on associated type of type parameter (unmet projection).
/* FP:suggestions.rs-0344 */                     suggest_restriction(
/* FP:suggestions.rs-0345 */                         self.tcx,
/* FP:suggestions.rs-0346 */                         body_id,
/* FP:suggestions.rs-0347 */                         generics,
/* FP:suggestions.rs-0348 */                         "the associated type",
/* FP:suggestions.rs-0349 */                         err,
/* FP:suggestions.rs-0350 */                         None,
/* FP:suggestions.rs-0351 */                         projection,
/* FP:suggestions.rs-0352 */                         trait_pred,
/* FP:suggestions.rs-0353 */                         None,
/* FP:suggestions.rs-0354 */                     );
/* FP:suggestions.rs-0355 */                     return;
/* FP:suggestions.rs-0356 */                 }
/* FP:suggestions.rs-0357 */ 
/* FP:suggestions.rs-0358 */                 hir::Node::Item(hir::Item {
/* FP:suggestions.rs-0359 */                     kind:
/* FP:suggestions.rs-0360 */                         hir::ItemKind::Struct(_, generics, _)
/* FP:suggestions.rs-0361 */                         | hir::ItemKind::Enum(_, generics, _)
/* FP:suggestions.rs-0362 */                         | hir::ItemKind::Union(_, generics, _)
/* FP:suggestions.rs-0363 */                         | hir::ItemKind::Trait(_, _, _, _, generics, ..)
/* FP:suggestions.rs-0364 */                         | hir::ItemKind::Impl(hir::Impl { generics, .. })
/* FP:suggestions.rs-0365 */                         | hir::ItemKind::Fn { generics, .. }
/* FP:suggestions.rs-0366 */                         | hir::ItemKind::TyAlias(_, generics, _)
/* FP:suggestions.rs-0367 */                         | hir::ItemKind::Const(_, generics, _, _)
/* FP:suggestions.rs-0368 */                         | hir::ItemKind::TraitAlias(_, generics, _),
/* FP:suggestions.rs-0369 */                     ..
/* FP:suggestions.rs-0370 */                 })
/* FP:suggestions.rs-0371 */                 | hir::Node::TraitItem(hir::TraitItem { generics, .. })
/* FP:suggestions.rs-0372 */                 | hir::Node::ImplItem(hir::ImplItem { generics, .. })
/* FP:suggestions.rs-0373 */                     if param_ty =>
/* FP:suggestions.rs-0374 */                 {
/* FP:suggestions.rs-0375 */                     // We skip the 0'th arg (self) because we do not want
/* FP:suggestions.rs-0376 */                     // to consider the predicate as not suggestible if the
/* FP:suggestions.rs-0377 */                     // self type is an arg position `impl Trait` -- instead,
/* FP:suggestions.rs-0378 */                     // we handle that by adding ` + Bound` below.
/* FP:suggestions.rs-0379 */                     // FIXME(compiler-errors): It would be nice to do the same
/* FP:suggestions.rs-0380 */                     // this that we do in `suggest_restriction` and pull the
/* FP:suggestions.rs-0381 */                     // `impl Trait` into a new generic if it shows up somewhere
/* FP:suggestions.rs-0382 */                     // else in the predicate.
/* FP:suggestions.rs-0383 */                     if !trait_pred.skip_binder().trait_ref.args[1..]
/* FP:suggestions.rs-0384 */                         .iter()
/* FP:suggestions.rs-0385 */                         .all(|g| g.is_suggestable(self.tcx, false))
/* FP:suggestions.rs-0386 */                     {
/* FP:suggestions.rs-0387 */                         return;
/* FP:suggestions.rs-0388 */                     }
/* FP:suggestions.rs-0389 */                     // Missing generic type parameter bound.
/* FP:suggestions.rs-0390 */                     let param_name = self_ty.to_string();
/* FP:suggestions.rs-0391 */                     let mut constraint = with_no_trimmed_paths!(
/* FP:suggestions.rs-0392 */                         trait_pred.print_modifiers_and_trait_path().to_string()
/* FP:suggestions.rs-0393 */                     );
/* FP:suggestions.rs-0394 */ 
/* FP:suggestions.rs-0395 */                     if let Some((name, term)) = associated_ty {
/* FP:suggestions.rs-0396 */                         // FIXME: this case overlaps with code in TyCtxt::note_and_explain_type_err.
/* FP:suggestions.rs-0397 */                         // That should be extracted into a helper function.
/* FP:suggestions.rs-0398 */                         if let Some(stripped) = constraint.strip_suffix('>') {
/* FP:suggestions.rs-0399 */                             constraint = format!("{stripped}, {name} = {term}>");
/* FP:suggestions.rs-0400 */                         } else {
/* FP:suggestions.rs-0401 */                             constraint.push_str(&format!("<{name} = {term}>"));
/* FP:suggestions.rs-0402 */                         }
/* FP:suggestions.rs-0403 */                     }
/* FP:suggestions.rs-0404 */ 
/* FP:suggestions.rs-0405 */                     if suggest_constraining_type_param(
/* FP:suggestions.rs-0406 */                         self.tcx,
/* FP:suggestions.rs-0407 */                         generics,
/* FP:suggestions.rs-0408 */                         err,
/* FP:suggestions.rs-0409 */                         &param_name,
/* FP:suggestions.rs-0410 */                         &constraint,
/* FP:suggestions.rs-0411 */                         Some(trait_pred.def_id()),
/* FP:suggestions.rs-0412 */                         None,
/* FP:suggestions.rs-0413 */                     ) {
/* FP:suggestions.rs-0414 */                         return;
/* FP:suggestions.rs-0415 */                     }
/* FP:suggestions.rs-0416 */                 }
/* FP:suggestions.rs-0417 */ 
/* FP:suggestions.rs-0418 */                 hir::Node::TraitItem(hir::TraitItem {
/* FP:suggestions.rs-0419 */                     generics,
/* FP:suggestions.rs-0420 */                     kind: hir::TraitItemKind::Fn(..),
/* FP:suggestions.rs-0421 */                     ..
/* FP:suggestions.rs-0422 */                 })
/* FP:suggestions.rs-0423 */                 | hir::Node::ImplItem(hir::ImplItem {
/* FP:suggestions.rs-0424 */                     generics,
/* FP:suggestions.rs-0425 */                     impl_kind: hir::ImplItemImplKind::Inherent { .. },
/* FP:suggestions.rs-0426 */                     kind: hir::ImplItemKind::Fn(..),
/* FP:suggestions.rs-0427 */                     ..
/* FP:suggestions.rs-0428 */                 }) if finder.can_suggest_bound(generics) => {
/* FP:suggestions.rs-0429 */                     // Missing generic type parameter bound.
/* FP:suggestions.rs-0430 */                     suggest_arbitrary_trait_bound(
/* FP:suggestions.rs-0431 */                         self.tcx,
/* FP:suggestions.rs-0432 */                         generics,
/* FP:suggestions.rs-0433 */                         err,
/* FP:suggestions.rs-0434 */                         trait_pred,
/* FP:suggestions.rs-0435 */                         associated_ty,
/* FP:suggestions.rs-0436 */                     );
/* FP:suggestions.rs-0437 */                 }
/* FP:suggestions.rs-0438 */                 hir::Node::Item(hir::Item {
/* FP:suggestions.rs-0439 */                     kind:
/* FP:suggestions.rs-0440 */                         hir::ItemKind::Struct(_, generics, _)
/* FP:suggestions.rs-0441 */                         | hir::ItemKind::Enum(_, generics, _)
/* FP:suggestions.rs-0442 */                         | hir::ItemKind::Union(_, generics, _)
/* FP:suggestions.rs-0443 */                         | hir::ItemKind::Trait(_, _, _, _, generics, ..)
/* FP:suggestions.rs-0444 */                         | hir::ItemKind::Impl(hir::Impl { generics, .. })
/* FP:suggestions.rs-0445 */                         | hir::ItemKind::Fn { generics, .. }
/* FP:suggestions.rs-0446 */                         | hir::ItemKind::TyAlias(_, generics, _)
/* FP:suggestions.rs-0447 */                         | hir::ItemKind::Const(_, generics, _, _)
/* FP:suggestions.rs-0448 */                         | hir::ItemKind::TraitAlias(_, generics, _),
/* FP:suggestions.rs-0449 */                     ..
/* FP:suggestions.rs-0450 */                 }) if finder.can_suggest_bound(generics) => {
/* FP:suggestions.rs-0451 */                     // Missing generic type parameter bound.
/* FP:suggestions.rs-0452 */                     if suggest_arbitrary_trait_bound(
/* FP:suggestions.rs-0453 */                         self.tcx,
/* FP:suggestions.rs-0454 */                         generics,
/* FP:suggestions.rs-0455 */                         err,
/* FP:suggestions.rs-0456 */                         trait_pred,
/* FP:suggestions.rs-0457 */                         associated_ty,
/* FP:suggestions.rs-0458 */                     ) {
/* FP:suggestions.rs-0459 */                         return;
/* FP:suggestions.rs-0460 */                     }
/* FP:suggestions.rs-0461 */                 }
/* FP:suggestions.rs-0462 */                 hir::Node::Crate(..) => return,
/* FP:suggestions.rs-0463 */ 
/* FP:suggestions.rs-0464 */                 _ => {}
/* FP:suggestions.rs-0465 */             }
/* FP:suggestions.rs-0466 */             body_id = self.tcx.local_parent(body_id);
/* FP:suggestions.rs-0467 */         }
/* FP:suggestions.rs-0468 */     }
/* FP:suggestions.rs-0469 */ 
/* FP:suggestions.rs-0470 */     /// Provide a suggestion to dereference arguments to functions and binary operators, if that
/* FP:suggestions.rs-0471 */     /// would satisfy trait bounds.
/* FP:suggestions.rs-0472 */     pub(super) fn suggest_dereferences(
/* FP:suggestions.rs-0473 */         &self,
/* FP:suggestions.rs-0474 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-0475 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-0476 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-0477 */     ) -> bool {
/* FP:suggestions.rs-0478 */         let mut code = obligation.cause.code();
/* FP:suggestions.rs-0479 */         if let ObligationCauseCode::FunctionArg { arg_hir_id, call_hir_id, .. } = code
/* FP:suggestions.rs-0480 */             && let Some(typeck_results) = &self.typeck_results
/* FP:suggestions.rs-0481 */             && let hir::Node::Expr(expr) = self.tcx.hir_node(*arg_hir_id)
/* FP:suggestions.rs-0482 */             && let Some(arg_ty) = typeck_results.expr_ty_adjusted_opt(expr)
/* FP:suggestions.rs-0483 */         {
/* FP:suggestions.rs-0484 */             // Suggest dereferencing the argument to a function/method call if possible
/* FP:suggestions.rs-0485 */ 
/* FP:suggestions.rs-0486 */             // Get the root obligation, since the leaf obligation we have may be unhelpful (#87437)
/* FP:suggestions.rs-0487 */             let mut real_trait_pred = trait_pred;
/* FP:suggestions.rs-0488 */             while let Some((parent_code, parent_trait_pred)) = code.parent_with_predicate() {
/* FP:suggestions.rs-0489 */                 code = parent_code;
/* FP:suggestions.rs-0490 */                 if let Some(parent_trait_pred) = parent_trait_pred {
/* FP:suggestions.rs-0491 */                     real_trait_pred = parent_trait_pred;
/* FP:suggestions.rs-0492 */                 }
/* FP:suggestions.rs-0493 */             }
/* FP:suggestions.rs-0494 */ 
/* FP:suggestions.rs-0495 */             // We `instantiate_bound_regions_with_erased` here because `make_subregion` does not handle
/* FP:suggestions.rs-0496 */             // `ReBound`, and we don't particularly care about the regions.
/* FP:suggestions.rs-0497 */             let real_ty = self.tcx.instantiate_bound_regions_with_erased(real_trait_pred.self_ty());
/* FP:suggestions.rs-0498 */             if !self.can_eq(obligation.param_env, real_ty, arg_ty) {
/* FP:suggestions.rs-0499 */                 return false;
/* FP:suggestions.rs-0500 */             }
/* FP:suggestions.rs-0501 */ 
/* FP:suggestions.rs-0502 */             // Potentially, we'll want to place our dereferences under a `&`. We don't try this for
/* FP:suggestions.rs-0503 */             // `&mut`, since we can't be sure users will get the side-effects they want from it.
/* FP:suggestions.rs-0504 */             // If this doesn't work, we'll try removing the `&` in `suggest_remove_reference`.
/* FP:suggestions.rs-0505 */             // FIXME(dianne): this misses the case where users need both to deref and remove `&`s.
/* FP:suggestions.rs-0506 */             // This method could be combined with `TypeErrCtxt::suggest_remove_reference` to handle
/* FP:suggestions.rs-0507 */             // that, similar to what `FnCtxt::suggest_deref_or_ref` does.
/* FP:suggestions.rs-0508 */             let (is_under_ref, base_ty, span) = match expr.kind {
/* FP:suggestions.rs-0509 */                 hir::ExprKind::AddrOf(hir::BorrowKind::Ref, hir::Mutability::Not, subexpr)
/* FP:suggestions.rs-0510 */                     if let &ty::Ref(region, base_ty, hir::Mutability::Not) = real_ty.kind() =>
/* FP:suggestions.rs-0511 */                 {
/* FP:suggestions.rs-0512 */                     (Some(region), base_ty, subexpr.span)
/* FP:suggestions.rs-0513 */                 }
/* FP:suggestions.rs-0514 */                 // Don't suggest `*&mut`, etc.
/* FP:suggestions.rs-0515 */                 hir::ExprKind::AddrOf(..) => return false,
/* FP:suggestions.rs-0516 */                 _ => (None, real_ty, obligation.cause.span),
/* FP:suggestions.rs-0517 */             };
/* FP:suggestions.rs-0518 */ 
/* FP:suggestions.rs-0519 */             let autoderef = (self.autoderef_steps)(base_ty);
/* FP:suggestions.rs-0520 */             let mut is_boxed = base_ty.is_box();
/* FP:suggestions.rs-0521 */             if let Some(steps) = autoderef.into_iter().position(|(mut ty, obligations)| {
/* FP:suggestions.rs-0522 */                 // Ensure one of the following for dereferencing to be valid: we're passing by
/* FP:suggestions.rs-0523 */                 // reference, `ty` is `Copy`, or we're moving out of a (potentially nested) `Box`.
/* FP:suggestions.rs-0524 */                 let can_deref = is_under_ref.is_some()
/* FP:suggestions.rs-0525 */                     || self.type_is_copy_modulo_regions(obligation.param_env, ty)
/* FP:suggestions.rs-0526 */                     || ty.is_numeric() // for inference vars (presumably but not provably `Copy`)
/* FP:suggestions.rs-0527 */                     || is_boxed && self.type_is_sized_modulo_regions(obligation.param_env, ty);
/* FP:suggestions.rs-0528 */                 is_boxed &= ty.is_box();
/* FP:suggestions.rs-0529 */ 
/* FP:suggestions.rs-0530 */                 // Re-add the `&` if necessary
/* FP:suggestions.rs-0531 */                 if let Some(region) = is_under_ref {
/* FP:suggestions.rs-0532 */                     ty = Ty::new_ref(self.tcx, region, ty, hir::Mutability::Not);
/* FP:suggestions.rs-0533 */                 }
/* FP:suggestions.rs-0534 */ 
/* FP:suggestions.rs-0535 */                 // Remapping bound vars here
/* FP:suggestions.rs-0536 */                 let real_trait_pred_and_ty =
/* FP:suggestions.rs-0537 */                     real_trait_pred.map_bound(|inner_trait_pred| (inner_trait_pred, ty));
/* FP:suggestions.rs-0538 */                 let obligation = self.mk_trait_obligation_with_new_self_ty(
/* FP:suggestions.rs-0539 */                     obligation.param_env,
/* FP:suggestions.rs-0540 */                     real_trait_pred_and_ty,
/* FP:suggestions.rs-0541 */                 );
/* FP:suggestions.rs-0542 */ 
/* FP:suggestions.rs-0543 */                 can_deref
/* FP:suggestions.rs-0544 */                     && obligations
/* FP:suggestions.rs-0545 */                         .iter()
/* FP:suggestions.rs-0546 */                         .chain([&obligation])
/* FP:suggestions.rs-0547 */                         .all(|obligation| self.predicate_may_hold(obligation))
/* FP:suggestions.rs-0548 */             }) && steps > 0
/* FP:suggestions.rs-0549 */             {
/* FP:suggestions.rs-0550 */                 let derefs = "*".repeat(steps);
/* FP:suggestions.rs-0551 */                 let msg = "consider dereferencing here";
/* FP:suggestions.rs-0552 */                 let call_node = self.tcx.hir_node(*call_hir_id);
/* FP:suggestions.rs-0553 */                 let is_receiver = matches!(
/* FP:suggestions.rs-0554 */                     call_node,
/* FP:suggestions.rs-0555 */                     Node::Expr(hir::Expr {
/* FP:suggestions.rs-0556 */                         kind: hir::ExprKind::MethodCall(_, receiver_expr, ..),
/* FP:suggestions.rs-0557 */                         ..
/* FP:suggestions.rs-0558 */                     })
/* FP:suggestions.rs-0559 */                     if receiver_expr.hir_id == *arg_hir_id
/* FP:suggestions.rs-0560 */                 );
/* FP:suggestions.rs-0561 */                 if is_receiver {
/* FP:suggestions.rs-0562 */                     err.multipart_suggestion_verbose(
/* FP:suggestions.rs-0563 */                         msg,
/* FP:suggestions.rs-0564 */                         vec![
/* FP:suggestions.rs-0565 */                             (span.shrink_to_lo(), format!("({derefs}")),
/* FP:suggestions.rs-0566 */                             (span.shrink_to_hi(), ")".to_string()),
/* FP:suggestions.rs-0567 */                         ],
/* FP:suggestions.rs-0568 */                         Applicability::MachineApplicable,
/* FP:suggestions.rs-0569 */                     )
/* FP:suggestions.rs-0570 */                 } else {
/* FP:suggestions.rs-0571 */                     err.span_suggestion_verbose(
/* FP:suggestions.rs-0572 */                         span.shrink_to_lo(),
/* FP:suggestions.rs-0573 */                         msg,
/* FP:suggestions.rs-0574 */                         derefs,
/* FP:suggestions.rs-0575 */                         Applicability::MachineApplicable,
/* FP:suggestions.rs-0576 */                     )
/* FP:suggestions.rs-0577 */                 };
/* FP:suggestions.rs-0578 */                 return true;
/* FP:suggestions.rs-0579 */             }
/* FP:suggestions.rs-0580 */         } else if let (
/* FP:suggestions.rs-0581 */             ObligationCauseCode::BinOp { lhs_hir_id, rhs_hir_id, .. },
/* FP:suggestions.rs-0582 */             predicate,
/* FP:suggestions.rs-0583 */         ) = code.peel_derives_with_predicate()
/* FP:suggestions.rs-0584 */             && let Some(typeck_results) = &self.typeck_results
/* FP:suggestions.rs-0585 */             && let hir::Node::Expr(lhs) = self.tcx.hir_node(*lhs_hir_id)
/* FP:suggestions.rs-0586 */             && let hir::Node::Expr(rhs) = self.tcx.hir_node(*rhs_hir_id)
/* FP:suggestions.rs-0587 */             && let Some(rhs_ty) = typeck_results.expr_ty_opt(rhs)
/* FP:suggestions.rs-0588 */             && let trait_pred = predicate.unwrap_or(trait_pred)
/* FP:suggestions.rs-0589 */             // Only run this code on binary operators
/* FP:suggestions.rs-0590 */             && hir::lang_items::BINARY_OPERATORS
/* FP:suggestions.rs-0591 */                 .iter()
/* FP:suggestions.rs-0592 */                 .filter_map(|&op| self.tcx.lang_items().get(op))
/* FP:suggestions.rs-0593 */                 .any(|op| {
/* FP:suggestions.rs-0594 */                     op == trait_pred.skip_binder().trait_ref.def_id
/* FP:suggestions.rs-0595 */                 })
/* FP:suggestions.rs-0596 */         {
/* FP:suggestions.rs-0597 */             // Suggest dereferencing the LHS, RHS, or both terms of a binop if possible
/* FP:suggestions.rs-0598 */ 
/* FP:suggestions.rs-0599 */             let trait_pred = predicate.unwrap_or(trait_pred);
/* FP:suggestions.rs-0600 */             let lhs_ty = self.tcx.instantiate_bound_regions_with_erased(trait_pred.self_ty());
/* FP:suggestions.rs-0601 */             let lhs_autoderef = (self.autoderef_steps)(lhs_ty);
/* FP:suggestions.rs-0602 */             let rhs_autoderef = (self.autoderef_steps)(rhs_ty);
/* FP:suggestions.rs-0603 */             let first_lhs = lhs_autoderef.first().unwrap().clone();
/* FP:suggestions.rs-0604 */             let first_rhs = rhs_autoderef.first().unwrap().clone();
/* FP:suggestions.rs-0605 */             let mut autoderefs = lhs_autoderef
/* FP:suggestions.rs-0606 */                 .into_iter()
/* FP:suggestions.rs-0607 */                 .enumerate()
/* FP:suggestions.rs-0608 */                 .rev()
/* FP:suggestions.rs-0609 */                 .zip_longest(rhs_autoderef.into_iter().enumerate().rev())
/* FP:suggestions.rs-0610 */                 .map(|t| match t {
/* FP:suggestions.rs-0611 */                     EitherOrBoth::Both(a, b) => (a, b),
/* FP:suggestions.rs-0612 */                     EitherOrBoth::Left(a) => (a, (0, first_rhs.clone())),
/* FP:suggestions.rs-0613 */                     EitherOrBoth::Right(b) => ((0, first_lhs.clone()), b),
/* FP:suggestions.rs-0614 */                 })
/* FP:suggestions.rs-0615 */                 .rev();
/* FP:suggestions.rs-0616 */             if let Some((lsteps, rsteps)) =
/* FP:suggestions.rs-0617 */                 autoderefs.find_map(|((lsteps, (l_ty, _)), (rsteps, (r_ty, _)))| {
/* FP:suggestions.rs-0618 */                     // Create a new predicate with the dereferenced LHS and RHS
/* FP:suggestions.rs-0619 */                     // We simultaneously dereference both sides rather than doing them
/* FP:suggestions.rs-0620 */                     // one at a time to account for cases such as &Box<T> == &&T
/* FP:suggestions.rs-0621 */                     let trait_pred_and_ty = trait_pred.map_bound(|inner| {
/* FP:suggestions.rs-0622 */                         (
/* FP:suggestions.rs-0623 */                             ty::TraitPredicate {
/* FP:suggestions.rs-0624 */                                 trait_ref: ty::TraitRef::new_from_args(
/* FP:suggestions.rs-0625 */                                     self.tcx,
/* FP:suggestions.rs-0626 */                                     inner.trait_ref.def_id,
/* FP:suggestions.rs-0627 */                                     self.tcx.mk_args(
/* FP:suggestions.rs-0628 */                                         &[&[l_ty.into(), r_ty.into()], &inner.trait_ref.args[2..]]
/* FP:suggestions.rs-0629 */                                             .concat(),
/* FP:suggestions.rs-0630 */                                     ),
/* FP:suggestions.rs-0631 */                                 ),
/* FP:suggestions.rs-0632 */                                 ..inner
/* FP:suggestions.rs-0633 */                             },
/* FP:suggestions.rs-0634 */                             l_ty,
/* FP:suggestions.rs-0635 */                         )
/* FP:suggestions.rs-0636 */                     });
/* FP:suggestions.rs-0637 */                     let obligation = self.mk_trait_obligation_with_new_self_ty(
/* FP:suggestions.rs-0638 */                         obligation.param_env,
/* FP:suggestions.rs-0639 */                         trait_pred_and_ty,
/* FP:suggestions.rs-0640 */                     );
/* FP:suggestions.rs-0641 */                     self.predicate_may_hold(&obligation).then_some(match (lsteps, rsteps) {
/* FP:suggestions.rs-0642 */                         (_, 0) => (Some(lsteps), None),
/* FP:suggestions.rs-0643 */                         (0, _) => (None, Some(rsteps)),
/* FP:suggestions.rs-0644 */                         _ => (Some(lsteps), Some(rsteps)),
/* FP:suggestions.rs-0645 */                     })
/* FP:suggestions.rs-0646 */                 })
/* FP:suggestions.rs-0647 */             {
/* FP:suggestions.rs-0648 */                 let make_sugg = |mut expr: &Expr<'_>, mut steps| {
/* FP:suggestions.rs-0649 */                     let mut prefix_span = expr.span.shrink_to_lo();
/* FP:suggestions.rs-0650 */                     let mut msg = "consider dereferencing here";
/* FP:suggestions.rs-0651 */                     if let hir::ExprKind::AddrOf(_, _, inner) = expr.kind {
/* FP:suggestions.rs-0652 */                         msg = "consider removing the borrow and dereferencing instead";
/* FP:suggestions.rs-0653 */                         if let hir::ExprKind::AddrOf(..) = inner.kind {
/* FP:suggestions.rs-0654 */                             msg = "consider removing the borrows and dereferencing instead";
/* FP:suggestions.rs-0655 */                         }
/* FP:suggestions.rs-0656 */                     }
/* FP:suggestions.rs-0657 */                     while let hir::ExprKind::AddrOf(_, _, inner) = expr.kind
/* FP:suggestions.rs-0658 */                         && steps > 0
/* FP:suggestions.rs-0659 */                     {
/* FP:suggestions.rs-0660 */                         prefix_span = prefix_span.with_hi(inner.span.lo());
/* FP:suggestions.rs-0661 */                         expr = inner;
/* FP:suggestions.rs-0662 */                         steps -= 1;
/* FP:suggestions.rs-0663 */                     }
/* FP:suggestions.rs-0664 */                     // Empty suggestions with empty spans ICE with debug assertions
/* FP:suggestions.rs-0665 */                     if steps == 0 {
/* FP:suggestions.rs-0666 */                         return (
/* FP:suggestions.rs-0667 */                             msg.trim_end_matches(" and dereferencing instead"),
/* FP:suggestions.rs-0668 */                             vec![(prefix_span, String::new())],
/* FP:suggestions.rs-0669 */                         );
/* FP:suggestions.rs-0670 */                     }
/* FP:suggestions.rs-0671 */                     let derefs = "*".repeat(steps);
/* FP:suggestions.rs-0672 */                     let needs_parens = steps > 0
/* FP:suggestions.rs-0673 */                         && match expr.kind {
/* FP:suggestions.rs-0674 */                             hir::ExprKind::Cast(_, _) | hir::ExprKind::Binary(_, _, _) => true,
/* FP:suggestions.rs-0675 */                             _ if is_range_literal(expr) => true,
/* FP:suggestions.rs-0676 */                             _ => false,
/* FP:suggestions.rs-0677 */                         };
/* FP:suggestions.rs-0678 */                     let mut suggestion = if needs_parens {
/* FP:suggestions.rs-0679 */                         vec![
/* FP:suggestions.rs-0680 */                             (
/* FP:suggestions.rs-0681 */                                 expr.span.with_lo(prefix_span.hi()).shrink_to_lo(),
/* FP:suggestions.rs-0682 */                                 format!("{derefs}("),
/* FP:suggestions.rs-0683 */                             ),
/* FP:suggestions.rs-0684 */                             (expr.span.shrink_to_hi(), ")".to_string()),
/* FP:suggestions.rs-0685 */                         ]
/* FP:suggestions.rs-0686 */                     } else {
/* FP:suggestions.rs-0687 */                         vec![(
/* FP:suggestions.rs-0688 */                             expr.span.with_lo(prefix_span.hi()).shrink_to_lo(),
/* FP:suggestions.rs-0689 */                             format!("{derefs}"),
/* FP:suggestions.rs-0690 */                         )]
/* FP:suggestions.rs-0691 */                     };
/* FP:suggestions.rs-0692 */                     // Empty suggestions with empty spans ICE with debug assertions
/* FP:suggestions.rs-0693 */                     if !prefix_span.is_empty() {
/* FP:suggestions.rs-0694 */                         suggestion.push((prefix_span, String::new()));
/* FP:suggestions.rs-0695 */                     }
/* FP:suggestions.rs-0696 */                     (msg, suggestion)
/* FP:suggestions.rs-0697 */                 };
/* FP:suggestions.rs-0698 */ 
/* FP:suggestions.rs-0699 */                 if let Some(lsteps) = lsteps
/* FP:suggestions.rs-0700 */                     && let Some(rsteps) = rsteps
/* FP:suggestions.rs-0701 */                     && lsteps > 0
/* FP:suggestions.rs-0702 */                     && rsteps > 0
/* FP:suggestions.rs-0703 */                 {
/* FP:suggestions.rs-0704 */                     let mut suggestion = make_sugg(lhs, lsteps).1;
/* FP:suggestions.rs-0705 */                     suggestion.append(&mut make_sugg(rhs, rsteps).1);
/* FP:suggestions.rs-0706 */                     err.multipart_suggestion_verbose(
/* FP:suggestions.rs-0707 */                         "consider dereferencing both sides of the expression",
/* FP:suggestions.rs-0708 */                         suggestion,
/* FP:suggestions.rs-0709 */                         Applicability::MachineApplicable,
/* FP:suggestions.rs-0710 */                     );
/* FP:suggestions.rs-0711 */                     return true;
/* FP:suggestions.rs-0712 */                 } else if let Some(lsteps) = lsteps
/* FP:suggestions.rs-0713 */                     && lsteps > 0
/* FP:suggestions.rs-0714 */                 {
/* FP:suggestions.rs-0715 */                     let (msg, suggestion) = make_sugg(lhs, lsteps);
/* FP:suggestions.rs-0716 */                     err.multipart_suggestion_verbose(
/* FP:suggestions.rs-0717 */                         msg,
/* FP:suggestions.rs-0718 */                         suggestion,
/* FP:suggestions.rs-0719 */                         Applicability::MachineApplicable,
/* FP:suggestions.rs-0720 */                     );
/* FP:suggestions.rs-0721 */                     return true;
/* FP:suggestions.rs-0722 */                 } else if let Some(rsteps) = rsteps
/* FP:suggestions.rs-0723 */                     && rsteps > 0
/* FP:suggestions.rs-0724 */                 {
/* FP:suggestions.rs-0725 */                     let (msg, suggestion) = make_sugg(rhs, rsteps);
/* FP:suggestions.rs-0726 */                     err.multipart_suggestion_verbose(
/* FP:suggestions.rs-0727 */                         msg,
/* FP:suggestions.rs-0728 */                         suggestion,
/* FP:suggestions.rs-0729 */                         Applicability::MachineApplicable,
/* FP:suggestions.rs-0730 */                     );
/* FP:suggestions.rs-0731 */                     return true;
/* FP:suggestions.rs-0732 */                 }
/* FP:suggestions.rs-0733 */             }
/* FP:suggestions.rs-0734 */         }
/* FP:suggestions.rs-0735 */         false
/* FP:suggestions.rs-0736 */     }
/* FP:suggestions.rs-0737 */ 
/* FP:suggestions.rs-0738 */     /// Given a closure's `DefId`, return the given name of the closure.
/* FP:suggestions.rs-0739 */     ///
/* FP:suggestions.rs-0740 */     /// This doesn't account for reassignments, but it's only used for suggestions.
/* FP:suggestions.rs-0741 */     fn get_closure_name(
/* FP:suggestions.rs-0742 */         &self,
/* FP:suggestions.rs-0743 */         def_id: DefId,
/* FP:suggestions.rs-0744 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-0745 */         msg: Cow<'static, str>,
/* FP:suggestions.rs-0746 */     ) -> Option<Symbol> {
/* FP:suggestions.rs-0747 */         let get_name = |err: &mut Diag<'_>, kind: &hir::PatKind<'_>| -> Option<Symbol> {
/* FP:suggestions.rs-0748 */             // Get the local name of this closure. This can be inaccurate because
/* FP:suggestions.rs-0749 */             // of the possibility of reassignment, but this should be good enough.
/* FP:suggestions.rs-0750 */             match &kind {
/* FP:suggestions.rs-0751 */                 hir::PatKind::Binding(hir::BindingMode::NONE, _, ident, None) => Some(ident.name),
/* FP:suggestions.rs-0752 */                 _ => {
/* FP:suggestions.rs-0753 */                     err.note(msg);
/* FP:suggestions.rs-0754 */                     None
/* FP:suggestions.rs-0755 */                 }
/* FP:suggestions.rs-0756 */             }
/* FP:suggestions.rs-0757 */         };
/* FP:suggestions.rs-0758 */ 
/* FP:suggestions.rs-0759 */         let hir_id = self.tcx.local_def_id_to_hir_id(def_id.as_local()?);
/* FP:suggestions.rs-0760 */         match self.tcx.parent_hir_node(hir_id) {
/* FP:suggestions.rs-0761 */             hir::Node::Stmt(hir::Stmt { kind: hir::StmtKind::Let(local), .. }) => {
/* FP:suggestions.rs-0762 */                 get_name(err, &local.pat.kind)
/* FP:suggestions.rs-0763 */             }
/* FP:suggestions.rs-0764 */             // Different to previous arm because one is `&hir::Local` and the other
/* FP:suggestions.rs-0765 */             // is `Box<hir::Local>`.
/* FP:suggestions.rs-0766 */             hir::Node::LetStmt(local) => get_name(err, &local.pat.kind),
/* FP:suggestions.rs-0767 */             _ => None,
/* FP:suggestions.rs-0768 */         }
/* FP:suggestions.rs-0769 */     }
/* FP:suggestions.rs-0770 */ 
/* FP:suggestions.rs-0771 */     /// We tried to apply the bound to an `fn` or closure. Check whether calling it would
/* FP:suggestions.rs-0772 */     /// evaluate to a type that *would* satisfy the trait bound. If it would, suggest calling
/* FP:suggestions.rs-0773 */     /// it: `bar(foo)` → `bar(foo())`. This case is *very* likely to be hit if `foo` is `async`.
/* FP:suggestions.rs-0774 */     pub(super) fn suggest_fn_call(
/* FP:suggestions.rs-0775 */         &self,
/* FP:suggestions.rs-0776 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-0777 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-0778 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-0779 */     ) -> bool {
/* FP:suggestions.rs-0780 */         // It doesn't make sense to make this suggestion outside of typeck...
/* FP:suggestions.rs-0781 */         // (also autoderef will ICE...)
/* FP:suggestions.rs-0782 */         if self.typeck_results.is_none() {
/* FP:suggestions.rs-0783 */             return false;
/* FP:suggestions.rs-0784 */         }
/* FP:suggestions.rs-0785 */ 
/* FP:suggestions.rs-0786 */         if let ty::PredicateKind::Clause(ty::ClauseKind::Trait(trait_pred)) =
/* FP:suggestions.rs-0787 */             obligation.predicate.kind().skip_binder()
/* FP:suggestions.rs-0788 */             && self.tcx.is_lang_item(trait_pred.def_id(), LangItem::Sized)
/* FP:suggestions.rs-0789 */         {
/* FP:suggestions.rs-0790 */             // Don't suggest calling to turn an unsized type into a sized type
/* FP:suggestions.rs-0791 */             return false;
/* FP:suggestions.rs-0792 */         }
/* FP:suggestions.rs-0793 */ 
/* FP:suggestions.rs-0794 */         let self_ty = self.instantiate_binder_with_fresh_vars(
/* FP:suggestions.rs-0795 */             DUMMY_SP,
/* FP:suggestions.rs-0796 */             BoundRegionConversionTime::FnCall,
/* FP:suggestions.rs-0797 */             trait_pred.self_ty(),
/* FP:suggestions.rs-0798 */         );
/* FP:suggestions.rs-0799 */ 
/* FP:suggestions.rs-0800 */         let Some((def_id_or_name, output, inputs)) =
/* FP:suggestions.rs-0801 */             self.extract_callable_info(obligation.cause.body_id, obligation.param_env, self_ty)
/* FP:suggestions.rs-0802 */         else {
/* FP:suggestions.rs-0803 */             return false;
/* FP:suggestions.rs-0804 */         };
/* FP:suggestions.rs-0805 */ 
/* FP:suggestions.rs-0806 */         // Remapping bound vars here
/* FP:suggestions.rs-0807 */         let trait_pred_and_self = trait_pred.map_bound(|trait_pred| (trait_pred, output));
/* FP:suggestions.rs-0808 */ 
/* FP:suggestions.rs-0809 */         let new_obligation =
/* FP:suggestions.rs-0810 */             self.mk_trait_obligation_with_new_self_ty(obligation.param_env, trait_pred_and_self);
/* FP:suggestions.rs-0811 */         if !self.predicate_must_hold_modulo_regions(&new_obligation) {
/* FP:suggestions.rs-0812 */             return false;
/* FP:suggestions.rs-0813 */         }
/* FP:suggestions.rs-0814 */ 
/* FP:suggestions.rs-0815 */         // Get the name of the callable and the arguments to be used in the suggestion.
/* FP:suggestions.rs-0816 */         let msg = match def_id_or_name {
/* FP:suggestions.rs-0817 */             DefIdOrName::DefId(def_id) => match self.tcx.def_kind(def_id) {
/* FP:suggestions.rs-0818 */                 DefKind::Ctor(CtorOf::Struct, _) => {
/* FP:suggestions.rs-0819 */                     Cow::from("use parentheses to construct this tuple struct")
/* FP:suggestions.rs-0820 */                 }
/* FP:suggestions.rs-0821 */                 DefKind::Ctor(CtorOf::Variant, _) => {
/* FP:suggestions.rs-0822 */                     Cow::from("use parentheses to construct this tuple variant")
/* FP:suggestions.rs-0823 */                 }
/* FP:suggestions.rs-0824 */                 kind => Cow::from(format!(
/* FP:suggestions.rs-0825 */                     "use parentheses to call this {}",
/* FP:suggestions.rs-0826 */                     self.tcx.def_kind_descr(kind, def_id)
/* FP:suggestions.rs-0827 */                 )),
/* FP:suggestions.rs-0828 */             },
/* FP:suggestions.rs-0829 */             DefIdOrName::Name(name) => Cow::from(format!("use parentheses to call this {name}")),
/* FP:suggestions.rs-0830 */         };
/* FP:suggestions.rs-0831 */ 
/* FP:suggestions.rs-0832 */         let args = inputs
/* FP:suggestions.rs-0833 */             .into_iter()
/* FP:suggestions.rs-0834 */             .map(|ty| {
/* FP:suggestions.rs-0835 */                 if ty.is_suggestable(self.tcx, false) {
/* FP:suggestions.rs-0836 */                     format!("/* {ty} */")
/* FP:suggestions.rs-0837 */                 } else {
/* FP:suggestions.rs-0838 */                     "/* value */".to_string()
/* FP:suggestions.rs-0839 */                 }
/* FP:suggestions.rs-0840 */             })
/* FP:suggestions.rs-0841 */             .collect::<Vec<_>>()
/* FP:suggestions.rs-0842 */             .join(", ");
/* FP:suggestions.rs-0843 */ 
/* FP:suggestions.rs-0844 */         if matches!(obligation.cause.code(), ObligationCauseCode::FunctionArg { .. })
/* FP:suggestions.rs-0845 */             && obligation.cause.span.can_be_used_for_suggestions()
/* FP:suggestions.rs-0846 */         {
/* FP:suggestions.rs-0847 */             let (span, sugg) = if let Some(snippet) =
/* FP:suggestions.rs-0848 */                 self.tcx.sess.source_map().span_to_snippet(obligation.cause.span).ok()
/* FP:suggestions.rs-0849 */                 && snippet.starts_with("|")
/* FP:suggestions.rs-0850 */             {
/* FP:suggestions.rs-0851 */                 (obligation.cause.span, format!("({snippet})({args})"))
/* FP:suggestions.rs-0852 */             } else {
/* FP:suggestions.rs-0853 */                 (obligation.cause.span.shrink_to_hi(), format!("({args})"))
/* FP:suggestions.rs-0854 */             };
/* FP:suggestions.rs-0855 */ 
/* FP:suggestions.rs-0856 */             // When the obligation error has been ensured to have been caused by
/* FP:suggestions.rs-0857 */             // an argument, the `obligation.cause.span` points at the expression
/* FP:suggestions.rs-0858 */             // of the argument, so we can provide a suggestion. Otherwise, we give
/* FP:suggestions.rs-0859 */             // a more general note.
/* FP:suggestions.rs-0860 */             err.span_suggestion_verbose(span, msg, sugg, Applicability::HasPlaceholders);
/* FP:suggestions.rs-0861 */         } else if let DefIdOrName::DefId(def_id) = def_id_or_name {
/* FP:suggestions.rs-0862 */             let name = match self.tcx.hir_get_if_local(def_id) {
/* FP:suggestions.rs-0863 */                 Some(hir::Node::Expr(hir::Expr {
/* FP:suggestions.rs-0864 */                     kind: hir::ExprKind::Closure(hir::Closure { fn_decl_span, .. }),
/* FP:suggestions.rs-0865 */                     ..
/* FP:suggestions.rs-0866 */                 })) => {
/* FP:suggestions.rs-0867 */                     err.span_label(*fn_decl_span, "consider calling this closure");
/* FP:suggestions.rs-0868 */                     let Some(name) = self.get_closure_name(def_id, err, msg.clone()) else {
/* FP:suggestions.rs-0869 */                         return false;
/* FP:suggestions.rs-0870 */                     };
/* FP:suggestions.rs-0871 */                     name.to_string()
/* FP:suggestions.rs-0872 */                 }
/* FP:suggestions.rs-0873 */                 Some(hir::Node::Item(hir::Item {
/* FP:suggestions.rs-0874 */                     kind: hir::ItemKind::Fn { ident, .. }, ..
/* FP:suggestions.rs-0875 */                 })) => {
/* FP:suggestions.rs-0876 */                     err.span_label(ident.span, "consider calling this function");
/* FP:suggestions.rs-0877 */                     ident.to_string()
/* FP:suggestions.rs-0878 */                 }
/* FP:suggestions.rs-0879 */                 Some(hir::Node::Ctor(..)) => {
/* FP:suggestions.rs-0880 */                     let name = self.tcx.def_path_str(def_id);
/* FP:suggestions.rs-0881 */                     err.span_label(
/* FP:suggestions.rs-0882 */                         self.tcx.def_span(def_id),
/* FP:suggestions.rs-0883 */                         format!("consider calling the constructor for `{name}`"),
/* FP:suggestions.rs-0884 */                     );
/* FP:suggestions.rs-0885 */                     name
/* FP:suggestions.rs-0886 */                 }
/* FP:suggestions.rs-0887 */                 _ => return false,
/* FP:suggestions.rs-0888 */             };
/* FP:suggestions.rs-0889 */             err.help(format!("{msg}: `{name}({args})`"));
/* FP:suggestions.rs-0890 */         }
/* FP:suggestions.rs-0891 */         true
/* FP:suggestions.rs-0892 */     }
/* FP:suggestions.rs-0893 */ 
/* FP:suggestions.rs-0894 */     pub(super) fn check_for_binding_assigned_block_without_tail_expression(
/* FP:suggestions.rs-0895 */         &self,
/* FP:suggestions.rs-0896 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-0897 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-0898 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-0899 */     ) {
/* FP:suggestions.rs-0900 */         let mut span = obligation.cause.span;
/* FP:suggestions.rs-0901 */         while span.from_expansion() {
/* FP:suggestions.rs-0902 */             // Remove all the desugaring and macro contexts.
/* FP:suggestions.rs-0903 */             span.remove_mark();
/* FP:suggestions.rs-0904 */         }
/* FP:suggestions.rs-0905 */         let mut expr_finder = FindExprBySpan::new(span, self.tcx);
/* FP:suggestions.rs-0906 */         let Some(body) = self.tcx.hir_maybe_body_owned_by(obligation.cause.body_id) else {
/* FP:suggestions.rs-0907 */             return;
/* FP:suggestions.rs-0908 */         };
/* FP:suggestions.rs-0909 */         expr_finder.visit_expr(body.value);
/* FP:suggestions.rs-0910 */         let Some(expr) = expr_finder.result else {
/* FP:suggestions.rs-0911 */             return;
/* FP:suggestions.rs-0912 */         };
/* FP:suggestions.rs-0913 */         let Some(typeck) = &self.typeck_results else {
/* FP:suggestions.rs-0914 */             return;
/* FP:suggestions.rs-0915 */         };
/* FP:suggestions.rs-0916 */         let Some(ty) = typeck.expr_ty_adjusted_opt(expr) else {
/* FP:suggestions.rs-0917 */             return;
/* FP:suggestions.rs-0918 */         };
/* FP:suggestions.rs-0919 */         if !ty.is_unit() {
/* FP:suggestions.rs-0920 */             return;
/* FP:suggestions.rs-0921 */         };
/* FP:suggestions.rs-0922 */         let hir::ExprKind::Path(hir::QPath::Resolved(None, path)) = expr.kind else {
/* FP:suggestions.rs-0923 */             return;
/* FP:suggestions.rs-0924 */         };
/* FP:suggestions.rs-0925 */         let Res::Local(hir_id) = path.res else {
/* FP:suggestions.rs-0926 */             return;
/* FP:suggestions.rs-0927 */         };
/* FP:suggestions.rs-0928 */         let hir::Node::Pat(pat) = self.tcx.hir_node(hir_id) else {
/* FP:suggestions.rs-0929 */             return;
/* FP:suggestions.rs-0930 */         };
/* FP:suggestions.rs-0931 */         let hir::Node::LetStmt(hir::LetStmt { ty: None, init: Some(init), .. }) =
/* FP:suggestions.rs-0932 */             self.tcx.parent_hir_node(pat.hir_id)
/* FP:suggestions.rs-0933 */         else {
/* FP:suggestions.rs-0934 */             return;
/* FP:suggestions.rs-0935 */         };
/* FP:suggestions.rs-0936 */         let hir::ExprKind::Block(block, None) = init.kind else {
/* FP:suggestions.rs-0937 */             return;
/* FP:suggestions.rs-0938 */         };
/* FP:suggestions.rs-0939 */         if block.expr.is_some() {
/* FP:suggestions.rs-0940 */             return;
/* FP:suggestions.rs-0941 */         }
/* FP:suggestions.rs-0942 */         let [.., stmt] = block.stmts else {
/* FP:suggestions.rs-0943 */             err.span_label(block.span, "this empty block is missing a tail expression");
/* FP:suggestions.rs-0944 */             return;
/* FP:suggestions.rs-0945 */         };
/* FP:suggestions.rs-0946 */         let hir::StmtKind::Semi(tail_expr) = stmt.kind else {
/* FP:suggestions.rs-0947 */             return;
/* FP:suggestions.rs-0948 */         };
/* FP:suggestions.rs-0949 */         let Some(ty) = typeck.expr_ty_opt(tail_expr) else {
/* FP:suggestions.rs-0950 */             err.span_label(block.span, "this block is missing a tail expression");
/* FP:suggestions.rs-0951 */             return;
/* FP:suggestions.rs-0952 */         };
/* FP:suggestions.rs-0953 */         let ty = self.resolve_numeric_literals_with_default(self.resolve_vars_if_possible(ty));
/* FP:suggestions.rs-0954 */         let trait_pred_and_self = trait_pred.map_bound(|trait_pred| (trait_pred, ty));
/* FP:suggestions.rs-0955 */ 
/* FP:suggestions.rs-0956 */         let new_obligation =
/* FP:suggestions.rs-0957 */             self.mk_trait_obligation_with_new_self_ty(obligation.param_env, trait_pred_and_self);
/* FP:suggestions.rs-0958 */         if self.predicate_must_hold_modulo_regions(&new_obligation) {
/* FP:suggestions.rs-0959 */             err.span_suggestion_short(
/* FP:suggestions.rs-0960 */                 stmt.span.with_lo(tail_expr.span.hi()),
/* FP:suggestions.rs-0961 */                 "remove this semicolon",
/* FP:suggestions.rs-0962 */                 "",
/* FP:suggestions.rs-0963 */                 Applicability::MachineApplicable,
/* FP:suggestions.rs-0964 */             );
/* FP:suggestions.rs-0965 */         } else {
/* FP:suggestions.rs-0966 */             err.span_label(block.span, "this block is missing a tail expression");
/* FP:suggestions.rs-0967 */         }
/* FP:suggestions.rs-0968 */     }
/* FP:suggestions.rs-0969 */ 
/* FP:suggestions.rs-0970 */     pub(super) fn suggest_add_clone_to_arg(
/* FP:suggestions.rs-0971 */         &self,
/* FP:suggestions.rs-0972 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-0973 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-0974 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-0975 */     ) -> bool {
/* FP:suggestions.rs-0976 */         let self_ty = self.resolve_vars_if_possible(trait_pred.self_ty());
/* FP:suggestions.rs-0977 */         self.enter_forall(self_ty, |ty: Ty<'_>| {
/* FP:suggestions.rs-0978 */             let Some(generics) = self.tcx.hir_get_generics(obligation.cause.body_id) else {
/* FP:suggestions.rs-0979 */                 return false;
/* FP:suggestions.rs-0980 */             };
/* FP:suggestions.rs-0981 */             let ty::Ref(_, inner_ty, hir::Mutability::Not) = ty.kind() else { return false };
/* FP:suggestions.rs-0982 */             let ty::Param(param) = inner_ty.kind() else { return false };
/* FP:suggestions.rs-0983 */             let ObligationCauseCode::FunctionArg { arg_hir_id, .. } = obligation.cause.code()
/* FP:suggestions.rs-0984 */             else {
/* FP:suggestions.rs-0985 */                 return false;
/* FP:suggestions.rs-0986 */             };
/* FP:suggestions.rs-0987 */ 
/* FP:suggestions.rs-0988 */             let clone_trait = self.tcx.require_lang_item(LangItem::Clone, obligation.cause.span);
/* FP:suggestions.rs-0989 */             let has_clone = |ty| {
/* FP:suggestions.rs-0990 */                 self.type_implements_trait(clone_trait, [ty], obligation.param_env)
/* FP:suggestions.rs-0991 */                     .must_apply_modulo_regions()
/* FP:suggestions.rs-0992 */             };
/* FP:suggestions.rs-0993 */ 
/* FP:suggestions.rs-0994 */             let existing_clone_call = match self.tcx.hir_node(*arg_hir_id) {
/* FP:suggestions.rs-0995 */                 // It's just a variable. Propose cloning it.
/* FP:suggestions.rs-0996 */                 Node::Expr(Expr { kind: hir::ExprKind::Path(_), .. }) => None,
/* FP:suggestions.rs-0997 */                 // It's already a call to `clone()`. We might be able to suggest
/* FP:suggestions.rs-0998 */                 // adding a `+ Clone` bound, though.
/* FP:suggestions.rs-0999 */                 Node::Expr(Expr {
/* FP:suggestions.rs-1000 */                     kind:
/* FP:suggestions.rs-1001 */                         hir::ExprKind::MethodCall(
/* FP:suggestions.rs-1002 */                             hir::PathSegment { ident, .. },
/* FP:suggestions.rs-1003 */                             _receiver,
/* FP:suggestions.rs-1004 */                             [],
/* FP:suggestions.rs-1005 */                             call_span,
/* FP:suggestions.rs-1006 */                         ),
/* FP:suggestions.rs-1007 */                     hir_id,
/* FP:suggestions.rs-1008 */                     ..
/* FP:suggestions.rs-1009 */                 }) if ident.name == sym::clone
/* FP:suggestions.rs-1010 */                     && !call_span.from_expansion()
/* FP:suggestions.rs-1011 */                     && !has_clone(*inner_ty) =>
/* FP:suggestions.rs-1012 */                 {
/* FP:suggestions.rs-1013 */                     // We only care about method calls corresponding to the real `Clone` trait.
/* FP:suggestions.rs-1014 */                     let Some(typeck_results) = self.typeck_results.as_ref() else { return false };
/* FP:suggestions.rs-1015 */                     let Some((DefKind::AssocFn, did)) = typeck_results.type_dependent_def(*hir_id)
/* FP:suggestions.rs-1016 */                     else {
/* FP:suggestions.rs-1017 */                         return false;
/* FP:suggestions.rs-1018 */                     };
/* FP:suggestions.rs-1019 */                     if self.tcx.trait_of_assoc(did) != Some(clone_trait) {
/* FP:suggestions.rs-1020 */                         return false;
/* FP:suggestions.rs-1021 */                     }
/* FP:suggestions.rs-1022 */                     Some(ident.span)
/* FP:suggestions.rs-1023 */                 }
/* FP:suggestions.rs-1024 */                 _ => return false,
/* FP:suggestions.rs-1025 */             };
/* FP:suggestions.rs-1026 */ 
/* FP:suggestions.rs-1027 */             let new_obligation = self.mk_trait_obligation_with_new_self_ty(
/* FP:suggestions.rs-1028 */                 obligation.param_env,
/* FP:suggestions.rs-1029 */                 trait_pred.map_bound(|trait_pred| (trait_pred, *inner_ty)),
/* FP:suggestions.rs-1030 */             );
/* FP:suggestions.rs-1031 */ 
/* FP:suggestions.rs-1032 */             if self.predicate_may_hold(&new_obligation) && has_clone(ty) {
/* FP:suggestions.rs-1033 */                 if !has_clone(param.to_ty(self.tcx)) {
/* FP:suggestions.rs-1034 */                     suggest_constraining_type_param(
/* FP:suggestions.rs-1035 */                         self.tcx,
/* FP:suggestions.rs-1036 */                         generics,
/* FP:suggestions.rs-1037 */                         err,
/* FP:suggestions.rs-1038 */                         param.name.as_str(),
/* FP:suggestions.rs-1039 */                         "Clone",
/* FP:suggestions.rs-1040 */                         Some(clone_trait),
/* FP:suggestions.rs-1041 */                         None,
/* FP:suggestions.rs-1042 */                     );
/* FP:suggestions.rs-1043 */                 }
/* FP:suggestions.rs-1044 */                 if let Some(existing_clone_call) = existing_clone_call {
/* FP:suggestions.rs-1045 */                     err.span_note(
/* FP:suggestions.rs-1046 */                         existing_clone_call,
/* FP:suggestions.rs-1047 */                         format!(
/* FP:suggestions.rs-1048 */                             "this `clone()` copies the reference, \
/* FP:suggestions.rs-1049 */                             which does not do anything, \
/* FP:suggestions.rs-1050 */                             because `{inner_ty}` does not implement `Clone`"
/* FP:suggestions.rs-1051 */                         ),
/* FP:suggestions.rs-1052 */                     );
/* FP:suggestions.rs-1053 */                 } else {
/* FP:suggestions.rs-1054 */                     err.span_suggestion_verbose(
/* FP:suggestions.rs-1055 */                         obligation.cause.span.shrink_to_hi(),
/* FP:suggestions.rs-1056 */                         "consider using clone here",
/* FP:suggestions.rs-1057 */                         ".clone()".to_string(),
/* FP:suggestions.rs-1058 */                         Applicability::MaybeIncorrect,
/* FP:suggestions.rs-1059 */                     );
/* FP:suggestions.rs-1060 */                 }
/* FP:suggestions.rs-1061 */                 return true;
/* FP:suggestions.rs-1062 */             }
/* FP:suggestions.rs-1063 */             false
/* FP:suggestions.rs-1064 */         })
/* FP:suggestions.rs-1065 */     }
/* FP:suggestions.rs-1066 */ 
/* FP:suggestions.rs-1067 */     /// Extracts information about a callable type for diagnostics. This is a
/* FP:suggestions.rs-1068 */     /// heuristic -- it doesn't necessarily mean that a type is always callable,
/* FP:suggestions.rs-1069 */     /// because the callable type must also be well-formed to be called.
/* FP:suggestions.rs-1070 */     pub fn extract_callable_info(
/* FP:suggestions.rs-1071 */         &self,
/* FP:suggestions.rs-1072 */         body_id: LocalDefId,
/* FP:suggestions.rs-1073 */         param_env: ty::ParamEnv<'tcx>,
/* FP:suggestions.rs-1074 */         found: Ty<'tcx>,
/* FP:suggestions.rs-1075 */     ) -> Option<(DefIdOrName, Ty<'tcx>, Vec<Ty<'tcx>>)> {
/* FP:suggestions.rs-1076 */         // Autoderef is useful here because sometimes we box callables, etc.
/* FP:suggestions.rs-1077 */         let Some((def_id_or_name, output, inputs)) =
/* FP:suggestions.rs-1078 */             (self.autoderef_steps)(found).into_iter().find_map(|(found, _)| match *found.kind() {
/* FP:suggestions.rs-1079 */                 ty::FnPtr(sig_tys, _) => Some((
/* FP:suggestions.rs-1080 */                     DefIdOrName::Name("function pointer"),
/* FP:suggestions.rs-1081 */                     sig_tys.output(),
/* FP:suggestions.rs-1082 */                     sig_tys.inputs(),
/* FP:suggestions.rs-1083 */                 )),
/* FP:suggestions.rs-1084 */                 ty::FnDef(def_id, _) => {
/* FP:suggestions.rs-1085 */                     let fn_sig = found.fn_sig(self.tcx);
/* FP:suggestions.rs-1086 */                     Some((DefIdOrName::DefId(def_id), fn_sig.output(), fn_sig.inputs()))
/* FP:suggestions.rs-1087 */                 }
/* FP:suggestions.rs-1088 */                 ty::Closure(def_id, args) => {
/* FP:suggestions.rs-1089 */                     let fn_sig = args.as_closure().sig();
/* FP:suggestions.rs-1090 */                     Some((
/* FP:suggestions.rs-1091 */                         DefIdOrName::DefId(def_id),
/* FP:suggestions.rs-1092 */                         fn_sig.output(),
/* FP:suggestions.rs-1093 */                         fn_sig.inputs().map_bound(|inputs| inputs[0].tuple_fields().as_slice()),
/* FP:suggestions.rs-1094 */                     ))
/* FP:suggestions.rs-1095 */                 }
/* FP:suggestions.rs-1096 */                 ty::CoroutineClosure(def_id, args) => {
/* FP:suggestions.rs-1097 */                     let sig_parts = args.as_coroutine_closure().coroutine_closure_sig();
/* FP:suggestions.rs-1098 */                     Some((
/* FP:suggestions.rs-1099 */                         DefIdOrName::DefId(def_id),
/* FP:suggestions.rs-1100 */                         sig_parts.map_bound(|sig| {
/* FP:suggestions.rs-1101 */                             sig.to_coroutine(
/* FP:suggestions.rs-1102 */                                 self.tcx,
/* FP:suggestions.rs-1103 */                                 args.as_coroutine_closure().parent_args(),
/* FP:suggestions.rs-1104 */                                 // Just use infer vars here, since we  don't really care
/* FP:suggestions.rs-1105 */                                 // what these types are, just that we're returning a coroutine.
/* FP:suggestions.rs-1106 */                                 self.next_ty_var(DUMMY_SP),
/* FP:suggestions.rs-1107 */                                 self.tcx.coroutine_for_closure(def_id),
/* FP:suggestions.rs-1108 */                                 self.next_ty_var(DUMMY_SP),
/* FP:suggestions.rs-1109 */                             )
/* FP:suggestions.rs-1110 */                         }),
/* FP:suggestions.rs-1111 */                         sig_parts.map_bound(|sig| sig.tupled_inputs_ty.tuple_fields().as_slice()),
/* FP:suggestions.rs-1112 */                     ))
/* FP:suggestions.rs-1113 */                 }
/* FP:suggestions.rs-1114 */                 ty::Alias(ty::Opaque, ty::AliasTy { def_id, args, .. }) => {
/* FP:suggestions.rs-1115 */                     self.tcx.item_self_bounds(def_id).instantiate(self.tcx, args).iter().find_map(
/* FP:suggestions.rs-1116 */                         |pred| {
/* FP:suggestions.rs-1117 */                             if let ty::ClauseKind::Projection(proj) = pred.kind().skip_binder()
/* FP:suggestions.rs-1118 */                             && self
/* FP:suggestions.rs-1119 */                                 .tcx
/* FP:suggestions.rs-1120 */                                 .is_lang_item(proj.projection_term.def_id, LangItem::FnOnceOutput)
/* FP:suggestions.rs-1121 */                             // args tuple will always be args[1]
/* FP:suggestions.rs-1122 */                             && let ty::Tuple(args) = proj.projection_term.args.type_at(1).kind()
/* FP:suggestions.rs-1123 */                             {
/* FP:suggestions.rs-1124 */                                 Some((
/* FP:suggestions.rs-1125 */                                     DefIdOrName::DefId(def_id),
/* FP:suggestions.rs-1126 */                                     pred.kind().rebind(proj.term.expect_type()),
/* FP:suggestions.rs-1127 */                                     pred.kind().rebind(args.as_slice()),
/* FP:suggestions.rs-1128 */                                 ))
/* FP:suggestions.rs-1129 */                             } else {
/* FP:suggestions.rs-1130 */                                 None
/* FP:suggestions.rs-1131 */                             }
/* FP:suggestions.rs-1132 */                         },
/* FP:suggestions.rs-1133 */                     )
/* FP:suggestions.rs-1134 */                 }
/* FP:suggestions.rs-1135 */                 ty::Dynamic(data, _, ty::Dyn) => data.iter().find_map(|pred| {
/* FP:suggestions.rs-1136 */                     if let ty::ExistentialPredicate::Projection(proj) = pred.skip_binder()
/* FP:suggestions.rs-1137 */                         && self.tcx.is_lang_item(proj.def_id, LangItem::FnOnceOutput)
/* FP:suggestions.rs-1138 */                         // for existential projection, args are shifted over by 1
/* FP:suggestions.rs-1139 */                         && let ty::Tuple(args) = proj.args.type_at(0).kind()
/* FP:suggestions.rs-1140 */                     {
/* FP:suggestions.rs-1141 */                         Some((
/* FP:suggestions.rs-1142 */                             DefIdOrName::Name("trait object"),
/* FP:suggestions.rs-1143 */                             pred.rebind(proj.term.expect_type()),
/* FP:suggestions.rs-1144 */                             pred.rebind(args.as_slice()),
/* FP:suggestions.rs-1145 */                         ))
/* FP:suggestions.rs-1146 */                     } else {
/* FP:suggestions.rs-1147 */                         None
/* FP:suggestions.rs-1148 */                     }
/* FP:suggestions.rs-1149 */                 }),
/* FP:suggestions.rs-1150 */                 ty::Param(param) => {
/* FP:suggestions.rs-1151 */                     let generics = self.tcx.generics_of(body_id);
/* FP:suggestions.rs-1152 */                     let name = if generics.count() > param.index as usize
/* FP:suggestions.rs-1153 */                         && let def = generics.param_at(param.index as usize, self.tcx)
/* FP:suggestions.rs-1154 */                         && matches!(def.kind, ty::GenericParamDefKind::Type { .. })
/* FP:suggestions.rs-1155 */                         && def.name == param.name
/* FP:suggestions.rs-1156 */                     {
/* FP:suggestions.rs-1157 */                         DefIdOrName::DefId(def.def_id)
/* FP:suggestions.rs-1158 */                     } else {
/* FP:suggestions.rs-1159 */                         DefIdOrName::Name("type parameter")
/* FP:suggestions.rs-1160 */                     };
/* FP:suggestions.rs-1161 */                     param_env.caller_bounds().iter().find_map(|pred| {
/* FP:suggestions.rs-1162 */                         if let ty::ClauseKind::Projection(proj) = pred.kind().skip_binder()
/* FP:suggestions.rs-1163 */                             && self
/* FP:suggestions.rs-1164 */                                 .tcx
/* FP:suggestions.rs-1165 */                                 .is_lang_item(proj.projection_term.def_id, LangItem::FnOnceOutput)
/* FP:suggestions.rs-1166 */                             && proj.projection_term.self_ty() == found
/* FP:suggestions.rs-1167 */                             // args tuple will always be args[1]
/* FP:suggestions.rs-1168 */                             && let ty::Tuple(args) = proj.projection_term.args.type_at(1).kind()
/* FP:suggestions.rs-1169 */                         {
/* FP:suggestions.rs-1170 */                             Some((
/* FP:suggestions.rs-1171 */                                 name,
/* FP:suggestions.rs-1172 */                                 pred.kind().rebind(proj.term.expect_type()),
/* FP:suggestions.rs-1173 */                                 pred.kind().rebind(args.as_slice()),
/* FP:suggestions.rs-1174 */                             ))
/* FP:suggestions.rs-1175 */                         } else {
/* FP:suggestions.rs-1176 */                             None
/* FP:suggestions.rs-1177 */                         }
/* FP:suggestions.rs-1178 */                     })
/* FP:suggestions.rs-1179 */                 }
/* FP:suggestions.rs-1180 */                 _ => None,
/* FP:suggestions.rs-1181 */             })
/* FP:suggestions.rs-1182 */         else {
/* FP:suggestions.rs-1183 */             return None;
/* FP:suggestions.rs-1184 */         };
/* FP:suggestions.rs-1185 */ 
/* FP:suggestions.rs-1186 */         let output = self.instantiate_binder_with_fresh_vars(
/* FP:suggestions.rs-1187 */             DUMMY_SP,
/* FP:suggestions.rs-1188 */             BoundRegionConversionTime::FnCall,
/* FP:suggestions.rs-1189 */             output,
/* FP:suggestions.rs-1190 */         );
/* FP:suggestions.rs-1191 */         let inputs = inputs
/* FP:suggestions.rs-1192 */             .skip_binder()
/* FP:suggestions.rs-1193 */             .iter()
/* FP:suggestions.rs-1194 */             .map(|ty| {
/* FP:suggestions.rs-1195 */                 self.instantiate_binder_with_fresh_vars(
/* FP:suggestions.rs-1196 */                     DUMMY_SP,
/* FP:suggestions.rs-1197 */                     BoundRegionConversionTime::FnCall,
/* FP:suggestions.rs-1198 */                     inputs.rebind(*ty),
/* FP:suggestions.rs-1199 */                 )
/* FP:suggestions.rs-1200 */             })
/* FP:suggestions.rs-1201 */             .collect();
/* FP:suggestions.rs-1202 */ 
/* FP:suggestions.rs-1203 */         // We don't want to register any extra obligations, which should be
/* FP:suggestions.rs-1204 */         // implied by wf, but also because that would possibly result in
/* FP:suggestions.rs-1205 */         // erroneous errors later on.
/* FP:suggestions.rs-1206 */         let InferOk { value: output, obligations: _ } =
/* FP:suggestions.rs-1207 */             self.at(&ObligationCause::dummy(), param_env).normalize(output);
/* FP:suggestions.rs-1208 */ 
/* FP:suggestions.rs-1209 */         if output.is_ty_var() { None } else { Some((def_id_or_name, output, inputs)) }
/* FP:suggestions.rs-1210 */     }
/* FP:suggestions.rs-1211 */ 
/* FP:suggestions.rs-1212 */     pub(super) fn suggest_add_reference_to_arg(
/* FP:suggestions.rs-1213 */         &self,
/* FP:suggestions.rs-1214 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-1215 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-1216 */         poly_trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-1217 */         has_custom_message: bool,
/* FP:suggestions.rs-1218 */     ) -> bool {
/* FP:suggestions.rs-1219 */         let span = obligation.cause.span;
/* FP:suggestions.rs-1220 */         let param_env = obligation.param_env;
/* FP:suggestions.rs-1221 */ 
/* FP:suggestions.rs-1222 */         let mk_result = |trait_pred_and_new_ty| {
/* FP:suggestions.rs-1223 */             let obligation =
/* FP:suggestions.rs-1224 */                 self.mk_trait_obligation_with_new_self_ty(param_env, trait_pred_and_new_ty);
/* FP:suggestions.rs-1225 */             self.predicate_must_hold_modulo_regions(&obligation)
/* FP:suggestions.rs-1226 */         };
/* FP:suggestions.rs-1227 */ 
/* FP:suggestions.rs-1228 */         let code = match obligation.cause.code() {
/* FP:suggestions.rs-1229 */             ObligationCauseCode::FunctionArg { parent_code, .. } => parent_code,
/* FP:suggestions.rs-1230 */             // FIXME(compiler-errors): This is kind of a mess, but required for obligations
/* FP:suggestions.rs-1231 */             // that come from a path expr to affect the *call* expr.
/* FP:suggestions.rs-1232 */             c @ ObligationCauseCode::WhereClauseInExpr(_, _, hir_id, _)
/* FP:suggestions.rs-1233 */                 if self.tcx.hir_span(*hir_id).lo() == span.lo() =>
/* FP:suggestions.rs-1234 */             {
/* FP:suggestions.rs-1235 */                 // `hir_id` corresponds to the HIR node that introduced a `where`-clause obligation.
/* FP:suggestions.rs-1236 */                 // If that obligation comes from a type in an associated method call, we need
/* FP:suggestions.rs-1237 */                 // special handling here.
/* FP:suggestions.rs-1238 */                 if let hir::Node::Expr(expr) = self.tcx.parent_hir_node(*hir_id)
/* FP:suggestions.rs-1239 */                     && let hir::ExprKind::Call(base, _) = expr.kind
/* FP:suggestions.rs-1240 */                     && let hir::ExprKind::Path(hir::QPath::TypeRelative(ty, segment)) = base.kind
/* FP:suggestions.rs-1241 */                     && let hir::Node::Expr(outer) = self.tcx.parent_hir_node(expr.hir_id)
/* FP:suggestions.rs-1242 */                     && let hir::ExprKind::AddrOf(hir::BorrowKind::Ref, mtbl, _) = outer.kind
/* FP:suggestions.rs-1243 */                     && ty.span == span
/* FP:suggestions.rs-1244 */                 {
/* FP:suggestions.rs-1245 */                     // We've encountered something like `&str::from("")`, where the intended code
/* FP:suggestions.rs-1246 */                     // was likely `<&str>::from("")`. The former is interpreted as "call method
/* FP:suggestions.rs-1247 */                     // `from` on `str` and borrow the result", while the latter means "call method
/* FP:suggestions.rs-1248 */                     // `from` on `&str`".
/* FP:suggestions.rs-1249 */ 
/* FP:suggestions.rs-1250 */                     let trait_pred_and_imm_ref = poly_trait_pred.map_bound(|p| {
/* FP:suggestions.rs-1251 */                         (p, Ty::new_imm_ref(self.tcx, self.tcx.lifetimes.re_static, p.self_ty()))
/* FP:suggestions.rs-1252 */                     });
/* FP:suggestions.rs-1253 */                     let trait_pred_and_mut_ref = poly_trait_pred.map_bound(|p| {
/* FP:suggestions.rs-1254 */                         (p, Ty::new_mut_ref(self.tcx, self.tcx.lifetimes.re_static, p.self_ty()))
/* FP:suggestions.rs-1255 */                     });
/* FP:suggestions.rs-1256 */ 
/* FP:suggestions.rs-1257 */                     let imm_ref_self_ty_satisfies_pred = mk_result(trait_pred_and_imm_ref);
/* FP:suggestions.rs-1258 */                     let mut_ref_self_ty_satisfies_pred = mk_result(trait_pred_and_mut_ref);
/* FP:suggestions.rs-1259 */                     let sugg_msg = |pre: &str| {
/* FP:suggestions.rs-1260 */                         format!(
/* FP:suggestions.rs-1261 */                             "you likely meant to call the associated function `{FN}` for type \
/* FP:suggestions.rs-1262 */                              `&{pre}{TY}`, but the code as written calls associated function `{FN}` on \
/* FP:suggestions.rs-1263 */                              type `{TY}`",
/* FP:suggestions.rs-1264 */                             FN = segment.ident,
/* FP:suggestions.rs-1265 */                             TY = poly_trait_pred.self_ty(),
/* FP:suggestions.rs-1266 */                         )
/* FP:suggestions.rs-1267 */                     };
/* FP:suggestions.rs-1268 */                     match (imm_ref_self_ty_satisfies_pred, mut_ref_self_ty_satisfies_pred, mtbl) {
/* FP:suggestions.rs-1269 */                         (true, _, hir::Mutability::Not) | (_, true, hir::Mutability::Mut) => {
/* FP:suggestions.rs-1270 */                             err.multipart_suggestion_verbose(
/* FP:suggestions.rs-1271 */                                 sugg_msg(mtbl.prefix_str()),
/* FP:suggestions.rs-1272 */                                 vec![
/* FP:suggestions.rs-1273 */                                     (outer.span.shrink_to_lo(), "<".to_string()),
/* FP:suggestions.rs-1274 */                                     (span.shrink_to_hi(), ">".to_string()),
/* FP:suggestions.rs-1275 */                                 ],
/* FP:suggestions.rs-1276 */                                 Applicability::MachineApplicable,
/* FP:suggestions.rs-1277 */                             );
/* FP:suggestions.rs-1278 */                         }
/* FP:suggestions.rs-1279 */                         (true, _, hir::Mutability::Mut) => {
/* FP:suggestions.rs-1280 */                             // There's an associated function found on the immutable borrow of the
/* FP:suggestions.rs-1281 */                             err.multipart_suggestion_verbose(
/* FP:suggestions.rs-1282 */                                 sugg_msg("mut "),
/* FP:suggestions.rs-1283 */                                 vec![
/* FP:suggestions.rs-1284 */                                     (outer.span.shrink_to_lo().until(span), "<&".to_string()),
/* FP:suggestions.rs-1285 */                                     (span.shrink_to_hi(), ">".to_string()),
/* FP:suggestions.rs-1286 */                                 ],
/* FP:suggestions.rs-1287 */                                 Applicability::MachineApplicable,
/* FP:suggestions.rs-1288 */                             );
/* FP:suggestions.rs-1289 */                         }
/* FP:suggestions.rs-1290 */                         (_, true, hir::Mutability::Not) => {
/* FP:suggestions.rs-1291 */                             err.multipart_suggestion_verbose(
/* FP:suggestions.rs-1292 */                                 sugg_msg(""),
/* FP:suggestions.rs-1293 */                                 vec![
/* FP:suggestions.rs-1294 */                                     (outer.span.shrink_to_lo().until(span), "<&mut ".to_string()),
/* FP:suggestions.rs-1295 */                                     (span.shrink_to_hi(), ">".to_string()),
/* FP:suggestions.rs-1296 */                                 ],
/* FP:suggestions.rs-1297 */                                 Applicability::MachineApplicable,
/* FP:suggestions.rs-1298 */                             );
/* FP:suggestions.rs-1299 */                         }
/* FP:suggestions.rs-1300 */                         _ => {}
/* FP:suggestions.rs-1301 */                     }
/* FP:suggestions.rs-1302 */                     // If we didn't return early here, we would instead suggest `&&str::from("")`.
/* FP:suggestions.rs-1303 */                     return false;
/* FP:suggestions.rs-1304 */                 }
/* FP:suggestions.rs-1305 */                 c
/* FP:suggestions.rs-1306 */             }
/* FP:suggestions.rs-1307 */             c if matches!(
/* FP:suggestions.rs-1308 */                 span.ctxt().outer_expn_data().kind,
/* FP:suggestions.rs-1309 */                 ExpnKind::Desugaring(DesugaringKind::ForLoop)
/* FP:suggestions.rs-1310 */             ) =>
/* FP:suggestions.rs-1311 */             {
/* FP:suggestions.rs-1312 */                 c
/* FP:suggestions.rs-1313 */             }
/* FP:suggestions.rs-1314 */             _ => return false,
/* FP:suggestions.rs-1315 */         };
/* FP:suggestions.rs-1316 */ 
/* FP:suggestions.rs-1317 */         // List of traits for which it would be nonsensical to suggest borrowing.
/* FP:suggestions.rs-1318 */         // For instance, immutable references are always Copy, so suggesting to
/* FP:suggestions.rs-1319 */         // borrow would always succeed, but it's probably not what the user wanted.
/* FP:suggestions.rs-1320 */         let mut never_suggest_borrow: Vec<_> =
/* FP:suggestions.rs-1321 */             [LangItem::Copy, LangItem::Clone, LangItem::Unpin, LangItem::Sized]
/* FP:suggestions.rs-1322 */                 .iter()
/* FP:suggestions.rs-1323 */                 .filter_map(|lang_item| self.tcx.lang_items().get(*lang_item))
/* FP:suggestions.rs-1324 */                 .collect();
/* FP:suggestions.rs-1325 */ 
/* FP:suggestions.rs-1326 */         if let Some(def_id) = self.tcx.get_diagnostic_item(sym::Send) {
/* FP:suggestions.rs-1327 */             never_suggest_borrow.push(def_id);
/* FP:suggestions.rs-1328 */         }
/* FP:suggestions.rs-1329 */ 
/* FP:suggestions.rs-1330 */         // Try to apply the original trait bound by borrowing.
/* FP:suggestions.rs-1331 */         let mut try_borrowing = |old_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-1332 */                                  blacklist: &[DefId]|
/* FP:suggestions.rs-1333 */          -> bool {
/* FP:suggestions.rs-1334 */             if blacklist.contains(&old_pred.def_id()) {
/* FP:suggestions.rs-1335 */                 return false;
/* FP:suggestions.rs-1336 */             }
/* FP:suggestions.rs-1337 */             // We map bounds to `&T` and `&mut T`
/* FP:suggestions.rs-1338 */             let trait_pred_and_imm_ref = old_pred.map_bound(|trait_pred| {
/* FP:suggestions.rs-1339 */                 (
/* FP:suggestions.rs-1340 */                     trait_pred,
/* FP:suggestions.rs-1341 */                     Ty::new_imm_ref(self.tcx, self.tcx.lifetimes.re_static, trait_pred.self_ty()),
/* FP:suggestions.rs-1342 */                 )
/* FP:suggestions.rs-1343 */             });
/* FP:suggestions.rs-1344 */             let trait_pred_and_mut_ref = old_pred.map_bound(|trait_pred| {
/* FP:suggestions.rs-1345 */                 (
/* FP:suggestions.rs-1346 */                     trait_pred,
/* FP:suggestions.rs-1347 */                     Ty::new_mut_ref(self.tcx, self.tcx.lifetimes.re_static, trait_pred.self_ty()),
/* FP:suggestions.rs-1348 */                 )
/* FP:suggestions.rs-1349 */             });
/* FP:suggestions.rs-1350 */ 
/* FP:suggestions.rs-1351 */             let imm_ref_self_ty_satisfies_pred = mk_result(trait_pred_and_imm_ref);
/* FP:suggestions.rs-1352 */             let mut_ref_self_ty_satisfies_pred = mk_result(trait_pred_and_mut_ref);
/* FP:suggestions.rs-1353 */ 
/* FP:suggestions.rs-1354 */             let (ref_inner_ty_satisfies_pred, ref_inner_ty_is_mut) =
/* FP:suggestions.rs-1355 */                 if let ObligationCauseCode::WhereClauseInExpr(..) = obligation.cause.code()
/* FP:suggestions.rs-1356 */                     && let ty::Ref(_, ty, mutability) = old_pred.self_ty().skip_binder().kind()
/* FP:suggestions.rs-1357 */                 {
/* FP:suggestions.rs-1358 */                     (
/* FP:suggestions.rs-1359 */                         mk_result(old_pred.map_bound(|trait_pred| (trait_pred, *ty))),
/* FP:suggestions.rs-1360 */                         mutability.is_mut(),
/* FP:suggestions.rs-1361 */                     )
/* FP:suggestions.rs-1362 */                 } else {
/* FP:suggestions.rs-1363 */                     (false, false)
/* FP:suggestions.rs-1364 */                 };
/* FP:suggestions.rs-1365 */ 
/* FP:suggestions.rs-1366 */             let is_immut = imm_ref_self_ty_satisfies_pred
/* FP:suggestions.rs-1367 */                 || (ref_inner_ty_satisfies_pred && !ref_inner_ty_is_mut);
/* FP:suggestions.rs-1368 */             let is_mut = mut_ref_self_ty_satisfies_pred || ref_inner_ty_is_mut;
/* FP:suggestions.rs-1369 */             if !is_immut && !is_mut {
/* FP:suggestions.rs-1370 */                 return false;
/* FP:suggestions.rs-1371 */             }
/* FP:suggestions.rs-1372 */             let Ok(_snippet) = self.tcx.sess.source_map().span_to_snippet(span) else {
/* FP:suggestions.rs-1373 */                 return false;
/* FP:suggestions.rs-1374 */             };
/* FP:suggestions.rs-1375 */             // We don't want a borrowing suggestion on the fields in structs
/* FP:suggestions.rs-1376 */             // ```
/* FP:suggestions.rs-1377 */             // #[derive(Clone)]
/* FP:suggestions.rs-1378 */             // struct Foo {
/* FP:suggestions.rs-1379 */             //     the_foos: Vec<Foo>
/* FP:suggestions.rs-1380 */             // }
/* FP:suggestions.rs-1381 */             // ```
/* FP:suggestions.rs-1382 */             if !matches!(
/* FP:suggestions.rs-1383 */                 span.ctxt().outer_expn_data().kind,
/* FP:suggestions.rs-1384 */                 ExpnKind::Root | ExpnKind::Desugaring(DesugaringKind::ForLoop)
/* FP:suggestions.rs-1385 */             ) {
/* FP:suggestions.rs-1386 */                 return false;
/* FP:suggestions.rs-1387 */             }
/* FP:suggestions.rs-1388 */             // We have a very specific type of error, where just borrowing this argument
/* FP:suggestions.rs-1389 */             // might solve the problem. In cases like this, the important part is the
/* FP:suggestions.rs-1390 */             // original type obligation, not the last one that failed, which is arbitrary.
/* FP:suggestions.rs-1391 */             // Because of this, we modify the error to refer to the original obligation and
/* FP:suggestions.rs-1392 */             // return early in the caller.
/* FP:suggestions.rs-1393 */ 
/* FP:suggestions.rs-1394 */             let mut label = || {
/* FP:suggestions.rs-1395 */                 let msg = format!(
/* FP:suggestions.rs-1396 */                     "the trait bound `{}` is not satisfied",
/* FP:suggestions.rs-1397 */                     self.tcx.short_string(old_pred, err.long_ty_path()),
/* FP:suggestions.rs-1398 */                 );
/* FP:suggestions.rs-1399 */                 let self_ty_str =
/* FP:suggestions.rs-1400 */                     self.tcx.short_string(old_pred.self_ty().skip_binder(), err.long_ty_path());
/* FP:suggestions.rs-1401 */                 let trait_path = self
/* FP:suggestions.rs-1402 */                     .tcx
/* FP:suggestions.rs-1403 */                     .short_string(old_pred.print_modifiers_and_trait_path(), err.long_ty_path());
/* FP:suggestions.rs-1404 */ 
/* FP:suggestions.rs-1405 */                 if has_custom_message {
/* FP:suggestions.rs-1406 */                     err.note(msg);
/* FP:suggestions.rs-1407 */                 } else {
/* FP:suggestions.rs-1408 */                     err.messages = vec![(crate::rustc_errors::DiagMessage::from(msg), Style::NoStyle)];
/* FP:suggestions.rs-1409 */                 }
/* FP:suggestions.rs-1410 */                 err.span_label(
/* FP:suggestions.rs-1411 */                     span,
/* FP:suggestions.rs-1412 */                     format!("the trait `{trait_path}` is not implemented for `{self_ty_str}`"),
/* FP:suggestions.rs-1413 */                 );
/* FP:suggestions.rs-1414 */             };
/* FP:suggestions.rs-1415 */ 
/* FP:suggestions.rs-1416 */             let mut sugg_prefixes = vec![];
/* FP:suggestions.rs-1417 */             if is_immut {
/* FP:suggestions.rs-1418 */                 sugg_prefixes.push("&");
/* FP:suggestions.rs-1419 */             }
/* FP:suggestions.rs-1420 */             if is_mut {
/* FP:suggestions.rs-1421 */                 sugg_prefixes.push("&mut ");
/* FP:suggestions.rs-1422 */             }
/* FP:suggestions.rs-1423 */             let sugg_msg = format!(
/* FP:suggestions.rs-1424 */                 "consider{} borrowing here",
/* FP:suggestions.rs-1425 */                 if is_mut && !is_immut { " mutably" } else { "" },
/* FP:suggestions.rs-1426 */             );
/* FP:suggestions.rs-1427 */ 
/* FP:suggestions.rs-1428 */             // Issue #104961, we need to add parentheses properly for compound expressions
/* FP:suggestions.rs-1429 */             // for example, `x.starts_with("hi".to_string() + "you")`
/* FP:suggestions.rs-1430 */             // should be `x.starts_with(&("hi".to_string() + "you"))`
/* FP:suggestions.rs-1431 */             let Some(body) = self.tcx.hir_maybe_body_owned_by(obligation.cause.body_id) else {
/* FP:suggestions.rs-1432 */                 return false;
/* FP:suggestions.rs-1433 */             };
/* FP:suggestions.rs-1434 */             let mut expr_finder = FindExprBySpan::new(span, self.tcx);
/* FP:suggestions.rs-1435 */             expr_finder.visit_expr(body.value);
/* FP:suggestions.rs-1436 */ 
/* FP:suggestions.rs-1437 */             if let Some(ty) = expr_finder.ty_result {
/* FP:suggestions.rs-1438 */                 if let hir::Node::Expr(expr) = self.tcx.parent_hir_node(ty.hir_id)
/* FP:suggestions.rs-1439 */                     && let hir::ExprKind::Path(hir::QPath::TypeRelative(_, _)) = expr.kind
/* FP:suggestions.rs-1440 */                     && ty.span == span
/* FP:suggestions.rs-1441 */                 {
/* FP:suggestions.rs-1442 */                     // We've encountered something like `str::from("")`, where the intended code
/* FP:suggestions.rs-1443 */                     // was likely `<&str>::from("")`. #143393.
/* FP:suggestions.rs-1444 */                     label();
/* FP:suggestions.rs-1445 */                     err.multipart_suggestions(
/* FP:suggestions.rs-1446 */                         sugg_msg,
/* FP:suggestions.rs-1447 */                         sugg_prefixes.into_iter().map(|sugg_prefix| {
/* FP:suggestions.rs-1448 */                             vec![
/* FP:suggestions.rs-1449 */                                 (span.shrink_to_lo(), format!("<{sugg_prefix}")),
/* FP:suggestions.rs-1450 */                                 (span.shrink_to_hi(), ">".to_string()),
/* FP:suggestions.rs-1451 */                             ]
/* FP:suggestions.rs-1452 */                         }),
/* FP:suggestions.rs-1453 */                         Applicability::MaybeIncorrect,
/* FP:suggestions.rs-1454 */                     );
/* FP:suggestions.rs-1455 */                     return true;
/* FP:suggestions.rs-1456 */                 }
/* FP:suggestions.rs-1457 */                 return false;
/* FP:suggestions.rs-1458 */             }
/* FP:suggestions.rs-1459 */             let Some(expr) = expr_finder.result else {
/* FP:suggestions.rs-1460 */                 return false;
/* FP:suggestions.rs-1461 */             };
/* FP:suggestions.rs-1462 */             if let hir::ExprKind::AddrOf(_, _, _) = expr.kind {
/* FP:suggestions.rs-1463 */                 return false;
/* FP:suggestions.rs-1464 */             }
/* FP:suggestions.rs-1465 */             let needs_parens_post = expr_needs_parens(expr);
/* FP:suggestions.rs-1466 */             let needs_parens_pre = match self.tcx.parent_hir_node(expr.hir_id) {
/* FP:suggestions.rs-1467 */                 Node::Expr(e)
/* FP:suggestions.rs-1468 */                     if let hir::ExprKind::MethodCall(_, base, _, _) = e.kind
/* FP:suggestions.rs-1469 */                         && base.hir_id == expr.hir_id =>
/* FP:suggestions.rs-1470 */                 {
/* FP:suggestions.rs-1471 */                     true
/* FP:suggestions.rs-1472 */                 }
/* FP:suggestions.rs-1473 */                 _ => false,
/* FP:suggestions.rs-1474 */             };
/* FP:suggestions.rs-1475 */ 
/* FP:suggestions.rs-1476 */             label();
/* FP:suggestions.rs-1477 */             let suggestions = sugg_prefixes.into_iter().map(|sugg_prefix| {
/* FP:suggestions.rs-1478 */                 match (needs_parens_pre, needs_parens_post) {
/* FP:suggestions.rs-1479 */                     (false, false) => vec![(span.shrink_to_lo(), sugg_prefix.to_string())],
/* FP:suggestions.rs-1480 */                     // We have something like `foo.bar()`, where we want to bororw foo, so we need
/* FP:suggestions.rs-1481 */                     // to suggest `(&mut foo).bar()`.
/* FP:suggestions.rs-1482 */                     (false, true) => vec![
/* FP:suggestions.rs-1483 */                         (span.shrink_to_lo(), format!("{sugg_prefix}(")),
/* FP:suggestions.rs-1484 */                         (span.shrink_to_hi(), ")".to_string()),
/* FP:suggestions.rs-1485 */                     ],
/* FP:suggestions.rs-1486 */                     // Issue #109436, we need to add parentheses properly for method calls
/* FP:suggestions.rs-1487 */                     // for example, `foo.into()` should be `(&foo).into()`
/* FP:suggestions.rs-1488 */                     (true, false) => vec![
/* FP:suggestions.rs-1489 */                         (span.shrink_to_lo(), format!("({sugg_prefix}")),
/* FP:suggestions.rs-1490 */                         (span.shrink_to_hi(), ")".to_string()),
/* FP:suggestions.rs-1491 */                     ],
/* FP:suggestions.rs-1492 */                     (true, true) => vec![
/* FP:suggestions.rs-1493 */                         (span.shrink_to_lo(), format!("({sugg_prefix}(")),
/* FP:suggestions.rs-1494 */                         (span.shrink_to_hi(), "))".to_string()),
/* FP:suggestions.rs-1495 */                     ],
/* FP:suggestions.rs-1496 */                 }
/* FP:suggestions.rs-1497 */             });
/* FP:suggestions.rs-1498 */             err.multipart_suggestions(sugg_msg, suggestions, Applicability::MaybeIncorrect);
/* FP:suggestions.rs-1499 */             return true;
/* FP:suggestions.rs-1500 */         };
/* FP:suggestions.rs-1501 */ 
/* FP:suggestions.rs-1502 */         if let ObligationCauseCode::ImplDerived(cause) = &*code {
/* FP:suggestions.rs-1503 */             try_borrowing(cause.derived.parent_trait_pred, &[])
/* FP:suggestions.rs-1504 */         } else if let ObligationCauseCode::WhereClause(..)
/* FP:suggestions.rs-1505 */         | ObligationCauseCode::WhereClauseInExpr(..) = code
/* FP:suggestions.rs-1506 */         {
/* FP:suggestions.rs-1507 */             try_borrowing(poly_trait_pred, &never_suggest_borrow)
/* FP:suggestions.rs-1508 */         } else {
/* FP:suggestions.rs-1509 */             false
/* FP:suggestions.rs-1510 */         }
/* FP:suggestions.rs-1511 */     }
/* FP:suggestions.rs-1512 */ 
/* FP:suggestions.rs-1513 */     // Suggest borrowing the type
/* FP:suggestions.rs-1514 */     pub(super) fn suggest_borrowing_for_object_cast(
/* FP:suggestions.rs-1515 */         &self,
/* FP:suggestions.rs-1516 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-1517 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-1518 */         self_ty: Ty<'tcx>,
/* FP:suggestions.rs-1519 */         target_ty: Ty<'tcx>,
/* FP:suggestions.rs-1520 */     ) {
/* FP:suggestions.rs-1521 */         let ty::Ref(_, object_ty, hir::Mutability::Not) = target_ty.kind() else {
/* FP:suggestions.rs-1522 */             return;
/* FP:suggestions.rs-1523 */         };
/* FP:suggestions.rs-1524 */         let ty::Dynamic(predicates, _, ty::Dyn) = object_ty.kind() else {
/* FP:suggestions.rs-1525 */             return;
/* FP:suggestions.rs-1526 */         };
/* FP:suggestions.rs-1527 */         let self_ref_ty = Ty::new_imm_ref(self.tcx, self.tcx.lifetimes.re_erased, self_ty);
/* FP:suggestions.rs-1528 */ 
/* FP:suggestions.rs-1529 */         for predicate in predicates.iter() {
/* FP:suggestions.rs-1530 */             if !self.predicate_must_hold_modulo_regions(
/* FP:suggestions.rs-1531 */                 &obligation.with(self.tcx, predicate.with_self_ty(self.tcx, self_ref_ty)),
/* FP:suggestions.rs-1532 */             ) {
/* FP:suggestions.rs-1533 */                 return;
/* FP:suggestions.rs-1534 */             }
/* FP:suggestions.rs-1535 */         }
/* FP:suggestions.rs-1536 */ 
/* FP:suggestions.rs-1537 */         err.span_suggestion_verbose(
/* FP:suggestions.rs-1538 */             obligation.cause.span.shrink_to_lo(),
/* FP:suggestions.rs-1539 */             format!(
/* FP:suggestions.rs-1540 */                 "consider borrowing the value, since `&{self_ty}` can be coerced into `{target_ty}`"
/* FP:suggestions.rs-1541 */             ),
/* FP:suggestions.rs-1542 */             "&",
/* FP:suggestions.rs-1543 */             Applicability::MaybeIncorrect,
/* FP:suggestions.rs-1544 */         );
/* FP:suggestions.rs-1545 */     }
/* FP:suggestions.rs-1546 */ 
/* FP:suggestions.rs-1547 */     /// Whenever references are used by mistake, like `for (i, e) in &vec.iter().enumerate()`,
/* FP:suggestions.rs-1548 */     /// suggest removing these references until we reach a type that implements the trait.
/* FP:suggestions.rs-1549 */     pub(super) fn suggest_remove_reference(
/* FP:suggestions.rs-1550 */         &self,
/* FP:suggestions.rs-1551 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-1552 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-1553 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-1554 */     ) -> bool {
/* FP:suggestions.rs-1555 */         let mut span = obligation.cause.span;
/* FP:suggestions.rs-1556 */         let mut trait_pred = trait_pred;
/* FP:suggestions.rs-1557 */         let mut code = obligation.cause.code();
/* FP:suggestions.rs-1558 */         while let Some((c, Some(parent_trait_pred))) = code.parent_with_predicate() {
/* FP:suggestions.rs-1559 */             // We want the root obligation, in order to detect properly handle
/* FP:suggestions.rs-1560 */             // `for _ in &mut &mut vec![] {}`.
/* FP:suggestions.rs-1561 */             code = c;
/* FP:suggestions.rs-1562 */             trait_pred = parent_trait_pred;
/* FP:suggestions.rs-1563 */         }
/* FP:suggestions.rs-1564 */         while span.desugaring_kind().is_some() {
/* FP:suggestions.rs-1565 */             // Remove all the hir desugaring contexts while maintaining the macro contexts.
/* FP:suggestions.rs-1566 */             span.remove_mark();
/* FP:suggestions.rs-1567 */         }
/* FP:suggestions.rs-1568 */         let mut expr_finder = super::FindExprBySpan::new(span, self.tcx);
/* FP:suggestions.rs-1569 */         let Some(body) = self.tcx.hir_maybe_body_owned_by(obligation.cause.body_id) else {
/* FP:suggestions.rs-1570 */             return false;
/* FP:suggestions.rs-1571 */         };
/* FP:suggestions.rs-1572 */         expr_finder.visit_expr(body.value);
/* FP:suggestions.rs-1573 */         let mut maybe_suggest = |suggested_ty, count, suggestions| {
/* FP:suggestions.rs-1574 */             // Remapping bound vars here
/* FP:suggestions.rs-1575 */             let trait_pred_and_suggested_ty =
/* FP:suggestions.rs-1576 */                 trait_pred.map_bound(|trait_pred| (trait_pred, suggested_ty));
/* FP:suggestions.rs-1577 */ 
/* FP:suggestions.rs-1578 */             let new_obligation = self.mk_trait_obligation_with_new_self_ty(
/* FP:suggestions.rs-1579 */                 obligation.param_env,
/* FP:suggestions.rs-1580 */                 trait_pred_and_suggested_ty,
/* FP:suggestions.rs-1581 */             );
/* FP:suggestions.rs-1582 */ 
/* FP:suggestions.rs-1583 */             if self.predicate_may_hold(&new_obligation) {
/* FP:suggestions.rs-1584 */                 let msg = if count == 1 {
/* FP:suggestions.rs-1585 */                     "consider removing the leading `&`-reference".to_string()
/* FP:suggestions.rs-1586 */                 } else {
/* FP:suggestions.rs-1587 */                     format!("consider removing {count} leading `&`-references")
/* FP:suggestions.rs-1588 */                 };
/* FP:suggestions.rs-1589 */ 
/* FP:suggestions.rs-1590 */                 err.multipart_suggestion_verbose(
/* FP:suggestions.rs-1591 */                     msg,
/* FP:suggestions.rs-1592 */                     suggestions,
/* FP:suggestions.rs-1593 */                     Applicability::MachineApplicable,
/* FP:suggestions.rs-1594 */                 );
/* FP:suggestions.rs-1595 */                 true
/* FP:suggestions.rs-1596 */             } else {
/* FP:suggestions.rs-1597 */                 false
/* FP:suggestions.rs-1598 */             }
/* FP:suggestions.rs-1599 */         };
/* FP:suggestions.rs-1600 */ 
/* FP:suggestions.rs-1601 */         // Maybe suggest removal of borrows from types in type parameters, like in
/* FP:suggestions.rs-1602 */         // `src/test/ui/not-panic/not-panic-safe.rs`.
/* FP:suggestions.rs-1603 */         let mut count = 0;
/* FP:suggestions.rs-1604 */         let mut suggestions = vec![];
/* FP:suggestions.rs-1605 */         // Skipping binder here, remapping below
/* FP:suggestions.rs-1606 */         let mut suggested_ty = trait_pred.self_ty().skip_binder();
/* FP:suggestions.rs-1607 */         if let Some(mut hir_ty) = expr_finder.ty_result {
/* FP:suggestions.rs-1608 */             while let hir::TyKind::Ref(_, mut_ty) = &hir_ty.kind {
/* FP:suggestions.rs-1609 */                 count += 1;
/* FP:suggestions.rs-1610 */                 let span = hir_ty.span.until(mut_ty.ty.span);
/* FP:suggestions.rs-1611 */                 suggestions.push((span, String::new()));
/* FP:suggestions.rs-1612 */ 
/* FP:suggestions.rs-1613 */                 let ty::Ref(_, inner_ty, _) = suggested_ty.kind() else {
/* FP:suggestions.rs-1614 */                     break;
/* FP:suggestions.rs-1615 */                 };
/* FP:suggestions.rs-1616 */                 suggested_ty = *inner_ty;
/* FP:suggestions.rs-1617 */ 
/* FP:suggestions.rs-1618 */                 hir_ty = mut_ty.ty;
/* FP:suggestions.rs-1619 */ 
/* FP:suggestions.rs-1620 */                 if maybe_suggest(suggested_ty, count, suggestions.clone()) {
/* FP:suggestions.rs-1621 */                     return true;
/* FP:suggestions.rs-1622 */                 }
/* FP:suggestions.rs-1623 */             }
/* FP:suggestions.rs-1624 */         }
/* FP:suggestions.rs-1625 */ 
/* FP:suggestions.rs-1626 */         // Maybe suggest removal of borrows from expressions, like in `for i in &&&foo {}`.
/* FP:suggestions.rs-1627 */         let Some(mut expr) = expr_finder.result else {
/* FP:suggestions.rs-1628 */             return false;
/* FP:suggestions.rs-1629 */         };
/* FP:suggestions.rs-1630 */         let mut count = 0;
/* FP:suggestions.rs-1631 */         let mut suggestions = vec![];
/* FP:suggestions.rs-1632 */         // Skipping binder here, remapping below
/* FP:suggestions.rs-1633 */         let mut suggested_ty = trait_pred.self_ty().skip_binder();
/* FP:suggestions.rs-1634 */         'outer: loop {
/* FP:suggestions.rs-1635 */             while let hir::ExprKind::AddrOf(_, _, borrowed) = expr.kind {
/* FP:suggestions.rs-1636 */                 count += 1;
/* FP:suggestions.rs-1637 */                 let span =
/* FP:suggestions.rs-1638 */                     if let Some(borrowed_span) = borrowed.span.find_ancestor_inside(expr.span) {
/* FP:suggestions.rs-1639 */                         expr.span.until(borrowed_span)
/* FP:suggestions.rs-1640 */                     } else {
/* FP:suggestions.rs-1641 */                         break 'outer;
/* FP:suggestions.rs-1642 */                     };
/* FP:suggestions.rs-1643 */ 
/* FP:suggestions.rs-1644 */                 // Double check that the span we extracted actually corresponds to a borrow,
/* FP:suggestions.rs-1645 */                 // rather than some macro garbage.
/* FP:suggestions.rs-1646 */                 match self.tcx.sess.source_map().span_to_snippet(span) {
/* FP:suggestions.rs-1647 */                     Ok(snippet) if snippet.starts_with("&") => {}
/* FP:suggestions.rs-1648 */                     _ => break 'outer,
/* FP:suggestions.rs-1649 */                 }
/* FP:suggestions.rs-1650 */ 
/* FP:suggestions.rs-1651 */                 suggestions.push((span, String::new()));
/* FP:suggestions.rs-1652 */ 
/* FP:suggestions.rs-1653 */                 let ty::Ref(_, inner_ty, _) = suggested_ty.kind() else {
/* FP:suggestions.rs-1654 */                     break 'outer;
/* FP:suggestions.rs-1655 */                 };
/* FP:suggestions.rs-1656 */                 suggested_ty = *inner_ty;
/* FP:suggestions.rs-1657 */ 
/* FP:suggestions.rs-1658 */                 expr = borrowed;
/* FP:suggestions.rs-1659 */ 
/* FP:suggestions.rs-1660 */                 if maybe_suggest(suggested_ty, count, suggestions.clone()) {
/* FP:suggestions.rs-1661 */                     return true;
/* FP:suggestions.rs-1662 */                 }
/* FP:suggestions.rs-1663 */             }
/* FP:suggestions.rs-1664 */             if let hir::ExprKind::Path(hir::QPath::Resolved(None, path)) = expr.kind
/* FP:suggestions.rs-1665 */                 && let Res::Local(hir_id) = path.res
/* FP:suggestions.rs-1666 */                 && let hir::Node::Pat(binding) = self.tcx.hir_node(hir_id)
/* FP:suggestions.rs-1667 */                 && let hir::Node::LetStmt(local) = self.tcx.parent_hir_node(binding.hir_id)
/* FP:suggestions.rs-1668 */                 && let None = local.ty
/* FP:suggestions.rs-1669 */                 && let Some(binding_expr) = local.init
/* FP:suggestions.rs-1670 */             {
/* FP:suggestions.rs-1671 */                 expr = binding_expr;
/* FP:suggestions.rs-1672 */             } else {
/* FP:suggestions.rs-1673 */                 break 'outer;
/* FP:suggestions.rs-1674 */             }
/* FP:suggestions.rs-1675 */         }
/* FP:suggestions.rs-1676 */         false
/* FP:suggestions.rs-1677 */     }
/* FP:suggestions.rs-1678 */ 
/* FP:suggestions.rs-1679 */     pub(super) fn suggest_remove_await(
/* FP:suggestions.rs-1680 */         &self,
/* FP:suggestions.rs-1681 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-1682 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-1683 */     ) {
/* FP:suggestions.rs-1684 */         if let ObligationCauseCode::AwaitableExpr(hir_id) = obligation.cause.code().peel_derives()
/* FP:suggestions.rs-1685 */             && let hir::Node::Expr(expr) = self.tcx.hir_node(*hir_id)
/* FP:suggestions.rs-1686 */         {
/* FP:suggestions.rs-1687 */             // FIXME: use `obligation.predicate.kind()...trait_ref.self_ty()` to see if we have `()`
/* FP:suggestions.rs-1688 */             // and if not maybe suggest doing something else? If we kept the expression around we
/* FP:suggestions.rs-1689 */             // could also check if it is an fn call (very likely) and suggest changing *that*, if
/* FP:suggestions.rs-1690 */             // it is from the local crate.
/* FP:suggestions.rs-1691 */ 
/* FP:suggestions.rs-1692 */             // use nth(1) to skip one layer of desugaring from `IntoIter::into_iter`
/* FP:suggestions.rs-1693 */             if let Some((_, hir::Node::Expr(await_expr))) = self.tcx.hir_parent_iter(*hir_id).nth(1)
/* FP:suggestions.rs-1694 */                 && let Some(expr_span) = expr.span.find_ancestor_inside_same_ctxt(await_expr.span)
/* FP:suggestions.rs-1695 */             {
/* FP:suggestions.rs-1696 */                 let removal_span = self
/* FP:suggestions.rs-1697 */                     .tcx
/* FP:suggestions.rs-1698 */                     .sess
/* FP:suggestions.rs-1699 */                     .source_map()
/* FP:suggestions.rs-1700 */                     .span_extend_while_whitespace(expr_span)
/* FP:suggestions.rs-1701 */                     .shrink_to_hi()
/* FP:suggestions.rs-1702 */                     .to(await_expr.span.shrink_to_hi());
/* FP:suggestions.rs-1703 */                 err.span_suggestion_verbose(
/* FP:suggestions.rs-1704 */                     removal_span,
/* FP:suggestions.rs-1705 */                     "remove the `.await`",
/* FP:suggestions.rs-1706 */                     "",
/* FP:suggestions.rs-1707 */                     Applicability::MachineApplicable,
/* FP:suggestions.rs-1708 */                 );
/* FP:suggestions.rs-1709 */             } else {
/* FP:suggestions.rs-1710 */                 err.span_label(obligation.cause.span, "remove the `.await`");
/* FP:suggestions.rs-1711 */             }
/* FP:suggestions.rs-1712 */             // FIXME: account for associated `async fn`s.
/* FP:suggestions.rs-1713 */             if let hir::Expr { span, kind: hir::ExprKind::Call(base, _), .. } = expr {
/* FP:suggestions.rs-1714 */                 if let ty::PredicateKind::Clause(ty::ClauseKind::Trait(pred)) =
/* FP:suggestions.rs-1715 */                     obligation.predicate.kind().skip_binder()
/* FP:suggestions.rs-1716 */                 {
/* FP:suggestions.rs-1717 */                     err.span_label(*span, format!("this call returns `{}`", pred.self_ty()));
/* FP:suggestions.rs-1718 */                 }
/* FP:suggestions.rs-1719 */                 if let Some(typeck_results) = &self.typeck_results
/* FP:suggestions.rs-1720 */                     && let ty = typeck_results.expr_ty_adjusted(base)
/* FP:suggestions.rs-1721 */                     && let ty::FnDef(def_id, _args) = ty.kind()
/* FP:suggestions.rs-1722 */                     && let Some(hir::Node::Item(item)) = self.tcx.hir_get_if_local(*def_id)
/* FP:suggestions.rs-1723 */                 {
/* FP:suggestions.rs-1724 */                     let (ident, _, _, _) = item.expect_fn();
/* FP:suggestions.rs-1725 */                     let msg = format!("alternatively, consider making `fn {ident}` asynchronous");
/* FP:suggestions.rs-1726 */                     if item.vis_span.is_empty() {
/* FP:suggestions.rs-1727 */                         err.span_suggestion_verbose(
/* FP:suggestions.rs-1728 */                             item.span.shrink_to_lo(),
/* FP:suggestions.rs-1729 */                             msg,
/* FP:suggestions.rs-1730 */                             "async ",
/* FP:suggestions.rs-1731 */                             Applicability::MaybeIncorrect,
/* FP:suggestions.rs-1732 */                         );
/* FP:suggestions.rs-1733 */                     } else {
/* FP:suggestions.rs-1734 */                         err.span_suggestion_verbose(
/* FP:suggestions.rs-1735 */                             item.vis_span.shrink_to_hi(),
/* FP:suggestions.rs-1736 */                             msg,
/* FP:suggestions.rs-1737 */                             " async",
/* FP:suggestions.rs-1738 */                             Applicability::MaybeIncorrect,
/* FP:suggestions.rs-1739 */                         );
/* FP:suggestions.rs-1740 */                     }
/* FP:suggestions.rs-1741 */                 }
/* FP:suggestions.rs-1742 */             }
/* FP:suggestions.rs-1743 */         }
/* FP:suggestions.rs-1744 */     }
/* FP:suggestions.rs-1745 */ 
/* FP:suggestions.rs-1746 */     /// Check if the trait bound is implemented for a different mutability and note it in the
/* FP:suggestions.rs-1747 */     /// final error.
/* FP:suggestions.rs-1748 */     pub(super) fn suggest_change_mut(
/* FP:suggestions.rs-1749 */         &self,
/* FP:suggestions.rs-1750 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-1751 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-1752 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-1753 */     ) {
/* FP:suggestions.rs-1754 */         let points_at_arg =
/* FP:suggestions.rs-1755 */             matches!(obligation.cause.code(), ObligationCauseCode::FunctionArg { .. },);
/* FP:suggestions.rs-1756 */ 
/* FP:suggestions.rs-1757 */         let span = obligation.cause.span;
/* FP:suggestions.rs-1758 */         if let Ok(snippet) = self.tcx.sess.source_map().span_to_snippet(span) {
/* FP:suggestions.rs-1759 */             let refs_number =
/* FP:suggestions.rs-1760 */                 snippet.chars().filter(|c| !c.is_whitespace()).take_while(|c| *c == '&').count();
/* FP:suggestions.rs-1761 */             if let Some('\'') = snippet.chars().filter(|c| !c.is_whitespace()).nth(refs_number) {
/* FP:suggestions.rs-1762 */                 // Do not suggest removal of borrow from type arguments.
/* FP:suggestions.rs-1763 */                 return;
/* FP:suggestions.rs-1764 */             }
/* FP:suggestions.rs-1765 */             let trait_pred = self.resolve_vars_if_possible(trait_pred);
/* FP:suggestions.rs-1766 */             if trait_pred.has_non_region_infer() {
/* FP:suggestions.rs-1767 */                 // Do not ICE while trying to find if a reborrow would succeed on a trait with
/* FP:suggestions.rs-1768 */                 // unresolved bindings.
/* FP:suggestions.rs-1769 */                 return;
/* FP:suggestions.rs-1770 */             }
/* FP:suggestions.rs-1771 */ 
/* FP:suggestions.rs-1772 */             // Skipping binder here, remapping below
/* FP:suggestions.rs-1773 */             if let ty::Ref(region, t_type, mutability) = *trait_pred.skip_binder().self_ty().kind()
/* FP:suggestions.rs-1774 */             {
/* FP:suggestions.rs-1775 */                 let suggested_ty = match mutability {
/* FP:suggestions.rs-1776 */                     hir::Mutability::Mut => Ty::new_imm_ref(self.tcx, region, t_type),
/* FP:suggestions.rs-1777 */                     hir::Mutability::Not => Ty::new_mut_ref(self.tcx, region, t_type),
/* FP:suggestions.rs-1778 */                 };
/* FP:suggestions.rs-1779 */ 
/* FP:suggestions.rs-1780 */                 // Remapping bound vars here
/* FP:suggestions.rs-1781 */                 let trait_pred_and_suggested_ty =
/* FP:suggestions.rs-1782 */                     trait_pred.map_bound(|trait_pred| (trait_pred, suggested_ty));
/* FP:suggestions.rs-1783 */ 
/* FP:suggestions.rs-1784 */                 let new_obligation = self.mk_trait_obligation_with_new_self_ty(
/* FP:suggestions.rs-1785 */                     obligation.param_env,
/* FP:suggestions.rs-1786 */                     trait_pred_and_suggested_ty,
/* FP:suggestions.rs-1787 */                 );
/* FP:suggestions.rs-1788 */                 let suggested_ty_would_satisfy_obligation = self
/* FP:suggestions.rs-1789 */                     .evaluate_obligation_no_overflow(&new_obligation)
/* FP:suggestions.rs-1790 */                     .must_apply_modulo_regions();
/* FP:suggestions.rs-1791 */                 if suggested_ty_would_satisfy_obligation {
/* FP:suggestions.rs-1792 */                     let sp = self
/* FP:suggestions.rs-1793 */                         .tcx
/* FP:suggestions.rs-1794 */                         .sess
/* FP:suggestions.rs-1795 */                         .source_map()
/* FP:suggestions.rs-1796 */                         .span_take_while(span, |c| c.is_whitespace() || *c == '&');
/* FP:suggestions.rs-1797 */                     if points_at_arg && mutability.is_not() && refs_number > 0 {
/* FP:suggestions.rs-1798 */                         // If we have a call like foo(&mut buf), then don't suggest foo(&mut mut buf)
/* FP:suggestions.rs-1799 */                         if snippet
/* FP:suggestions.rs-1800 */                             .trim_start_matches(|c: char| c.is_whitespace() || c == '&')
/* FP:suggestions.rs-1801 */                             .starts_with("mut")
/* FP:suggestions.rs-1802 */                         {
/* FP:suggestions.rs-1803 */                             return;
/* FP:suggestions.rs-1804 */                         }
/* FP:suggestions.rs-1805 */                         err.span_suggestion_verbose(
/* FP:suggestions.rs-1806 */                             sp,
/* FP:suggestions.rs-1807 */                             "consider changing this borrow's mutability",
/* FP:suggestions.rs-1808 */                             "&mut ",
/* FP:suggestions.rs-1809 */                             Applicability::MachineApplicable,
/* FP:suggestions.rs-1810 */                         );
/* FP:suggestions.rs-1811 */                     } else {
/* FP:suggestions.rs-1812 */                         err.note(format!(
/* FP:suggestions.rs-1813 */                             "`{}` is implemented for `{}`, but not for `{}`",
/* FP:suggestions.rs-1814 */                             trait_pred.print_modifiers_and_trait_path(),
/* FP:suggestions.rs-1815 */                             suggested_ty,
/* FP:suggestions.rs-1816 */                             trait_pred.skip_binder().self_ty(),
/* FP:suggestions.rs-1817 */                         ));
/* FP:suggestions.rs-1818 */                     }
/* FP:suggestions.rs-1819 */                 }
/* FP:suggestions.rs-1820 */             }
/* FP:suggestions.rs-1821 */         }
/* FP:suggestions.rs-1822 */     }
/* FP:suggestions.rs-1823 */ 
/* FP:suggestions.rs-1824 */     pub(super) fn suggest_semicolon_removal(
/* FP:suggestions.rs-1825 */         &self,
/* FP:suggestions.rs-1826 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-1827 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-1828 */         span: Span,
/* FP:suggestions.rs-1829 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-1830 */     ) -> bool {
/* FP:suggestions.rs-1831 */         let node = self.tcx.hir_node_by_def_id(obligation.cause.body_id);
/* FP:suggestions.rs-1832 */         if let hir::Node::Item(hir::Item { kind: hir::ItemKind::Fn {sig, body: body_id, .. }, .. }) = node
/* FP:suggestions.rs-1833 */             && let hir::ExprKind::Block(blk, _) = &self.tcx.hir_body(*body_id).value.kind
/* FP:suggestions.rs-1834 */             && sig.decl.output.span().overlaps(span)
/* FP:suggestions.rs-1835 */             && blk.expr.is_none()
/* FP:suggestions.rs-1836 */             && trait_pred.self_ty().skip_binder().is_unit()
/* FP:suggestions.rs-1837 */             && let Some(stmt) = blk.stmts.last()
/* FP:suggestions.rs-1838 */             && let hir::StmtKind::Semi(expr) = stmt.kind
/* FP:suggestions.rs-1839 */             // Only suggest this if the expression behind the semicolon implements the predicate
/* FP:suggestions.rs-1840 */             && let Some(typeck_results) = &self.typeck_results
/* FP:suggestions.rs-1841 */             && let Some(ty) = typeck_results.expr_ty_opt(expr)
/* FP:suggestions.rs-1842 */             && self.predicate_may_hold(&self.mk_trait_obligation_with_new_self_ty(
/* FP:suggestions.rs-1843 */                 obligation.param_env, trait_pred.map_bound(|trait_pred| (trait_pred, ty))
/* FP:suggestions.rs-1844 */             ))
/* FP:suggestions.rs-1845 */         {
/* FP:suggestions.rs-1846 */             err.span_label(
/* FP:suggestions.rs-1847 */                 expr.span,
/* FP:suggestions.rs-1848 */                 format!(
/* FP:suggestions.rs-1849 */                     "this expression has type `{}`, which implements `{}`",
/* FP:suggestions.rs-1850 */                     ty,
/* FP:suggestions.rs-1851 */                     trait_pred.print_modifiers_and_trait_path()
/* FP:suggestions.rs-1852 */                 ),
/* FP:suggestions.rs-1853 */             );
/* FP:suggestions.rs-1854 */             err.span_suggestion(
/* FP:suggestions.rs-1855 */                 self.tcx.sess.source_map().end_point(stmt.span),
/* FP:suggestions.rs-1856 */                 "remove this semicolon",
/* FP:suggestions.rs-1857 */                 "",
/* FP:suggestions.rs-1858 */                 Applicability::MachineApplicable,
/* FP:suggestions.rs-1859 */             );
/* FP:suggestions.rs-1860 */             return true;
/* FP:suggestions.rs-1861 */         }
/* FP:suggestions.rs-1862 */         false
/* FP:suggestions.rs-1863 */     }
/* FP:suggestions.rs-1864 */ 
/* FP:suggestions.rs-1865 */     pub(super) fn return_type_span(&self, obligation: &PredicateObligation<'tcx>) -> Option<Span> {
/* FP:suggestions.rs-1866 */         let hir::Node::Item(hir::Item { kind: hir::ItemKind::Fn { sig, .. }, .. }) =
/* FP:suggestions.rs-1867 */             self.tcx.hir_node_by_def_id(obligation.cause.body_id)
/* FP:suggestions.rs-1868 */         else {
/* FP:suggestions.rs-1869 */             return None;
/* FP:suggestions.rs-1870 */         };
/* FP:suggestions.rs-1871 */ 
/* FP:suggestions.rs-1872 */         if let hir::FnRetTy::Return(ret_ty) = sig.decl.output { Some(ret_ty.span) } else { None }
/* FP:suggestions.rs-1873 */     }
/* FP:suggestions.rs-1874 */ 
/* FP:suggestions.rs-1875 */     /// If all conditions are met to identify a returned `dyn Trait`, suggest using `impl Trait` if
/* FP:suggestions.rs-1876 */     /// applicable and signal that the error has been expanded appropriately and needs to be
/* FP:suggestions.rs-1877 */     /// emitted.
/* FP:suggestions.rs-1878 */     pub(super) fn suggest_impl_trait(
/* FP:suggestions.rs-1879 */         &self,
/* FP:suggestions.rs-1880 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-1881 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-1882 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-1883 */     ) -> bool {
/* FP:suggestions.rs-1884 */         let ObligationCauseCode::SizedReturnType = obligation.cause.code() else {
/* FP:suggestions.rs-1885 */             return false;
/* FP:suggestions.rs-1886 */         };
/* FP:suggestions.rs-1887 */         let ty::Dynamic(_, _, ty::Dyn) = trait_pred.self_ty().skip_binder().kind() else {
/* FP:suggestions.rs-1888 */             return false;
/* FP:suggestions.rs-1889 */         };
/* FP:suggestions.rs-1890 */ 
/* FP:suggestions.rs-1891 */         err.code(E0746);
/* FP:suggestions.rs-1892 */         err.primary_message("return type cannot be a trait object without pointer indirection");
/* FP:suggestions.rs-1893 */         err.children.clear();
/* FP:suggestions.rs-1894 */ 
/* FP:suggestions.rs-1895 */         let span = obligation.cause.span;
/* FP:suggestions.rs-1896 */         let body = self.tcx.hir_body_owned_by(obligation.cause.body_id);
/* FP:suggestions.rs-1897 */ 
/* FP:suggestions.rs-1898 */         let mut visitor = ReturnsVisitor::default();
/* FP:suggestions.rs-1899 */         visitor.visit_body(&body);
/* FP:suggestions.rs-1900 */ 
/* FP:suggestions.rs-1901 */         let (pre, impl_span) = if let Ok(snip) = self.tcx.sess.source_map().span_to_snippet(span)
/* FP:suggestions.rs-1902 */             && snip.starts_with("dyn ")
/* FP:suggestions.rs-1903 */         {
/* FP:suggestions.rs-1904 */             ("", span.with_hi(span.lo() + BytePos(4)))
/* FP:suggestions.rs-1905 */         } else {
/* FP:suggestions.rs-1906 */             ("dyn ", span.shrink_to_lo())
/* FP:suggestions.rs-1907 */         };
/* FP:suggestions.rs-1908 */ 
/* FP:suggestions.rs-1909 */         err.span_suggestion_verbose(
/* FP:suggestions.rs-1910 */             impl_span,
/* FP:suggestions.rs-1911 */             "consider returning an `impl Trait` instead of a `dyn Trait`",
/* FP:suggestions.rs-1912 */             "impl ",
/* FP:suggestions.rs-1913 */             Applicability::MaybeIncorrect,
/* FP:suggestions.rs-1914 */         );
/* FP:suggestions.rs-1915 */ 
/* FP:suggestions.rs-1916 */         let mut sugg = vec![
/* FP:suggestions.rs-1917 */             (span.shrink_to_lo(), format!("Box<{pre}")),
/* FP:suggestions.rs-1918 */             (span.shrink_to_hi(), ">".to_string()),
/* FP:suggestions.rs-1919 */         ];
/* FP:suggestions.rs-1920 */         sugg.extend(visitor.returns.into_iter().flat_map(|expr| {
/* FP:suggestions.rs-1921 */             let span =
/* FP:suggestions.rs-1922 */                 expr.span.find_ancestor_in_same_ctxt(obligation.cause.span).unwrap_or(expr.span);
/* FP:suggestions.rs-1923 */             if !span.can_be_used_for_suggestions() {
/* FP:suggestions.rs-1924 */                 vec![]
/* FP:suggestions.rs-1925 */             } else if let hir::ExprKind::Call(path, ..) = expr.kind
/* FP:suggestions.rs-1926 */                 && let hir::ExprKind::Path(hir::QPath::TypeRelative(ty, method)) = path.kind
/* FP:suggestions.rs-1927 */                 && method.ident.name == sym::new
/* FP:suggestions.rs-1928 */                 && let hir::TyKind::Path(hir::QPath::Resolved(.., box_path)) = ty.kind
/* FP:suggestions.rs-1929 */                 && box_path
/* FP:suggestions.rs-1930 */                     .res
/* FP:suggestions.rs-1931 */                     .opt_def_id()
/* FP:suggestions.rs-1932 */                     .is_some_and(|def_id| self.tcx.is_lang_item(def_id, LangItem::OwnedBox))
/* FP:suggestions.rs-1933 */             {
/* FP:suggestions.rs-1934 */                 // Don't box `Box::new`
/* FP:suggestions.rs-1935 */                 vec![]
/* FP:suggestions.rs-1936 */             } else {
/* FP:suggestions.rs-1937 */                 vec![
/* FP:suggestions.rs-1938 */                     (span.shrink_to_lo(), "Box::new(".to_string()),
/* FP:suggestions.rs-1939 */                     (span.shrink_to_hi(), ")".to_string()),
/* FP:suggestions.rs-1940 */                 ]
/* FP:suggestions.rs-1941 */             }
/* FP:suggestions.rs-1942 */         }));
/* FP:suggestions.rs-1943 */ 
/* FP:suggestions.rs-1944 */         err.multipart_suggestion(
/* FP:suggestions.rs-1945 */             format!(
/* FP:suggestions.rs-1946 */                 "alternatively, box the return type, and wrap all of the returned values in \
/* FP:suggestions.rs-1947 */                  `Box::new`",
/* FP:suggestions.rs-1948 */             ),
/* FP:suggestions.rs-1949 */             sugg,
/* FP:suggestions.rs-1950 */             Applicability::MaybeIncorrect,
/* FP:suggestions.rs-1951 */         );
/* FP:suggestions.rs-1952 */ 
/* FP:suggestions.rs-1953 */         true
/* FP:suggestions.rs-1954 */     }
/* FP:suggestions.rs-1955 */ 
/* FP:suggestions.rs-1956 */     pub(super) fn report_closure_arg_mismatch(
/* FP:suggestions.rs-1957 */         &self,
/* FP:suggestions.rs-1958 */         span: Span,
/* FP:suggestions.rs-1959 */         found_span: Option<Span>,
/* FP:suggestions.rs-1960 */         found: ty::TraitRef<'tcx>,
/* FP:suggestions.rs-1961 */         expected: ty::TraitRef<'tcx>,
/* FP:suggestions.rs-1962 */         cause: &ObligationCauseCode<'tcx>,
/* FP:suggestions.rs-1963 */         found_node: Option<Node<'_>>,
/* FP:suggestions.rs-1964 */         param_env: ty::ParamEnv<'tcx>,
/* FP:suggestions.rs-1965 */     ) -> Diag<'a> {
/* FP:suggestions.rs-1966 */         pub(crate) fn build_fn_sig_ty<'tcx>(
/* FP:suggestions.rs-1967 */             infcx: &InferCtxt<'tcx>,
/* FP:suggestions.rs-1968 */             trait_ref: ty::TraitRef<'tcx>,
/* FP:suggestions.rs-1969 */         ) -> Ty<'tcx> {
/* FP:suggestions.rs-1970 */             let inputs = trait_ref.args.type_at(1);
/* FP:suggestions.rs-1971 */             let sig = match inputs.kind() {
/* FP:suggestions.rs-1972 */                 ty::Tuple(inputs) if infcx.tcx.is_fn_trait(trait_ref.def_id) => {
/* FP:suggestions.rs-1973 */                     infcx.tcx.mk_fn_sig(
/* FP:suggestions.rs-1974 */                         *inputs,
/* FP:suggestions.rs-1975 */                         infcx.next_ty_var(DUMMY_SP),
/* FP:suggestions.rs-1976 */                         false,
/* FP:suggestions.rs-1977 */                         hir::Safety::Safe,
/* FP:suggestions.rs-1978 */                         ExternAbi::Rust,
/* FP:suggestions.rs-1979 */                     )
/* FP:suggestions.rs-1980 */                 }
/* FP:suggestions.rs-1981 */                 _ => infcx.tcx.mk_fn_sig(
/* FP:suggestions.rs-1982 */                     [inputs],
/* FP:suggestions.rs-1983 */                     infcx.next_ty_var(DUMMY_SP),
/* FP:suggestions.rs-1984 */                     false,
/* FP:suggestions.rs-1985 */                     hir::Safety::Safe,
/* FP:suggestions.rs-1986 */                     ExternAbi::Rust,
/* FP:suggestions.rs-1987 */                 ),
/* FP:suggestions.rs-1988 */             };
/* FP:suggestions.rs-1989 */ 
/* FP:suggestions.rs-1990 */             Ty::new_fn_ptr(infcx.tcx, ty::Binder::dummy(sig))
/* FP:suggestions.rs-1991 */         }
/* FP:suggestions.rs-1992 */ 
/* FP:suggestions.rs-1993 */         let argument_kind = match expected.self_ty().kind() {
/* FP:suggestions.rs-1994 */             ty::Closure(..) => "closure",
/* FP:suggestions.rs-1995 */             ty::Coroutine(..) => "coroutine",
/* FP:suggestions.rs-1996 */             _ => "function",
/* FP:suggestions.rs-1997 */         };
/* FP:suggestions.rs-1998 */         let mut err = struct_span_code_err!(
/* FP:suggestions.rs-1999 */             self.dcx(),
/* FP:suggestions.rs-2000 */             span,
/* FP:suggestions.rs-2001 */             E0631,
/* FP:suggestions.rs-2002 */             "type mismatch in {argument_kind} arguments",
/* FP:suggestions.rs-2003 */         );
/* FP:suggestions.rs-2004 */ 
/* FP:suggestions.rs-2005 */         err.span_label(span, "expected due to this");
/* FP:suggestions.rs-2006 */ 
/* FP:suggestions.rs-2007 */         let found_span = found_span.unwrap_or(span);
/* FP:suggestions.rs-2008 */         err.span_label(found_span, "found signature defined here");
/* FP:suggestions.rs-2009 */ 
/* FP:suggestions.rs-2010 */         let expected = build_fn_sig_ty(self, expected);
/* FP:suggestions.rs-2011 */         let found = build_fn_sig_ty(self, found);
/* FP:suggestions.rs-2012 */ 
/* FP:suggestions.rs-2013 */         let (expected_str, found_str) = self.cmp(expected, found);
/* FP:suggestions.rs-2014 */ 
/* FP:suggestions.rs-2015 */         let signature_kind = format!("{argument_kind} signature");
/* FP:suggestions.rs-2016 */         err.note_expected_found(&signature_kind, expected_str, &signature_kind, found_str);
/* FP:suggestions.rs-2017 */ 
/* FP:suggestions.rs-2018 */         self.note_conflicting_fn_args(&mut err, cause, expected, found, param_env);
/* FP:suggestions.rs-2019 */         self.note_conflicting_closure_bounds(cause, &mut err);
/* FP:suggestions.rs-2020 */ 
/* FP:suggestions.rs-2021 */         if let Some(found_node) = found_node {
/* FP:suggestions.rs-2022 */             hint_missing_borrow(self, param_env, span, found, expected, found_node, &mut err);
/* FP:suggestions.rs-2023 */         }
/* FP:suggestions.rs-2024 */ 
/* FP:suggestions.rs-2025 */         err
/* FP:suggestions.rs-2026 */     }
/* FP:suggestions.rs-2027 */ 
/* FP:suggestions.rs-2028 */     fn note_conflicting_fn_args(
/* FP:suggestions.rs-2029 */         &self,
/* FP:suggestions.rs-2030 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-2031 */         cause: &ObligationCauseCode<'tcx>,
/* FP:suggestions.rs-2032 */         expected: Ty<'tcx>,
/* FP:suggestions.rs-2033 */         found: Ty<'tcx>,
/* FP:suggestions.rs-2034 */         param_env: ty::ParamEnv<'tcx>,
/* FP:suggestions.rs-2035 */     ) {
/* FP:suggestions.rs-2036 */         let ObligationCauseCode::FunctionArg { arg_hir_id, .. } = cause else {
/* FP:suggestions.rs-2037 */             return;
/* FP:suggestions.rs-2038 */         };
/* FP:suggestions.rs-2039 */         let ty::FnPtr(sig_tys, hdr) = expected.kind() else {
/* FP:suggestions.rs-2040 */             return;
/* FP:suggestions.rs-2041 */         };
/* FP:suggestions.rs-2042 */         let expected = sig_tys.with(*hdr);
/* FP:suggestions.rs-2043 */         let ty::FnPtr(sig_tys, hdr) = found.kind() else {
/* FP:suggestions.rs-2044 */             return;
/* FP:suggestions.rs-2045 */         };
/* FP:suggestions.rs-2046 */         let found = sig_tys.with(*hdr);
/* FP:suggestions.rs-2047 */         let Node::Expr(arg) = self.tcx.hir_node(*arg_hir_id) else {
/* FP:suggestions.rs-2048 */             return;
/* FP:suggestions.rs-2049 */         };
/* FP:suggestions.rs-2050 */         let hir::ExprKind::Path(path) = arg.kind else {
/* FP:suggestions.rs-2051 */             return;
/* FP:suggestions.rs-2052 */         };
/* FP:suggestions.rs-2053 */         let expected_inputs = self.tcx.instantiate_bound_regions_with_erased(expected).inputs();
/* FP:suggestions.rs-2054 */         let found_inputs = self.tcx.instantiate_bound_regions_with_erased(found).inputs();
/* FP:suggestions.rs-2055 */         let both_tys = expected_inputs.iter().copied().zip(found_inputs.iter().copied());
/* FP:suggestions.rs-2056 */ 
/* FP:suggestions.rs-2057 */         let arg_expr = |infcx: &InferCtxt<'tcx>, name, expected: Ty<'tcx>, found: Ty<'tcx>| {
/* FP:suggestions.rs-2058 */             let (expected_ty, expected_refs) = get_deref_type_and_refs(expected);
/* FP:suggestions.rs-2059 */             let (found_ty, found_refs) = get_deref_type_and_refs(found);
/* FP:suggestions.rs-2060 */ 
/* FP:suggestions.rs-2061 */             if infcx.can_eq(param_env, found_ty, expected_ty) {
/* FP:suggestions.rs-2062 */                 if found_refs.len() == expected_refs.len()
/* FP:suggestions.rs-2063 */                     && found_refs.iter().eq(expected_refs.iter())
/* FP:suggestions.rs-2064 */                 {
/* FP:suggestions.rs-2065 */                     name
/* FP:suggestions.rs-2066 */                 } else if found_refs.len() > expected_refs.len() {
/* FP:suggestions.rs-2067 */                     let refs = &found_refs[..found_refs.len() - expected_refs.len()];
/* FP:suggestions.rs-2068 */                     if found_refs[..expected_refs.len()].iter().eq(expected_refs.iter()) {
/* FP:suggestions.rs-2069 */                         format!(
/* FP:suggestions.rs-2070 */                             "{}{name}",
/* FP:suggestions.rs-2071 */                             refs.iter()
/* FP:suggestions.rs-2072 */                                 .map(|mutbl| format!("&{}", mutbl.prefix_str()))
/* FP:suggestions.rs-2073 */                                 .collect::<Vec<_>>()
/* FP:suggestions.rs-2074 */                                 .join(""),
/* FP:suggestions.rs-2075 */                         )
/* FP:suggestions.rs-2076 */                     } else {
/* FP:suggestions.rs-2077 */                         // The refs have different mutability.
/* FP:suggestions.rs-2078 */                         format!(
/* FP:suggestions.rs-2079 */                             "{}*{name}",
/* FP:suggestions.rs-2080 */                             refs.iter()
/* FP:suggestions.rs-2081 */                                 .map(|mutbl| format!("&{}", mutbl.prefix_str()))
/* FP:suggestions.rs-2082 */                                 .collect::<Vec<_>>()
/* FP:suggestions.rs-2083 */                                 .join(""),
/* FP:suggestions.rs-2084 */                         )
/* FP:suggestions.rs-2085 */                     }
/* FP:suggestions.rs-2086 */                 } else if expected_refs.len() > found_refs.len() {
/* FP:suggestions.rs-2087 */                     format!(
/* FP:suggestions.rs-2088 */                         "{}{name}",
/* FP:suggestions.rs-2089 */                         (0..(expected_refs.len() - found_refs.len()))
/* FP:suggestions.rs-2090 */                             .map(|_| "*")
/* FP:suggestions.rs-2091 */                             .collect::<Vec<_>>()
/* FP:suggestions.rs-2092 */                             .join(""),
/* FP:suggestions.rs-2093 */                     )
/* FP:suggestions.rs-2094 */                 } else {
/* FP:suggestions.rs-2095 */                     format!(
/* FP:suggestions.rs-2096 */                         "{}{name}",
/* FP:suggestions.rs-2097 */                         found_refs
/* FP:suggestions.rs-2098 */                             .iter()
/* FP:suggestions.rs-2099 */                             .map(|mutbl| format!("&{}", mutbl.prefix_str()))
/* FP:suggestions.rs-2100 */                             .chain(found_refs.iter().map(|_| "*".to_string()))
/* FP:suggestions.rs-2101 */                             .collect::<Vec<_>>()
/* FP:suggestions.rs-2102 */                             .join(""),
/* FP:suggestions.rs-2103 */                     )
/* FP:suggestions.rs-2104 */                 }
/* FP:suggestions.rs-2105 */             } else {
/* FP:suggestions.rs-2106 */                 format!("/* {found} */")
/* FP:suggestions.rs-2107 */             }
/* FP:suggestions.rs-2108 */         };
/* FP:suggestions.rs-2109 */         let args_have_same_underlying_type = both_tys.clone().all(|(expected, found)| {
/* FP:suggestions.rs-2110 */             let (expected_ty, _) = get_deref_type_and_refs(expected);
/* FP:suggestions.rs-2111 */             let (found_ty, _) = get_deref_type_and_refs(found);
/* FP:suggestions.rs-2112 */             self.can_eq(param_env, found_ty, expected_ty)
/* FP:suggestions.rs-2113 */         });
/* FP:suggestions.rs-2114 */         let (closure_names, call_names): (Vec<_>, Vec<_>) = if args_have_same_underlying_type
/* FP:suggestions.rs-2115 */             && !expected_inputs.is_empty()
/* FP:suggestions.rs-2116 */             && expected_inputs.len() == found_inputs.len()
/* FP:suggestions.rs-2117 */             && let Some(typeck) = &self.typeck_results
/* FP:suggestions.rs-2118 */             && let Res::Def(res_kind, fn_def_id) = typeck.qpath_res(&path, *arg_hir_id)
/* FP:suggestions.rs-2119 */             && res_kind.is_fn_like()
/* FP:suggestions.rs-2120 */         {
/* FP:suggestions.rs-2121 */             let closure: Vec<_> = self
/* FP:suggestions.rs-2122 */                 .tcx
/* FP:suggestions.rs-2123 */                 .fn_arg_idents(fn_def_id)
/* FP:suggestions.rs-2124 */                 .iter()
/* FP:suggestions.rs-2125 */                 .enumerate()
/* FP:suggestions.rs-2126 */                 .map(|(i, ident)| {
/* FP:suggestions.rs-2127 */                     if let Some(ident) = ident
/* FP:suggestions.rs-2128 */                         && !matches!(ident, Ident { name: kw::Underscore | kw::SelfLower, .. })
/* FP:suggestions.rs-2129 */                     {
/* FP:suggestions.rs-2130 */                         format!("{ident}")
/* FP:suggestions.rs-2131 */                     } else {
/* FP:suggestions.rs-2132 */                         format!("arg{i}")
/* FP:suggestions.rs-2133 */                     }
/* FP:suggestions.rs-2134 */                 })
/* FP:suggestions.rs-2135 */                 .collect();
/* FP:suggestions.rs-2136 */             let args = closure
/* FP:suggestions.rs-2137 */                 .iter()
/* FP:suggestions.rs-2138 */                 .zip(both_tys)
/* FP:suggestions.rs-2139 */                 .map(|(name, (expected, found))| {
/* FP:suggestions.rs-2140 */                     arg_expr(self.infcx, name.to_owned(), expected, found)
/* FP:suggestions.rs-2141 */                 })
/* FP:suggestions.rs-2142 */                 .collect();
/* FP:suggestions.rs-2143 */             (closure, args)
/* FP:suggestions.rs-2144 */         } else {
/* FP:suggestions.rs-2145 */             let closure_args = expected_inputs
/* FP:suggestions.rs-2146 */                 .iter()
/* FP:suggestions.rs-2147 */                 .enumerate()
/* FP:suggestions.rs-2148 */                 .map(|(i, _)| format!("arg{i}"))
/* FP:suggestions.rs-2149 */                 .collect::<Vec<_>>();
/* FP:suggestions.rs-2150 */             let call_args = both_tys
/* FP:suggestions.rs-2151 */                 .enumerate()
/* FP:suggestions.rs-2152 */                 .map(|(i, (expected, found))| {
/* FP:suggestions.rs-2153 */                     arg_expr(self.infcx, format!("arg{i}"), expected, found)
/* FP:suggestions.rs-2154 */                 })
/* FP:suggestions.rs-2155 */                 .collect::<Vec<_>>();
/* FP:suggestions.rs-2156 */             (closure_args, call_args)
/* FP:suggestions.rs-2157 */         };
/* FP:suggestions.rs-2158 */         let closure_names: Vec<_> = closure_names
/* FP:suggestions.rs-2159 */             .into_iter()
/* FP:suggestions.rs-2160 */             .zip(expected_inputs.iter())
/* FP:suggestions.rs-2161 */             .map(|(name, ty)| {
/* FP:suggestions.rs-2162 */                 format!(
/* FP:suggestions.rs-2163 */                     "{name}{}",
/* FP:suggestions.rs-2164 */                     if ty.has_infer_types() {
/* FP:suggestions.rs-2165 */                         String::new()
/* FP:suggestions.rs-2166 */                     } else if ty.references_error() {
/* FP:suggestions.rs-2167 */                         ": /* type */".to_string()
/* FP:suggestions.rs-2168 */                     } else {
/* FP:suggestions.rs-2169 */                         format!(": {ty}")
/* FP:suggestions.rs-2170 */                     }
/* FP:suggestions.rs-2171 */                 )
/* FP:suggestions.rs-2172 */             })
/* FP:suggestions.rs-2173 */             .collect();
/* FP:suggestions.rs-2174 */         err.multipart_suggestion(
/* FP:suggestions.rs-2175 */             "consider wrapping the function in a closure",
/* FP:suggestions.rs-2176 */             vec![
/* FP:suggestions.rs-2177 */                 (arg.span.shrink_to_lo(), format!("|{}| ", closure_names.join(", "))),
/* FP:suggestions.rs-2178 */                 (arg.span.shrink_to_hi(), format!("({})", call_names.join(", "))),
/* FP:suggestions.rs-2179 */             ],
/* FP:suggestions.rs-2180 */             Applicability::MaybeIncorrect,
/* FP:suggestions.rs-2181 */         );
/* FP:suggestions.rs-2182 */     }
/* FP:suggestions.rs-2183 */ 
/* FP:suggestions.rs-2184 */     // Add a note if there are two `Fn`-family bounds that have conflicting argument
/* FP:suggestions.rs-2185 */     // requirements, which will always cause a closure to have a type error.
/* FP:suggestions.rs-2186 */     fn note_conflicting_closure_bounds(
/* FP:suggestions.rs-2187 */         &self,
/* FP:suggestions.rs-2188 */         cause: &ObligationCauseCode<'tcx>,
/* FP:suggestions.rs-2189 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-2190 */     ) {
/* FP:suggestions.rs-2191 */         // First, look for an `WhereClauseInExpr`, which means we can get
/* FP:suggestions.rs-2192 */         // the uninstantiated predicate list of the called function. And check
/* FP:suggestions.rs-2193 */         // that the predicate that we failed to satisfy is a `Fn`-like trait.
/* FP:suggestions.rs-2194 */         if let ObligationCauseCode::WhereClauseInExpr(def_id, _, _, idx) = cause
/* FP:suggestions.rs-2195 */             && let predicates = self.tcx.predicates_of(def_id).instantiate_identity(self.tcx)
/* FP:suggestions.rs-2196 */             && let Some(pred) = predicates.predicates.get(*idx)
/* FP:suggestions.rs-2197 */             && let ty::ClauseKind::Trait(trait_pred) = pred.kind().skip_binder()
/* FP:suggestions.rs-2198 */             && self.tcx.is_fn_trait(trait_pred.def_id())
/* FP:suggestions.rs-2199 */         {
/* FP:suggestions.rs-2200 */             let expected_self =
/* FP:suggestions.rs-2201 */                 self.tcx.anonymize_bound_vars(pred.kind().rebind(trait_pred.self_ty()));
/* FP:suggestions.rs-2202 */             let expected_args =
/* FP:suggestions.rs-2203 */                 self.tcx.anonymize_bound_vars(pred.kind().rebind(trait_pred.trait_ref.args));
/* FP:suggestions.rs-2204 */ 
/* FP:suggestions.rs-2205 */             // Find another predicate whose self-type is equal to the expected self type,
/* FP:suggestions.rs-2206 */             // but whose args don't match.
/* FP:suggestions.rs-2207 */             let other_pred = predicates.into_iter().enumerate().find(|(other_idx, (pred, _))| {
/* FP:suggestions.rs-2208 */                 match pred.kind().skip_binder() {
/* FP:suggestions.rs-2209 */                     ty::ClauseKind::Trait(trait_pred)
/* FP:suggestions.rs-2210 */                         if self.tcx.is_fn_trait(trait_pred.def_id())
/* FP:suggestions.rs-2211 */                             && other_idx != idx
/* FP:suggestions.rs-2212 */                             // Make sure that the self type matches
/* FP:suggestions.rs-2213 */                             // (i.e. constraining this closure)
/* FP:suggestions.rs-2214 */                             && expected_self
/* FP:suggestions.rs-2215 */                                 == self.tcx.anonymize_bound_vars(
/* FP:suggestions.rs-2216 */                                     pred.kind().rebind(trait_pred.self_ty()),
/* FP:suggestions.rs-2217 */                                 )
/* FP:suggestions.rs-2218 */                             // But the args don't match (i.e. incompatible args)
/* FP:suggestions.rs-2219 */                             && expected_args
/* FP:suggestions.rs-2220 */                                 != self.tcx.anonymize_bound_vars(
/* FP:suggestions.rs-2221 */                                     pred.kind().rebind(trait_pred.trait_ref.args),
/* FP:suggestions.rs-2222 */                                 ) =>
/* FP:suggestions.rs-2223 */                     {
/* FP:suggestions.rs-2224 */                         true
/* FP:suggestions.rs-2225 */                     }
/* FP:suggestions.rs-2226 */                     _ => false,
/* FP:suggestions.rs-2227 */                 }
/* FP:suggestions.rs-2228 */             });
/* FP:suggestions.rs-2229 */             // If we found one, then it's very likely the cause of the error.
/* FP:suggestions.rs-2230 */             if let Some((_, (_, other_pred_span))) = other_pred {
/* FP:suggestions.rs-2231 */                 err.span_note(
/* FP:suggestions.rs-2232 */                     other_pred_span,
/* FP:suggestions.rs-2233 */                     "closure inferred to have a different signature due to this bound",
/* FP:suggestions.rs-2234 */                 );
/* FP:suggestions.rs-2235 */             }
/* FP:suggestions.rs-2236 */         }
/* FP:suggestions.rs-2237 */     }
/* FP:suggestions.rs-2238 */ 
/* FP:suggestions.rs-2239 */     pub(super) fn suggest_fully_qualified_path(
/* FP:suggestions.rs-2240 */         &self,
/* FP:suggestions.rs-2241 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-2242 */         item_def_id: DefId,
/* FP:suggestions.rs-2243 */         span: Span,
/* FP:suggestions.rs-2244 */         trait_ref: DefId,
/* FP:suggestions.rs-2245 */     ) {
/* FP:suggestions.rs-2246 */         if let Some(assoc_item) = self.tcx.opt_associated_item(item_def_id)
/* FP:suggestions.rs-2247 */             && let ty::AssocKind::Const { .. } | ty::AssocKind::Type { .. } = assoc_item.kind
/* FP:suggestions.rs-2248 */         {
/* FP:suggestions.rs-2249 */             err.note(format!(
/* FP:suggestions.rs-2250 */                 "{}s cannot be accessed directly on a `trait`, they can only be \
/* FP:suggestions.rs-2251 */                         accessed through a specific `impl`",
/* FP:suggestions.rs-2252 */                 self.tcx.def_kind_descr(assoc_item.as_def_kind(), item_def_id)
/* FP:suggestions.rs-2253 */             ));
/* FP:suggestions.rs-2254 */ 
/* FP:suggestions.rs-2255 */             if !assoc_item.is_impl_trait_in_trait() {
/* FP:suggestions.rs-2256 */                 err.span_suggestion_verbose(
/* FP:suggestions.rs-2257 */                     span,
/* FP:suggestions.rs-2258 */                     "use the fully qualified path to an implementation",
/* FP:suggestions.rs-2259 */                     format!(
/* FP:suggestions.rs-2260 */                         "<Type as {}>::{}",
/* FP:suggestions.rs-2261 */                         self.tcx.def_path_str(trait_ref),
/* FP:suggestions.rs-2262 */                         assoc_item.name()
/* FP:suggestions.rs-2263 */                     ),
/* FP:suggestions.rs-2264 */                     Applicability::HasPlaceholders,
/* FP:suggestions.rs-2265 */                 );
/* FP:suggestions.rs-2266 */             }
/* FP:suggestions.rs-2267 */         }
/* FP:suggestions.rs-2268 */     }
/* FP:suggestions.rs-2269 */ 
/* FP:suggestions.rs-2270 */     /// Adds an async-await specific note to the diagnostic when the future does not implement
/* FP:suggestions.rs-2271 */     /// an auto trait because of a captured type.
/* FP:suggestions.rs-2272 */     ///
/* FP:suggestions.rs-2273 */     /// ```text
/* FP:suggestions.rs-2274 */     /// note: future does not implement `Qux` as this value is used across an await
/* FP:suggestions.rs-2275 */     ///   --> $DIR/issue-64130-3-other.rs:17:5
/* FP:suggestions.rs-2276 */     ///    |
/* FP:suggestions.rs-2277 */     /// LL |     let x = Foo;
/* FP:suggestions.rs-2278 */     ///    |         - has type `Foo`
/* FP:suggestions.rs-2279 */     /// LL |     baz().await;
/* FP:suggestions.rs-2280 */     ///    |     ^^^^^^^^^^^ await occurs here, with `x` maybe used later
/* FP:suggestions.rs-2281 */     /// LL | }
/* FP:suggestions.rs-2282 */     ///    | - `x` is later dropped here
/* FP:suggestions.rs-2283 */     /// ```
/* FP:suggestions.rs-2284 */     ///
/* FP:suggestions.rs-2285 */     /// When the diagnostic does not implement `Send` or `Sync` specifically, then the diagnostic
/* FP:suggestions.rs-2286 */     /// is "replaced" with a different message and a more specific error.
/* FP:suggestions.rs-2287 */     ///
/* FP:suggestions.rs-2288 */     /// ```text
/* FP:suggestions.rs-2289 */     /// error: future cannot be sent between threads safely
/* FP:suggestions.rs-2290 */     ///   --> $DIR/issue-64130-2-send.rs:21:5
/* FP:suggestions.rs-2291 */     ///    |
/* FP:suggestions.rs-2292 */     /// LL | fn is_send<T: Send>(t: T) { }
/* FP:suggestions.rs-2293 */     ///    |               ---- required by this bound in `is_send`
/* FP:suggestions.rs-2294 */     /// ...
/* FP:suggestions.rs-2295 */     /// LL |     is_send(bar());
/* FP:suggestions.rs-2296 */     ///    |     ^^^^^^^ future returned by `bar` is not send
/* FP:suggestions.rs-2297 */     ///    |
/* FP:suggestions.rs-2298 */     ///    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not
/* FP:suggestions.rs-2299 */     ///            implemented for `Foo`
/* FP:suggestions.rs-2300 */     /// note: future is not send as this value is used across an await
/* FP:suggestions.rs-2301 */     ///   --> $DIR/issue-64130-2-send.rs:15:5
/* FP:suggestions.rs-2302 */     ///    |
/* FP:suggestions.rs-2303 */     /// LL |     let x = Foo;
/* FP:suggestions.rs-2304 */     ///    |         - has type `Foo`
/* FP:suggestions.rs-2305 */     /// LL |     baz().await;
/* FP:suggestions.rs-2306 */     ///    |     ^^^^^^^^^^^ await occurs here, with `x` maybe used later
/* FP:suggestions.rs-2307 */     /// LL | }
/* FP:suggestions.rs-2308 */     ///    | - `x` is later dropped here
/* FP:suggestions.rs-2309 */     /// ```
/* FP:suggestions.rs-2310 */     ///
/* FP:suggestions.rs-2311 */     /// Returns `true` if an async-await specific note was added to the diagnostic.
/* FP:suggestions.rs-2312 */     #[instrument(level = "debug", skip_all, fields(?obligation.predicate, ?obligation.cause.span))]
/* FP:suggestions.rs-2313 */     pub fn maybe_note_obligation_cause_for_async_await<G: EmissionGuarantee>(
/* FP:suggestions.rs-2314 */         &self,
/* FP:suggestions.rs-2315 */         err: &mut Diag<'_, G>,
/* FP:suggestions.rs-2316 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-2317 */     ) -> bool {
/* FP:suggestions.rs-2318 */         // Attempt to detect an async-await error by looking at the obligation causes, looking
/* FP:suggestions.rs-2319 */         // for a coroutine to be present.
/* FP:suggestions.rs-2320 */         //
/* FP:suggestions.rs-2321 */         // When a future does not implement a trait because of a captured type in one of the
/* FP:suggestions.rs-2322 */         // coroutines somewhere in the call stack, then the result is a chain of obligations.
/* FP:suggestions.rs-2323 */         //
/* FP:suggestions.rs-2324 */         // Given an `async fn` A that calls an `async fn` B which captures a non-send type and that
/* FP:suggestions.rs-2325 */         // future is passed as an argument to a function C which requires a `Send` type, then the
/* FP:suggestions.rs-2326 */         // chain looks something like this:
/* FP:suggestions.rs-2327 */         //
/* FP:suggestions.rs-2328 */         // - `BuiltinDerivedObligation` with a coroutine witness (B)
/* FP:suggestions.rs-2329 */         // - `BuiltinDerivedObligation` with a coroutine (B)
/* FP:suggestions.rs-2330 */         // - `BuiltinDerivedObligation` with `impl std::future::Future` (B)
/* FP:suggestions.rs-2331 */         // - `BuiltinDerivedObligation` with a coroutine witness (A)
/* FP:suggestions.rs-2332 */         // - `BuiltinDerivedObligation` with a coroutine (A)
/* FP:suggestions.rs-2333 */         // - `BuiltinDerivedObligation` with `impl std::future::Future` (A)
/* FP:suggestions.rs-2334 */         // - `BindingObligation` with `impl_send` (Send requirement)
/* FP:suggestions.rs-2335 */         //
/* FP:suggestions.rs-2336 */         // The first obligation in the chain is the most useful and has the coroutine that captured
/* FP:suggestions.rs-2337 */         // the type. The last coroutine (`outer_coroutine` below) has information about where the
/* FP:suggestions.rs-2338 */         // bound was introduced. At least one coroutine should be present for this diagnostic to be
/* FP:suggestions.rs-2339 */         // modified.
/* FP:suggestions.rs-2340 */         let (mut trait_ref, mut target_ty) = match obligation.predicate.kind().skip_binder() {
/* FP:suggestions.rs-2341 */             ty::PredicateKind::Clause(ty::ClauseKind::Trait(p)) => (Some(p), Some(p.self_ty())),
/* FP:suggestions.rs-2342 */             _ => (None, None),
/* FP:suggestions.rs-2343 */         };
/* FP:suggestions.rs-2344 */         let mut coroutine = None;
/* FP:suggestions.rs-2345 */         let mut outer_coroutine = None;
/* FP:suggestions.rs-2346 */         let mut next_code = Some(obligation.cause.code());
/* FP:suggestions.rs-2347 */ 
/* FP:suggestions.rs-2348 */         let mut seen_upvar_tys_infer_tuple = false;
/* FP:suggestions.rs-2349 */ 
/* FP:suggestions.rs-2350 */         while let Some(code) = next_code {
/* FP:suggestions.rs-2351 */             debug!(?code);
/* FP:suggestions.rs-2352 */             match code {
/* FP:suggestions.rs-2353 */                 ObligationCauseCode::FunctionArg { parent_code, .. } => {
/* FP:suggestions.rs-2354 */                     next_code = Some(parent_code);
/* FP:suggestions.rs-2355 */                 }
/* FP:suggestions.rs-2356 */                 ObligationCauseCode::ImplDerived(cause) => {
/* FP:suggestions.rs-2357 */                     let ty = cause.derived.parent_trait_pred.skip_binder().self_ty();
/* FP:suggestions.rs-2358 */                     debug!(
/* FP:suggestions.rs-2359 */                         parent_trait_ref = ?cause.derived.parent_trait_pred,
/* FP:suggestions.rs-2360 */                         self_ty.kind = ?ty.kind(),
/* FP:suggestions.rs-2361 */                         "ImplDerived",
/* FP:suggestions.rs-2362 */                     );
/* FP:suggestions.rs-2363 */ 
/* FP:suggestions.rs-2364 */                     match *ty.kind() {
/* FP:suggestions.rs-2365 */                         ty::Coroutine(did, ..) | ty::CoroutineWitness(did, _) => {
/* FP:suggestions.rs-2366 */                             coroutine = coroutine.or(Some(did));
/* FP:suggestions.rs-2367 */                             outer_coroutine = Some(did);
/* FP:suggestions.rs-2368 */                         }
/* FP:suggestions.rs-2369 */                         ty::Tuple(_) if !seen_upvar_tys_infer_tuple => {
/* FP:suggestions.rs-2370 */                             // By introducing a tuple of upvar types into the chain of obligations
/* FP:suggestions.rs-2371 */                             // of a coroutine, the first non-coroutine item is now the tuple itself,
/* FP:suggestions.rs-2372 */                             // we shall ignore this.
/* FP:suggestions.rs-2373 */ 
/* FP:suggestions.rs-2374 */                             seen_upvar_tys_infer_tuple = true;
/* FP:suggestions.rs-2375 */                         }
/* FP:suggestions.rs-2376 */                         _ if coroutine.is_none() => {
/* FP:suggestions.rs-2377 */                             trait_ref = Some(cause.derived.parent_trait_pred.skip_binder());
/* FP:suggestions.rs-2378 */                             target_ty = Some(ty);
/* FP:suggestions.rs-2379 */                         }
/* FP:suggestions.rs-2380 */                         _ => {}
/* FP:suggestions.rs-2381 */                     }
/* FP:suggestions.rs-2382 */ 
/* FP:suggestions.rs-2383 */                     next_code = Some(&cause.derived.parent_code);
/* FP:suggestions.rs-2384 */                 }
/* FP:suggestions.rs-2385 */                 ObligationCauseCode::WellFormedDerived(derived_obligation)
/* FP:suggestions.rs-2386 */                 | ObligationCauseCode::BuiltinDerived(derived_obligation) => {
/* FP:suggestions.rs-2387 */                     let ty = derived_obligation.parent_trait_pred.skip_binder().self_ty();
/* FP:suggestions.rs-2388 */                     debug!(
/* FP:suggestions.rs-2389 */                         parent_trait_ref = ?derived_obligation.parent_trait_pred,
/* FP:suggestions.rs-2390 */                         self_ty.kind = ?ty.kind(),
/* FP:suggestions.rs-2391 */                     );
/* FP:suggestions.rs-2392 */ 
/* FP:suggestions.rs-2393 */                     match *ty.kind() {
/* FP:suggestions.rs-2394 */                         ty::Coroutine(did, ..) | ty::CoroutineWitness(did, ..) => {
/* FP:suggestions.rs-2395 */                             coroutine = coroutine.or(Some(did));
/* FP:suggestions.rs-2396 */                             outer_coroutine = Some(did);
/* FP:suggestions.rs-2397 */                         }
/* FP:suggestions.rs-2398 */                         ty::Tuple(_) if !seen_upvar_tys_infer_tuple => {
/* FP:suggestions.rs-2399 */                             // By introducing a tuple of upvar types into the chain of obligations
/* FP:suggestions.rs-2400 */                             // of a coroutine, the first non-coroutine item is now the tuple itself,
/* FP:suggestions.rs-2401 */                             // we shall ignore this.
/* FP:suggestions.rs-2402 */ 
/* FP:suggestions.rs-2403 */                             seen_upvar_tys_infer_tuple = true;
/* FP:suggestions.rs-2404 */                         }
/* FP:suggestions.rs-2405 */                         _ if coroutine.is_none() => {
/* FP:suggestions.rs-2406 */                             trait_ref = Some(derived_obligation.parent_trait_pred.skip_binder());
/* FP:suggestions.rs-2407 */                             target_ty = Some(ty);
/* FP:suggestions.rs-2408 */                         }
/* FP:suggestions.rs-2409 */                         _ => {}
/* FP:suggestions.rs-2410 */                     }
/* FP:suggestions.rs-2411 */ 
/* FP:suggestions.rs-2412 */                     next_code = Some(&derived_obligation.parent_code);
/* FP:suggestions.rs-2413 */                 }
/* FP:suggestions.rs-2414 */                 _ => break,
/* FP:suggestions.rs-2415 */             }
/* FP:suggestions.rs-2416 */         }
/* FP:suggestions.rs-2417 */ 
/* FP:suggestions.rs-2418 */         // Only continue if a coroutine was found.
/* FP:suggestions.rs-2419 */         debug!(?coroutine, ?trait_ref, ?target_ty);
/* FP:suggestions.rs-2420 */         let (Some(coroutine_did), Some(trait_ref), Some(target_ty)) =
/* FP:suggestions.rs-2421 */             (coroutine, trait_ref, target_ty)
/* FP:suggestions.rs-2422 */         else {
/* FP:suggestions.rs-2423 */             return false;
/* FP:suggestions.rs-2424 */         };
/* FP:suggestions.rs-2425 */ 
/* FP:suggestions.rs-2426 */         let span = self.tcx.def_span(coroutine_did);
/* FP:suggestions.rs-2427 */ 
/* FP:suggestions.rs-2428 */         let coroutine_did_root = self.tcx.typeck_root_def_id(coroutine_did);
/* FP:suggestions.rs-2429 */         debug!(
/* FP:suggestions.rs-2430 */             ?coroutine_did,
/* FP:suggestions.rs-2431 */             ?coroutine_did_root,
/* FP:suggestions.rs-2432 */             typeck_results.hir_owner = ?self.typeck_results.as_ref().map(|t| t.hir_owner),
/* FP:suggestions.rs-2433 */             ?span,
/* FP:suggestions.rs-2434 */         );
/* FP:suggestions.rs-2435 */ 
/* FP:suggestions.rs-2436 */         let coroutine_body =
/* FP:suggestions.rs-2437 */             coroutine_did.as_local().and_then(|def_id| self.tcx.hir_maybe_body_owned_by(def_id));
/* FP:suggestions.rs-2438 */         let mut visitor = AwaitsVisitor::default();
/* FP:suggestions.rs-2439 */         if let Some(body) = coroutine_body {
/* FP:suggestions.rs-2440 */             visitor.visit_body(&body);
/* FP:suggestions.rs-2441 */         }
/* FP:suggestions.rs-2442 */         debug!(awaits = ?visitor.awaits);
/* FP:suggestions.rs-2443 */ 
/* FP:suggestions.rs-2444 */         // Look for a type inside the coroutine interior that matches the target type to get
/* FP:suggestions.rs-2445 */         // a span.
/* FP:suggestions.rs-2446 */         let target_ty_erased = self.tcx.erase_and_anonymize_regions(target_ty);
/* FP:suggestions.rs-2447 */         let ty_matches = |ty| -> bool {
/* FP:suggestions.rs-2448 */             // Careful: the regions for types that appear in the
/* FP:suggestions.rs-2449 */             // coroutine interior are not generally known, so we
/* FP:suggestions.rs-2450 */             // want to erase them when comparing (and anyway,
/* FP:suggestions.rs-2451 */             // `Send` and other bounds are generally unaffected by
/* FP:suggestions.rs-2452 */             // the choice of region). When erasing regions, we
/* FP:suggestions.rs-2453 */             // also have to erase late-bound regions. This is
/* FP:suggestions.rs-2454 */             // because the types that appear in the coroutine
/* FP:suggestions.rs-2455 */             // interior generally contain "bound regions" to
/* FP:suggestions.rs-2456 */             // represent regions that are part of the suspended
/* FP:suggestions.rs-2457 */             // coroutine frame. Bound regions are preserved by
/* FP:suggestions.rs-2458 */             // `erase_and_anonymize_regions` and so we must also call
/* FP:suggestions.rs-2459 */             // `instantiate_bound_regions_with_erased`.
/* FP:suggestions.rs-2460 */             let ty_erased = self.tcx.instantiate_bound_regions_with_erased(ty);
/* FP:suggestions.rs-2461 */             let ty_erased = self.tcx.erase_and_anonymize_regions(ty_erased);
/* FP:suggestions.rs-2462 */             let eq = ty_erased == target_ty_erased;
/* FP:suggestions.rs-2463 */             debug!(?ty_erased, ?target_ty_erased, ?eq);
/* FP:suggestions.rs-2464 */             eq
/* FP:suggestions.rs-2465 */         };
/* FP:suggestions.rs-2466 */ 
/* FP:suggestions.rs-2467 */         // Get the typeck results from the infcx if the coroutine is the function we are currently
/* FP:suggestions.rs-2468 */         // type-checking; otherwise, get them by performing a query. This is needed to avoid
/* FP:suggestions.rs-2469 */         // cycles. If we can't use resolved types because the coroutine comes from another crate,
/* FP:suggestions.rs-2470 */         // we still provide a targeted error but without all the relevant spans.
/* FP:suggestions.rs-2471 */         let coroutine_data = match &self.typeck_results {
/* FP:suggestions.rs-2472 */             Some(t) if t.hir_owner.to_def_id() == coroutine_did_root => CoroutineData(t),
/* FP:suggestions.rs-2473 */             _ if coroutine_did.is_local() => {
/* FP:suggestions.rs-2474 */                 CoroutineData(self.tcx.typeck(coroutine_did.expect_local()))
/* FP:suggestions.rs-2475 */             }
/* FP:suggestions.rs-2476 */             _ => return false,
/* FP:suggestions.rs-2477 */         };
/* FP:suggestions.rs-2478 */ 
/* FP:suggestions.rs-2479 */         let coroutine_within_in_progress_typeck = match &self.typeck_results {
/* FP:suggestions.rs-2480 */             Some(t) => t.hir_owner.to_def_id() == coroutine_did_root,
/* FP:suggestions.rs-2481 */             _ => false,
/* FP:suggestions.rs-2482 */         };
/* FP:suggestions.rs-2483 */ 
/* FP:suggestions.rs-2484 */         let mut interior_or_upvar_span = None;
/* FP:suggestions.rs-2485 */ 
/* FP:suggestions.rs-2486 */         let from_awaited_ty = coroutine_data.get_from_await_ty(visitor, self.tcx, ty_matches);
/* FP:suggestions.rs-2487 */         debug!(?from_awaited_ty);
/* FP:suggestions.rs-2488 */ 
/* FP:suggestions.rs-2489 */         // Avoid disclosing internal information to downstream crates.
/* FP:suggestions.rs-2490 */         if coroutine_did.is_local()
/* FP:suggestions.rs-2491 */             // Try to avoid cycles.
/* FP:suggestions.rs-2492 */             && !coroutine_within_in_progress_typeck
/* FP:suggestions.rs-2493 */             && let Some(coroutine_info) = self.tcx.mir_coroutine_witnesses(coroutine_did)
/* FP:suggestions.rs-2494 */         {
/* FP:suggestions.rs-2495 */             debug!(?coroutine_info);
/* FP:suggestions.rs-2496 */             'find_source: for (variant, source_info) in
/* FP:suggestions.rs-2497 */                 coroutine_info.variant_fields.iter().zip(&coroutine_info.variant_source_info)
/* FP:suggestions.rs-2498 */             {
/* FP:suggestions.rs-2499 */                 debug!(?variant);
/* FP:suggestions.rs-2500 */                 for &local in variant {
/* FP:suggestions.rs-2501 */                     let decl = &coroutine_info.field_tys[local];
/* FP:suggestions.rs-2502 */                     debug!(?decl);
/* FP:suggestions.rs-2503 */                     if ty_matches(ty::Binder::dummy(decl.ty)) && !decl.ignore_for_traits {
/* FP:suggestions.rs-2504 */                         interior_or_upvar_span = Some(CoroutineInteriorOrUpvar::Interior(
/* FP:suggestions.rs-2505 */                             decl.source_info.span,
/* FP:suggestions.rs-2506 */                             Some((source_info.span, from_awaited_ty)),
/* FP:suggestions.rs-2507 */                         ));
/* FP:suggestions.rs-2508 */                         break 'find_source;
/* FP:suggestions.rs-2509 */                     }
/* FP:suggestions.rs-2510 */                 }
/* FP:suggestions.rs-2511 */             }
/* FP:suggestions.rs-2512 */         }
/* FP:suggestions.rs-2513 */ 
/* FP:suggestions.rs-2514 */         if interior_or_upvar_span.is_none() {
/* FP:suggestions.rs-2515 */             interior_or_upvar_span =
/* FP:suggestions.rs-2516 */                 coroutine_data.try_get_upvar_span(self, coroutine_did, ty_matches);
/* FP:suggestions.rs-2517 */         }
/* FP:suggestions.rs-2518 */ 
/* FP:suggestions.rs-2519 */         if interior_or_upvar_span.is_none() && !coroutine_did.is_local() {
/* FP:suggestions.rs-2520 */             interior_or_upvar_span = Some(CoroutineInteriorOrUpvar::Interior(span, None));
/* FP:suggestions.rs-2521 */         }
/* FP:suggestions.rs-2522 */ 
/* FP:suggestions.rs-2523 */         debug!(?interior_or_upvar_span);
/* FP:suggestions.rs-2524 */         if let Some(interior_or_upvar_span) = interior_or_upvar_span {
/* FP:suggestions.rs-2525 */             let is_async = self.tcx.coroutine_is_async(coroutine_did);
/* FP:suggestions.rs-2526 */             self.note_obligation_cause_for_async_await(
/* FP:suggestions.rs-2527 */                 err,
/* FP:suggestions.rs-2528 */                 interior_or_upvar_span,
/* FP:suggestions.rs-2529 */                 is_async,
/* FP:suggestions.rs-2530 */                 outer_coroutine,
/* FP:suggestions.rs-2531 */                 trait_ref,
/* FP:suggestions.rs-2532 */                 target_ty,
/* FP:suggestions.rs-2533 */                 obligation,
/* FP:suggestions.rs-2534 */                 next_code,
/* FP:suggestions.rs-2535 */             );
/* FP:suggestions.rs-2536 */             true
/* FP:suggestions.rs-2537 */         } else {
/* FP:suggestions.rs-2538 */             false
/* FP:suggestions.rs-2539 */         }
/* FP:suggestions.rs-2540 */     }
/* FP:suggestions.rs-2541 */ 
/* FP:suggestions.rs-2542 */     /// Unconditionally adds the diagnostic note described in
/* FP:suggestions.rs-2543 */     /// `maybe_note_obligation_cause_for_async_await`'s documentation comment.
/* FP:suggestions.rs-2544 */     #[instrument(level = "debug", skip_all)]
/* FP:suggestions.rs-2545 */     fn note_obligation_cause_for_async_await<G: EmissionGuarantee>(
/* FP:suggestions.rs-2546 */         &self,
/* FP:suggestions.rs-2547 */         err: &mut Diag<'_, G>,
/* FP:suggestions.rs-2548 */         interior_or_upvar_span: CoroutineInteriorOrUpvar,
/* FP:suggestions.rs-2549 */         is_async: bool,
/* FP:suggestions.rs-2550 */         outer_coroutine: Option<DefId>,
/* FP:suggestions.rs-2551 */         trait_pred: ty::TraitPredicate<'tcx>,
/* FP:suggestions.rs-2552 */         target_ty: Ty<'tcx>,
/* FP:suggestions.rs-2553 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-2554 */         next_code: Option<&ObligationCauseCode<'tcx>>,
/* FP:suggestions.rs-2555 */     ) {
/* FP:suggestions.rs-2556 */         let source_map = self.tcx.sess.source_map();
/* FP:suggestions.rs-2557 */ 
/* FP:suggestions.rs-2558 */         let (await_or_yield, an_await_or_yield) =
/* FP:suggestions.rs-2559 */             if is_async { ("await", "an await") } else { ("yield", "a yield") };
/* FP:suggestions.rs-2560 */         let future_or_coroutine = if is_async { "future" } else { "coroutine" };
/* FP:suggestions.rs-2561 */ 
/* FP:suggestions.rs-2562 */         // Special case the primary error message when send or sync is the trait that was
/* FP:suggestions.rs-2563 */         // not implemented.
/* FP:suggestions.rs-2564 */         let trait_explanation = if let Some(name @ (sym::Send | sym::Sync)) =
/* FP:suggestions.rs-2565 */             self.tcx.get_diagnostic_name(trait_pred.def_id())
/* FP:suggestions.rs-2566 */         {
/* FP:suggestions.rs-2567 */             let (trait_name, trait_verb) =
/* FP:suggestions.rs-2568 */                 if name == sym::Send { ("`Send`", "sent") } else { ("`Sync`", "shared") };
/* FP:suggestions.rs-2569 */ 
/* FP:suggestions.rs-2570 */             err.code = None;
/* FP:suggestions.rs-2571 */             err.primary_message(format!(
/* FP:suggestions.rs-2572 */                 "{future_or_coroutine} cannot be {trait_verb} between threads safely"
/* FP:suggestions.rs-2573 */             ));
/* FP:suggestions.rs-2574 */ 
/* FP:suggestions.rs-2575 */             let original_span = err.span.primary_span().unwrap();
/* FP:suggestions.rs-2576 */             let mut span = MultiSpan::from_span(original_span);
/* FP:suggestions.rs-2577 */ 
/* FP:suggestions.rs-2578 */             let message = outer_coroutine
/* FP:suggestions.rs-2579 */                 .and_then(|coroutine_did| {
/* FP:suggestions.rs-2580 */                     Some(match self.tcx.coroutine_kind(coroutine_did).unwrap() {
/* FP:suggestions.rs-2581 */                         CoroutineKind::Coroutine(_) => format!("coroutine is not {trait_name}"),
/* FP:suggestions.rs-2582 */                         CoroutineKind::Desugared(
/* FP:suggestions.rs-2583 */                             CoroutineDesugaring::Async,
/* FP:suggestions.rs-2584 */                             CoroutineSource::Fn,
/* FP:suggestions.rs-2585 */                         ) => self
/* FP:suggestions.rs-2586 */                             .tcx
/* FP:suggestions.rs-2587 */                             .parent(coroutine_did)
/* FP:suggestions.rs-2588 */                             .as_local()
/* FP:suggestions.rs-2589 */                             .map(|parent_did| self.tcx.local_def_id_to_hir_id(parent_did))
/* FP:suggestions.rs-2590 */                             .and_then(|parent_hir_id| self.tcx.hir_opt_name(parent_hir_id))
/* FP:suggestions.rs-2591 */                             .map(|name| {
/* FP:suggestions.rs-2592 */                                 format!("future returned by `{name}` is not {trait_name}")
/* FP:suggestions.rs-2593 */                             })?,
/* FP:suggestions.rs-2594 */                         CoroutineKind::Desugared(
/* FP:suggestions.rs-2595 */                             CoroutineDesugaring::Async,
/* FP:suggestions.rs-2596 */                             CoroutineSource::Block,
/* FP:suggestions.rs-2597 */                         ) => {
/* FP:suggestions.rs-2598 */                             format!("future created by async block is not {trait_name}")
/* FP:suggestions.rs-2599 */                         }
/* FP:suggestions.rs-2600 */                         CoroutineKind::Desugared(
/* FP:suggestions.rs-2601 */                             CoroutineDesugaring::Async,
/* FP:suggestions.rs-2602 */                             CoroutineSource::Closure,
/* FP:suggestions.rs-2603 */                         ) => {
/* FP:suggestions.rs-2604 */                             format!("future created by async closure is not {trait_name}")
/* FP:suggestions.rs-2605 */                         }
/* FP:suggestions.rs-2606 */                         CoroutineKind::Desugared(
/* FP:suggestions.rs-2607 */                             CoroutineDesugaring::AsyncGen,
/* FP:suggestions.rs-2608 */                             CoroutineSource::Fn,
/* FP:suggestions.rs-2609 */                         ) => self
/* FP:suggestions.rs-2610 */                             .tcx
/* FP:suggestions.rs-2611 */                             .parent(coroutine_did)
/* FP:suggestions.rs-2612 */                             .as_local()
/* FP:suggestions.rs-2613 */                             .map(|parent_did| self.tcx.local_def_id_to_hir_id(parent_did))
/* FP:suggestions.rs-2614 */                             .and_then(|parent_hir_id| self.tcx.hir_opt_name(parent_hir_id))
/* FP:suggestions.rs-2615 */                             .map(|name| {
/* FP:suggestions.rs-2616 */                                 format!("async iterator returned by `{name}` is not {trait_name}")
/* FP:suggestions.rs-2617 */                             })?,
/* FP:suggestions.rs-2618 */                         CoroutineKind::Desugared(
/* FP:suggestions.rs-2619 */                             CoroutineDesugaring::AsyncGen,
/* FP:suggestions.rs-2620 */                             CoroutineSource::Block,
/* FP:suggestions.rs-2621 */                         ) => {
/* FP:suggestions.rs-2622 */                             format!("async iterator created by async gen block is not {trait_name}")
/* FP:suggestions.rs-2623 */                         }
/* FP:suggestions.rs-2624 */                         CoroutineKind::Desugared(
/* FP:suggestions.rs-2625 */                             CoroutineDesugaring::AsyncGen,
/* FP:suggestions.rs-2626 */                             CoroutineSource::Closure,
/* FP:suggestions.rs-2627 */                         ) => {
/* FP:suggestions.rs-2628 */                             format!(
/* FP:suggestions.rs-2629 */                                 "async iterator created by async gen closure is not {trait_name}"
/* FP:suggestions.rs-2630 */                             )
/* FP:suggestions.rs-2631 */                         }
/* FP:suggestions.rs-2632 */                         CoroutineKind::Desugared(CoroutineDesugaring::Gen, CoroutineSource::Fn) => {
/* FP:suggestions.rs-2633 */                             self.tcx
/* FP:suggestions.rs-2634 */                                 .parent(coroutine_did)
/* FP:suggestions.rs-2635 */                                 .as_local()
/* FP:suggestions.rs-2636 */                                 .map(|parent_did| self.tcx.local_def_id_to_hir_id(parent_did))
/* FP:suggestions.rs-2637 */                                 .and_then(|parent_hir_id| self.tcx.hir_opt_name(parent_hir_id))
/* FP:suggestions.rs-2638 */                                 .map(|name| {
/* FP:suggestions.rs-2639 */                                     format!("iterator returned by `{name}` is not {trait_name}")
/* FP:suggestions.rs-2640 */                                 })?
/* FP:suggestions.rs-2641 */                         }
/* FP:suggestions.rs-2642 */                         CoroutineKind::Desugared(
/* FP:suggestions.rs-2643 */                             CoroutineDesugaring::Gen,
/* FP:suggestions.rs-2644 */                             CoroutineSource::Block,
/* FP:suggestions.rs-2645 */                         ) => {
/* FP:suggestions.rs-2646 */                             format!("iterator created by gen block is not {trait_name}")
/* FP:suggestions.rs-2647 */                         }
/* FP:suggestions.rs-2648 */                         CoroutineKind::Desugared(
/* FP:suggestions.rs-2649 */                             CoroutineDesugaring::Gen,
/* FP:suggestions.rs-2650 */                             CoroutineSource::Closure,
/* FP:suggestions.rs-2651 */                         ) => {
/* FP:suggestions.rs-2652 */                             format!("iterator created by gen closure is not {trait_name}")
/* FP:suggestions.rs-2653 */                         }
/* FP:suggestions.rs-2654 */                     })
/* FP:suggestions.rs-2655 */                 })
/* FP:suggestions.rs-2656 */                 .unwrap_or_else(|| format!("{future_or_coroutine} is not {trait_name}"));
/* FP:suggestions.rs-2657 */ 
/* FP:suggestions.rs-2658 */             span.push_span_label(original_span, message);
/* FP:suggestions.rs-2659 */             err.span(span);
/* FP:suggestions.rs-2660 */ 
/* FP:suggestions.rs-2661 */             format!("is not {trait_name}")
/* FP:suggestions.rs-2662 */         } else {
/* FP:suggestions.rs-2663 */             format!("does not implement `{}`", trait_pred.print_modifiers_and_trait_path())
/* FP:suggestions.rs-2664 */         };
/* FP:suggestions.rs-2665 */ 
/* FP:suggestions.rs-2666 */         let mut explain_yield = |interior_span: Span, yield_span: Span| {
/* FP:suggestions.rs-2667 */             let mut span = MultiSpan::from_span(yield_span);
/* FP:suggestions.rs-2668 */             let snippet = match source_map.span_to_snippet(interior_span) {
/* FP:suggestions.rs-2669 */                 // #70935: If snippet contains newlines, display "the value" instead
/* FP:suggestions.rs-2670 */                 // so that we do not emit complex diagnostics.
/* FP:suggestions.rs-2671 */                 Ok(snippet) if !snippet.contains('\n') => format!("`{snippet}`"),
/* FP:suggestions.rs-2672 */                 _ => "the value".to_string(),
/* FP:suggestions.rs-2673 */             };
/* FP:suggestions.rs-2674 */             // note: future is not `Send` as this value is used across an await
/* FP:suggestions.rs-2675 */             //   --> $DIR/issue-70935-complex-spans.rs:13:9
/* FP:suggestions.rs-2676 */             //    |
/* FP:suggestions.rs-2677 */             // LL |            baz(|| async {
/* FP:suggestions.rs-2678 */             //    |  ______________-
/* FP:suggestions.rs-2679 */             //    | |
/* FP:suggestions.rs-2680 */             //    | |
/* FP:suggestions.rs-2681 */             // LL | |              foo(tx.clone());
/* FP:suggestions.rs-2682 */             // LL | |          }).await;
/* FP:suggestions.rs-2683 */             //    | |          - ^^^^^^ await occurs here, with value maybe used later
/* FP:suggestions.rs-2684 */             //    | |__________|
/* FP:suggestions.rs-2685 */             //    |            has type `closure` which is not `Send`
/* FP:suggestions.rs-2686 */             // note: value is later dropped here
/* FP:suggestions.rs-2687 */             // LL | |          }).await;
/* FP:suggestions.rs-2688 */             //    | |                  ^
/* FP:suggestions.rs-2689 */             //
/* FP:suggestions.rs-2690 */             span.push_span_label(
/* FP:suggestions.rs-2691 */                 yield_span,
/* FP:suggestions.rs-2692 */                 format!("{await_or_yield} occurs here, with {snippet} maybe used later"),
/* FP:suggestions.rs-2693 */             );
/* FP:suggestions.rs-2694 */             span.push_span_label(
/* FP:suggestions.rs-2695 */                 interior_span,
/* FP:suggestions.rs-2696 */                 format!("has type `{target_ty}` which {trait_explanation}"),
/* FP:suggestions.rs-2697 */             );
/* FP:suggestions.rs-2698 */             err.span_note(
/* FP:suggestions.rs-2699 */                 span,
/* FP:suggestions.rs-2700 */                 format!("{future_or_coroutine} {trait_explanation} as this value is used across {an_await_or_yield}"),
/* FP:suggestions.rs-2701 */             );
/* FP:suggestions.rs-2702 */         };
/* FP:suggestions.rs-2703 */         match interior_or_upvar_span {
/* FP:suggestions.rs-2704 */             CoroutineInteriorOrUpvar::Interior(interior_span, interior_extra_info) => {
/* FP:suggestions.rs-2705 */                 if let Some((yield_span, from_awaited_ty)) = interior_extra_info {
/* FP:suggestions.rs-2706 */                     if let Some(await_span) = from_awaited_ty {
/* FP:suggestions.rs-2707 */                         // The type causing this obligation is one being awaited at await_span.
/* FP:suggestions.rs-2708 */                         let mut span = MultiSpan::from_span(await_span);
/* FP:suggestions.rs-2709 */                         span.push_span_label(
/* FP:suggestions.rs-2710 */                             await_span,
/* FP:suggestions.rs-2711 */                             format!(
/* FP:suggestions.rs-2712 */                                 "await occurs here on type `{target_ty}`, which {trait_explanation}"
/* FP:suggestions.rs-2713 */                             ),
/* FP:suggestions.rs-2714 */                         );
/* FP:suggestions.rs-2715 */                         err.span_note(
/* FP:suggestions.rs-2716 */                             span,
/* FP:suggestions.rs-2717 */                             format!(
/* FP:suggestions.rs-2718 */                                 "future {trait_explanation} as it awaits another future which {trait_explanation}"
/* FP:suggestions.rs-2719 */                             ),
/* FP:suggestions.rs-2720 */                         );
/* FP:suggestions.rs-2721 */                     } else {
/* FP:suggestions.rs-2722 */                         // Look at the last interior type to get a span for the `.await`.
/* FP:suggestions.rs-2723 */                         explain_yield(interior_span, yield_span);
/* FP:suggestions.rs-2724 */                     }
/* FP:suggestions.rs-2725 */                 }
/* FP:suggestions.rs-2726 */             }
/* FP:suggestions.rs-2727 */             CoroutineInteriorOrUpvar::Upvar(upvar_span) => {
/* FP:suggestions.rs-2728 */                 // `Some((ref_ty, is_mut))` if `target_ty` is `&T` or `&mut T` and fails to impl `Send`
/* FP:suggestions.rs-2729 */                 let non_send = match target_ty.kind() {
/* FP:suggestions.rs-2730 */                     ty::Ref(_, ref_ty, mutability) => match self.evaluate_obligation(obligation) {
/* FP:suggestions.rs-2731 */                         Ok(eval) if !eval.may_apply() => Some((ref_ty, mutability.is_mut())),
/* FP:suggestions.rs-2732 */                         _ => None,
/* FP:suggestions.rs-2733 */                     },
/* FP:suggestions.rs-2734 */                     _ => None,
/* FP:suggestions.rs-2735 */                 };
/* FP:suggestions.rs-2736 */ 
/* FP:suggestions.rs-2737 */                 let (span_label, span_note) = match non_send {
/* FP:suggestions.rs-2738 */                     // if `target_ty` is `&T` or `&mut T` and fails to impl `Send`,
/* FP:suggestions.rs-2739 */                     // include suggestions to make `T: Sync` so that `&T: Send`,
/* FP:suggestions.rs-2740 */                     // or to make `T: Send` so that `&mut T: Send`
/* FP:suggestions.rs-2741 */                     Some((ref_ty, is_mut)) => {
/* FP:suggestions.rs-2742 */                         let ref_ty_trait = if is_mut { "Send" } else { "Sync" };
/* FP:suggestions.rs-2743 */                         let ref_kind = if is_mut { "&mut" } else { "&" };
/* FP:suggestions.rs-2744 */                         (
/* FP:suggestions.rs-2745 */                             format!(
/* FP:suggestions.rs-2746 */                                 "has type `{target_ty}` which {trait_explanation}, because `{ref_ty}` is not `{ref_ty_trait}`"
/* FP:suggestions.rs-2747 */                             ),
/* FP:suggestions.rs-2748 */                             format!(
/* FP:suggestions.rs-2749 */                                 "captured value {trait_explanation} because `{ref_kind}` references cannot be sent unless their referent is `{ref_ty_trait}`"
/* FP:suggestions.rs-2750 */                             ),
/* FP:suggestions.rs-2751 */                         )
/* FP:suggestions.rs-2752 */                     }
/* FP:suggestions.rs-2753 */                     None => (
/* FP:suggestions.rs-2754 */                         format!("has type `{target_ty}` which {trait_explanation}"),
/* FP:suggestions.rs-2755 */                         format!("captured value {trait_explanation}"),
/* FP:suggestions.rs-2756 */                     ),
/* FP:suggestions.rs-2757 */                 };
/* FP:suggestions.rs-2758 */ 
/* FP:suggestions.rs-2759 */                 let mut span = MultiSpan::from_span(upvar_span);
/* FP:suggestions.rs-2760 */                 span.push_span_label(upvar_span, span_label);
/* FP:suggestions.rs-2761 */                 err.span_note(span, span_note);
/* FP:suggestions.rs-2762 */             }
/* FP:suggestions.rs-2763 */         }
/* FP:suggestions.rs-2764 */ 
/* FP:suggestions.rs-2765 */         // Add a note for the item obligation that remains - normally a note pointing to the
/* FP:suggestions.rs-2766 */         // bound that introduced the obligation (e.g. `T: Send`).
/* FP:suggestions.rs-2767 */         debug!(?next_code);
/* FP:suggestions.rs-2768 */         self.note_obligation_cause_code(
/* FP:suggestions.rs-2769 */             obligation.cause.body_id,
/* FP:suggestions.rs-2770 */             err,
/* FP:suggestions.rs-2771 */             obligation.predicate,
/* FP:suggestions.rs-2772 */             obligation.param_env,
/* FP:suggestions.rs-2773 */             next_code.unwrap(),
/* FP:suggestions.rs-2774 */             &mut Vec::new(),
/* FP:suggestions.rs-2775 */             &mut Default::default(),
/* FP:suggestions.rs-2776 */         );
/* FP:suggestions.rs-2777 */     }
/* FP:suggestions.rs-2778 */ 
/* FP:suggestions.rs-2779 */     pub(super) fn note_obligation_cause_code<G: EmissionGuarantee, T>(
/* FP:suggestions.rs-2780 */         &self,
/* FP:suggestions.rs-2781 */         body_id: LocalDefId,
/* FP:suggestions.rs-2782 */         err: &mut Diag<'_, G>,
/* FP:suggestions.rs-2783 */         predicate: T,
/* FP:suggestions.rs-2784 */         param_env: ty::ParamEnv<'tcx>,
/* FP:suggestions.rs-2785 */         cause_code: &ObligationCauseCode<'tcx>,
/* FP:suggestions.rs-2786 */         obligated_types: &mut Vec<Ty<'tcx>>,
/* FP:suggestions.rs-2787 */         seen_requirements: &mut FxHashSet<DefId>,
/* FP:suggestions.rs-2788 */     ) where
/* FP:suggestions.rs-2789 */         T: Upcast<TyCtxt<'tcx>, ty::Predicate<'tcx>>,
/* FP:suggestions.rs-2790 */     {
/* FP:suggestions.rs-2791 */         let tcx = self.tcx;
/* FP:suggestions.rs-2792 */         let predicate = predicate.upcast(tcx);
/* FP:suggestions.rs-2793 */         let suggest_remove_deref = |err: &mut Diag<'_, G>, expr: &hir::Expr<'_>| {
/* FP:suggestions.rs-2794 */             if let Some(pred) = predicate.as_trait_clause()
/* FP:suggestions.rs-2795 */                 && tcx.is_lang_item(pred.def_id(), LangItem::Sized)
/* FP:suggestions.rs-2796 */                 && let hir::ExprKind::Unary(hir::UnOp::Deref, inner) = expr.kind
/* FP:suggestions.rs-2797 */             {
/* FP:suggestions.rs-2798 */                 err.span_suggestion_verbose(
/* FP:suggestions.rs-2799 */                     expr.span.until(inner.span),
/* FP:suggestions.rs-2800 */                     "references are always `Sized`, even if they point to unsized data; consider \
/* FP:suggestions.rs-2801 */                      not dereferencing the expression",
/* FP:suggestions.rs-2802 */                     String::new(),
/* FP:suggestions.rs-2803 */                     Applicability::MaybeIncorrect,
/* FP:suggestions.rs-2804 */                 );
/* FP:suggestions.rs-2805 */             }
/* FP:suggestions.rs-2806 */         };
/* FP:suggestions.rs-2807 */         match *cause_code {
/* FP:suggestions.rs-2808 */             ObligationCauseCode::ExprAssignable
/* FP:suggestions.rs-2809 */             | ObligationCauseCode::MatchExpressionArm { .. }
/* FP:suggestions.rs-2810 */             | ObligationCauseCode::Pattern { .. }
/* FP:suggestions.rs-2811 */             | ObligationCauseCode::IfExpression { .. }
/* FP:suggestions.rs-2812 */             | ObligationCauseCode::IfExpressionWithNoElse
/* FP:suggestions.rs-2813 */             | ObligationCauseCode::MainFunctionType
/* FP:suggestions.rs-2814 */             | ObligationCauseCode::LangFunctionType(_)
/* FP:suggestions.rs-2815 */             | ObligationCauseCode::IntrinsicType
/* FP:suggestions.rs-2816 */             | ObligationCauseCode::MethodReceiver
/* FP:suggestions.rs-2817 */             | ObligationCauseCode::ReturnNoExpression
/* FP:suggestions.rs-2818 */             | ObligationCauseCode::Misc
/* FP:suggestions.rs-2819 */             | ObligationCauseCode::WellFormed(..)
/* FP:suggestions.rs-2820 */             | ObligationCauseCode::MatchImpl(..)
/* FP:suggestions.rs-2821 */             | ObligationCauseCode::ReturnValue(_)
/* FP:suggestions.rs-2822 */             | ObligationCauseCode::BlockTailExpression(..)
/* FP:suggestions.rs-2823 */             | ObligationCauseCode::AwaitableExpr(_)
/* FP:suggestions.rs-2824 */             | ObligationCauseCode::ForLoopIterator
/* FP:suggestions.rs-2825 */             | ObligationCauseCode::QuestionMark
/* FP:suggestions.rs-2826 */             | ObligationCauseCode::CheckAssociatedTypeBounds { .. }
/* FP:suggestions.rs-2827 */             | ObligationCauseCode::LetElse
/* FP:suggestions.rs-2828 */             | ObligationCauseCode::UnOp { .. }
/* FP:suggestions.rs-2829 */             | ObligationCauseCode::BinOp { .. }
/* FP:suggestions.rs-2830 */             | ObligationCauseCode::AscribeUserTypeProvePredicate(..)
/* FP:suggestions.rs-2831 */             | ObligationCauseCode::AlwaysApplicableImpl
/* FP:suggestions.rs-2832 */             | ObligationCauseCode::ConstParam(_)
/* FP:suggestions.rs-2833 */             | ObligationCauseCode::ReferenceOutlivesReferent(..)
/* FP:suggestions.rs-2834 */             | ObligationCauseCode::ObjectTypeBound(..) => {}
/* FP:suggestions.rs-2835 */             ObligationCauseCode::RustCall => {
/* FP:suggestions.rs-2836 */                 if let Some(pred) = predicate.as_trait_clause()
/* FP:suggestions.rs-2837 */                     && tcx.is_lang_item(pred.def_id(), LangItem::Sized)
/* FP:suggestions.rs-2838 */                 {
/* FP:suggestions.rs-2839 */                     err.note("argument required to be sized due to `extern \"rust-call\"` ABI");
/* FP:suggestions.rs-2840 */                 }
/* FP:suggestions.rs-2841 */             }
/* FP:suggestions.rs-2842 */             ObligationCauseCode::SliceOrArrayElem => {
/* FP:suggestions.rs-2843 */                 err.note("slice and array elements must have `Sized` type");
/* FP:suggestions.rs-2844 */             }
/* FP:suggestions.rs-2845 */             ObligationCauseCode::ArrayLen(array_ty) => {
/* FP:suggestions.rs-2846 */                 err.note(format!("the length of array `{array_ty}` must be type `usize`"));
/* FP:suggestions.rs-2847 */             }
/* FP:suggestions.rs-2848 */             ObligationCauseCode::TupleElem => {
/* FP:suggestions.rs-2849 */                 err.note("only the last element of a tuple may have a dynamically sized type");
/* FP:suggestions.rs-2850 */             }
/* FP:suggestions.rs-2851 */             ObligationCauseCode::DynCompatible(span) => {
/* FP:suggestions.rs-2852 */                 err.multipart_suggestion(
/* FP:suggestions.rs-2853 */                     "you might have meant to use `Self` to refer to the implementing type",
/* FP:suggestions.rs-2854 */                     vec![(span, "Self".into())],
/* FP:suggestions.rs-2855 */                     Applicability::MachineApplicable,
/* FP:suggestions.rs-2856 */                 );
/* FP:suggestions.rs-2857 */             }
/* FP:suggestions.rs-2858 */             ObligationCauseCode::WhereClause(item_def_id, span)
/* FP:suggestions.rs-2859 */             | ObligationCauseCode::WhereClauseInExpr(item_def_id, span, ..)
/* FP:suggestions.rs-2860 */             | ObligationCauseCode::HostEffectInExpr(item_def_id, span, ..)
/* FP:suggestions.rs-2861 */                 if !span.is_dummy() =>
/* FP:suggestions.rs-2862 */             {
/* FP:suggestions.rs-2863 */                 if let ObligationCauseCode::WhereClauseInExpr(_, _, hir_id, pos) = &cause_code {
/* FP:suggestions.rs-2864 */                     if let Node::Expr(expr) = tcx.parent_hir_node(*hir_id)
/* FP:suggestions.rs-2865 */                         && let hir::ExprKind::Call(_, args) = expr.kind
/* FP:suggestions.rs-2866 */                         && let Some(expr) = args.get(*pos)
/* FP:suggestions.rs-2867 */                     {
/* FP:suggestions.rs-2868 */                         suggest_remove_deref(err, &expr);
/* FP:suggestions.rs-2869 */                     } else if let Node::Expr(expr) = self.tcx.hir_node(*hir_id)
/* FP:suggestions.rs-2870 */                         && let hir::ExprKind::MethodCall(_, _, args, _) = expr.kind
/* FP:suggestions.rs-2871 */                         && let Some(expr) = args.get(*pos)
/* FP:suggestions.rs-2872 */                     {
/* FP:suggestions.rs-2873 */                         suggest_remove_deref(err, &expr);
/* FP:suggestions.rs-2874 */                     }
/* FP:suggestions.rs-2875 */                 }
/* FP:suggestions.rs-2876 */                 let item_name = tcx.def_path_str(item_def_id);
/* FP:suggestions.rs-2877 */                 let short_item_name = with_forced_trimmed_paths!(tcx.def_path_str(item_def_id));
/* FP:suggestions.rs-2878 */                 let mut multispan = MultiSpan::from(span);
/* FP:suggestions.rs-2879 */                 let sm = tcx.sess.source_map();
/* FP:suggestions.rs-2880 */                 if let Some(ident) = tcx.opt_item_ident(item_def_id) {
/* FP:suggestions.rs-2881 */                     let same_line =
/* FP:suggestions.rs-2882 */                         match (sm.lookup_line(ident.span.hi()), sm.lookup_line(span.lo())) {
/* FP:suggestions.rs-2883 */                             (Ok(l), Ok(r)) => l.line == r.line,
/* FP:suggestions.rs-2884 */                             _ => true,
/* FP:suggestions.rs-2885 */                         };
/* FP:suggestions.rs-2886 */                     if ident.span.is_visible(sm) && !ident.span.overlaps(span) && !same_line {
/* FP:suggestions.rs-2887 */                         multispan.push_span_label(
/* FP:suggestions.rs-2888 */                             ident.span,
/* FP:suggestions.rs-2889 */                             format!(
/* FP:suggestions.rs-2890 */                                 "required by a bound in this {}",
/* FP:suggestions.rs-2891 */                                 tcx.def_kind(item_def_id).descr(item_def_id)
/* FP:suggestions.rs-2892 */                             ),
/* FP:suggestions.rs-2893 */                         );
/* FP:suggestions.rs-2894 */                     }
/* FP:suggestions.rs-2895 */                 }
/* FP:suggestions.rs-2896 */                 let mut a = "a";
/* FP:suggestions.rs-2897 */                 let mut this = "this bound";
/* FP:suggestions.rs-2898 */                 let mut note = None;
/* FP:suggestions.rs-2899 */                 let mut help = None;
/* FP:suggestions.rs-2900 */                 if let ty::PredicateKind::Clause(clause) = predicate.kind().skip_binder() {
/* FP:suggestions.rs-2901 */                     match clause {
/* FP:suggestions.rs-2902 */                         ty::ClauseKind::Trait(trait_pred) => {
/* FP:suggestions.rs-2903 */                             let def_id = trait_pred.def_id();
/* FP:suggestions.rs-2904 */                             let visible_item = if let Some(local) = def_id.as_local() {
/* FP:suggestions.rs-2905 */                                 let ty = trait_pred.self_ty();
/* FP:suggestions.rs-2906 */                                 // when `TraitA: TraitB` and `S` only impl TraitA,
/* FP:suggestions.rs-2907 */                                 // we check if `TraitB` can be reachable from `S`
/* FP:suggestions.rs-2908 */                                 // to determine whether to note `TraitA` is sealed trait.
/* FP:suggestions.rs-2909 */                                 if let ty::Adt(adt, _) = ty.kind() {
/* FP:suggestions.rs-2910 */                                     let visibilities = &tcx.resolutions(()).effective_visibilities;
/* FP:suggestions.rs-2911 */                                     visibilities.effective_vis(local).is_none_or(|v| {
/* FP:suggestions.rs-2912 */                                         v.at_level(Level::Reexported)
/* FP:suggestions.rs-2913 */                                             .is_accessible_from(adt.did(), tcx)
/* FP:suggestions.rs-2914 */                                     })
/* FP:suggestions.rs-2915 */                                 } else {
/* FP:suggestions.rs-2916 */                                     // FIXME(xizheyin): if the type is not ADT, we should not suggest it
/* FP:suggestions.rs-2917 */                                     true
/* FP:suggestions.rs-2918 */                                 }
/* FP:suggestions.rs-2919 */                             } else {
/* FP:suggestions.rs-2920 */                                 // Check for foreign traits being reachable.
/* FP:suggestions.rs-2921 */                                 tcx.visible_parent_map(()).get(&def_id).is_some()
/* FP:suggestions.rs-2922 */                             };
/* FP:suggestions.rs-2923 */                             if tcx.is_lang_item(def_id, LangItem::Sized) {
/* FP:suggestions.rs-2924 */                                 // Check if this is an implicit bound, even in foreign crates.
/* FP:suggestions.rs-2925 */                                 if tcx
/* FP:suggestions.rs-2926 */                                     .generics_of(item_def_id)
/* FP:suggestions.rs-2927 */                                     .own_params
/* FP:suggestions.rs-2928 */                                     .iter()
/* FP:suggestions.rs-2929 */                                     .any(|param| tcx.def_span(param.def_id) == span)
/* FP:suggestions.rs-2930 */                                 {
/* FP:suggestions.rs-2931 */                                     a = "an implicit `Sized`";
/* FP:suggestions.rs-2932 */                                     this =
/* FP:suggestions.rs-2933 */                                         "the implicit `Sized` requirement on this type parameter";
/* FP:suggestions.rs-2934 */                                 }
/* FP:suggestions.rs-2935 */                                 if let Some(hir::Node::TraitItem(hir::TraitItem {
/* FP:suggestions.rs-2936 */                                     generics,
/* FP:suggestions.rs-2937 */                                     kind: hir::TraitItemKind::Type(bounds, None),
/* FP:suggestions.rs-2938 */                                     ..
/* FP:suggestions.rs-2939 */                                 })) = tcx.hir_get_if_local(item_def_id)
/* FP:suggestions.rs-2940 */                                     // Do not suggest relaxing if there is an explicit `Sized` obligation.
/* FP:suggestions.rs-2941 */                                     && !bounds.iter()
/* FP:suggestions.rs-2942 */                                         .filter_map(|bound| bound.trait_ref())
/* FP:suggestions.rs-2943 */                                         .any(|tr| tr.trait_def_id().is_some_and(|def_id| tcx.is_lang_item(def_id, LangItem::Sized)))
/* FP:suggestions.rs-2944 */                                 {
/* FP:suggestions.rs-2945 */                                     let (span, separator) = if let [.., last] = bounds {
/* FP:suggestions.rs-2946 */                                         (last.span().shrink_to_hi(), " +")
/* FP:suggestions.rs-2947 */                                     } else {
/* FP:suggestions.rs-2948 */                                         (generics.span.shrink_to_hi(), ":")
/* FP:suggestions.rs-2949 */                                     };
/* FP:suggestions.rs-2950 */                                     err.span_suggestion_verbose(
/* FP:suggestions.rs-2951 */                                         span,
/* FP:suggestions.rs-2952 */                                         "consider relaxing the implicit `Sized` restriction",
/* FP:suggestions.rs-2953 */                                         format!("{separator} ?Sized"),
/* FP:suggestions.rs-2954 */                                         Applicability::MachineApplicable,
/* FP:suggestions.rs-2955 */                                     );
/* FP:suggestions.rs-2956 */                                 }
/* FP:suggestions.rs-2957 */                             }
/* FP:suggestions.rs-2958 */                             if let DefKind::Trait = tcx.def_kind(item_def_id)
/* FP:suggestions.rs-2959 */                                 && !visible_item
/* FP:suggestions.rs-2960 */                             {
/* FP:suggestions.rs-2961 */                                 note = Some(format!(
/* FP:suggestions.rs-2962 */                                     "`{short_item_name}` is a \"sealed trait\", because to implement it \
/* FP:suggestions.rs-2963 */                                     you also need to implement `{}`, which is not accessible; this is \
/* FP:suggestions.rs-2964 */                                     usually done to force you to use one of the provided types that \
/* FP:suggestions.rs-2965 */                                     already implement it",
/* FP:suggestions.rs-2966 */                                     with_no_trimmed_paths!(tcx.def_path_str(def_id)),
/* FP:suggestions.rs-2967 */                                 ));
/* FP:suggestions.rs-2968 */                                 let impls_of = tcx.trait_impls_of(def_id);
/* FP:suggestions.rs-2969 */                                 let impls = impls_of
/* FP:suggestions.rs-2970 */                                     .non_blanket_impls()
/* FP:suggestions.rs-2971 */                                     .values()
/* FP:suggestions.rs-2972 */                                     .flatten()
/* FP:suggestions.rs-2973 */                                     .chain(impls_of.blanket_impls().iter())
/* FP:suggestions.rs-2974 */                                     .collect::<Vec<_>>();
/* FP:suggestions.rs-2975 */                                 if !impls.is_empty() {
/* FP:suggestions.rs-2976 */                                     let len = impls.len();
/* FP:suggestions.rs-2977 */                                     let mut types = impls
/* FP:suggestions.rs-2978 */                                         .iter()
/* FP:suggestions.rs-2979 */                                         .map(|t| {
/* FP:suggestions.rs-2980 */                                             with_no_trimmed_paths!(format!(
/* FP:suggestions.rs-2981 */                                                 "  {}",
/* FP:suggestions.rs-2982 */                                                 tcx.type_of(*t).instantiate_identity(),
/* FP:suggestions.rs-2983 */                                             ))
/* FP:suggestions.rs-2984 */                                         })
/* FP:suggestions.rs-2985 */                                         .collect::<Vec<_>>();
/* FP:suggestions.rs-2986 */                                     let post = if types.len() > 9 {
/* FP:suggestions.rs-2987 */                                         types.truncate(8);
/* FP:suggestions.rs-2988 */                                         format!("\nand {} others", len - 8)
/* FP:suggestions.rs-2989 */                                     } else {
/* FP:suggestions.rs-2990 */                                         String::new()
/* FP:suggestions.rs-2991 */                                     };
/* FP:suggestions.rs-2992 */                                     help = Some(format!(
/* FP:suggestions.rs-2993 */                                         "the following type{} implement{} the trait:\n{}{post}",
/* FP:suggestions.rs-2994 */                                         pluralize!(len),
/* FP:suggestions.rs-2995 */                                         if len == 1 { "s" } else { "" },
/* FP:suggestions.rs-2996 */                                         types.join("\n"),
/* FP:suggestions.rs-2997 */                                     ));
/* FP:suggestions.rs-2998 */                                 }
/* FP:suggestions.rs-2999 */                             }
/* FP:suggestions.rs-3000 */                         }
/* FP:suggestions.rs-3001 */                         ty::ClauseKind::ConstArgHasType(..) => {
/* FP:suggestions.rs-3002 */                             let descr =
/* FP:suggestions.rs-3003 */                                 format!("required by a const generic parameter in `{item_name}`");
/* FP:suggestions.rs-3004 */                             if span.is_visible(sm) {
/* FP:suggestions.rs-3005 */                                 let msg = format!(
/* FP:suggestions.rs-3006 */                                     "required by this const generic parameter in `{short_item_name}`"
/* FP:suggestions.rs-3007 */                                 );
/* FP:suggestions.rs-3008 */                                 multispan.push_span_label(span, msg);
/* FP:suggestions.rs-3009 */                                 err.span_note(multispan, descr);
/* FP:suggestions.rs-3010 */                             } else {
/* FP:suggestions.rs-3011 */                                 err.span_note(tcx.def_span(item_def_id), descr);
/* FP:suggestions.rs-3012 */                             }
/* FP:suggestions.rs-3013 */                             return;
/* FP:suggestions.rs-3014 */                         }
/* FP:suggestions.rs-3015 */                         _ => (),
/* FP:suggestions.rs-3016 */                     }
/* FP:suggestions.rs-3017 */                 }
/* FP:suggestions.rs-3018 */ 
/* FP:suggestions.rs-3019 */                 // If this is from a format string literal desugaring,
/* FP:suggestions.rs-3020 */                 // we've already said "required by this formatting parameter"
/* FP:suggestions.rs-3021 */                 let is_in_fmt_lit = if let Some(s) = err.span.primary_span() {
/* FP:suggestions.rs-3022 */                     matches!(s.desugaring_kind(), Some(DesugaringKind::FormatLiteral { .. }))
/* FP:suggestions.rs-3023 */                 } else {
/* FP:suggestions.rs-3024 */                     false
/* FP:suggestions.rs-3025 */                 };
/* FP:suggestions.rs-3026 */                 if !is_in_fmt_lit {
/* FP:suggestions.rs-3027 */                     let descr = format!("required by {a} bound in `{item_name}`");
/* FP:suggestions.rs-3028 */                     if span.is_visible(sm) {
/* FP:suggestions.rs-3029 */                         let msg = format!("required by {this} in `{short_item_name}`");
/* FP:suggestions.rs-3030 */                         multispan.push_span_label(span, msg);
/* FP:suggestions.rs-3031 */                         err.span_note(multispan, descr);
/* FP:suggestions.rs-3032 */                     } else {
/* FP:suggestions.rs-3033 */                         err.span_note(tcx.def_span(item_def_id), descr);
/* FP:suggestions.rs-3034 */                     }
/* FP:suggestions.rs-3035 */                 }
/* FP:suggestions.rs-3036 */                 if let Some(note) = note {
/* FP:suggestions.rs-3037 */                     err.note(note);
/* FP:suggestions.rs-3038 */                 }
/* FP:suggestions.rs-3039 */                 if let Some(help) = help {
/* FP:suggestions.rs-3040 */                     err.help(help);
/* FP:suggestions.rs-3041 */                 }
/* FP:suggestions.rs-3042 */             }
/* FP:suggestions.rs-3043 */             ObligationCauseCode::WhereClause(..)
/* FP:suggestions.rs-3044 */             | ObligationCauseCode::WhereClauseInExpr(..)
/* FP:suggestions.rs-3045 */             | ObligationCauseCode::HostEffectInExpr(..) => {
/* FP:suggestions.rs-3046 */                 // We hold the `DefId` of the item introducing the obligation, but displaying it
/* FP:suggestions.rs-3047 */                 // doesn't add user usable information. It always point at an associated item.
/* FP:suggestions.rs-3048 */             }
/* FP:suggestions.rs-3049 */             ObligationCauseCode::OpaqueTypeBound(span, definition_def_id) => {
/* FP:suggestions.rs-3050 */                 err.span_note(span, "required by a bound in an opaque type");
/* FP:suggestions.rs-3051 */                 if let Some(definition_def_id) = definition_def_id
/* FP:suggestions.rs-3052 */                     // If there are any stalled coroutine obligations, then this
/* FP:suggestions.rs-3053 */                     // error may be due to that, and not because the body has more
/* FP:suggestions.rs-3054 */                     // where-clauses.
/* FP:suggestions.rs-3055 */                     && self.tcx.typeck(definition_def_id).coroutine_stalled_predicates.is_empty()
/* FP:suggestions.rs-3056 */                 {
/* FP:suggestions.rs-3057 */                     // FIXME(compiler-errors): We could probably point to something
/* FP:suggestions.rs-3058 */                     // specific here if we tried hard enough...
/* FP:suggestions.rs-3059 */                     err.span_note(
/* FP:suggestions.rs-3060 */                         tcx.def_span(definition_def_id),
/* FP:suggestions.rs-3061 */                         "this definition site has more where clauses than the opaque type",
/* FP:suggestions.rs-3062 */                     );
/* FP:suggestions.rs-3063 */                 }
/* FP:suggestions.rs-3064 */             }
/* FP:suggestions.rs-3065 */             ObligationCauseCode::Coercion { source, target } => {
/* FP:suggestions.rs-3066 */                 let source =
/* FP:suggestions.rs-3067 */                     tcx.short_string(self.resolve_vars_if_possible(source), err.long_ty_path());
/* FP:suggestions.rs-3068 */                 let target =
/* FP:suggestions.rs-3069 */                     tcx.short_string(self.resolve_vars_if_possible(target), err.long_ty_path());
/* FP:suggestions.rs-3070 */                 err.note(with_forced_trimmed_paths!(format!(
/* FP:suggestions.rs-3071 */                     "required for the cast from `{source}` to `{target}`",
/* FP:suggestions.rs-3072 */                 )));
/* FP:suggestions.rs-3073 */             }
/* FP:suggestions.rs-3074 */             ObligationCauseCode::RepeatElementCopy { is_constable, elt_span } => {
/* FP:suggestions.rs-3075 */                 err.note(
/* FP:suggestions.rs-3076 */                     "the `Copy` trait is required because this value will be copied for each element of the array",
/* FP:suggestions.rs-3077 */                 );
/* FP:suggestions.rs-3078 */                 let sm = tcx.sess.source_map();
/* FP:suggestions.rs-3079 */                 if matches!(is_constable, IsConstable::Fn | IsConstable::Ctor)
/* FP:suggestions.rs-3080 */                     && let Ok(_) = sm.span_to_snippet(elt_span)
/* FP:suggestions.rs-3081 */                 {
/* FP:suggestions.rs-3082 */                     err.multipart_suggestion(
/* FP:suggestions.rs-3083 */                         "create an inline `const` block",
/* FP:suggestions.rs-3084 */                         vec![
/* FP:suggestions.rs-3085 */                             (elt_span.shrink_to_lo(), "const { ".to_string()),
/* FP:suggestions.rs-3086 */                             (elt_span.shrink_to_hi(), " }".to_string()),
/* FP:suggestions.rs-3087 */                         ],
/* FP:suggestions.rs-3088 */                         Applicability::MachineApplicable,
/* FP:suggestions.rs-3089 */                     );
/* FP:suggestions.rs-3090 */                 } else {
/* FP:suggestions.rs-3091 */                     // FIXME: we may suggest array::repeat instead
/* FP:suggestions.rs-3092 */                     err.help("consider using `core::array::from_fn` to initialize the array");
/* FP:suggestions.rs-3093 */                     err.help("see https://doc.rust-lang.org/stable/std/array/fn.from_fn.html for more information");
/* FP:suggestions.rs-3094 */                 }
/* FP:suggestions.rs-3095 */             }
/* FP:suggestions.rs-3096 */             ObligationCauseCode::VariableType(hir_id) => {
/* FP:suggestions.rs-3097 */                 if let Some(typeck_results) = &self.typeck_results
/* FP:suggestions.rs-3098 */                     && let Some(ty) = typeck_results.node_type_opt(hir_id)
/* FP:suggestions.rs-3099 */                     && let ty::Error(_) = ty.kind()
/* FP:suggestions.rs-3100 */                 {
/* FP:suggestions.rs-3101 */                     err.note(format!(
/* FP:suggestions.rs-3102 */                         "`{predicate}` isn't satisfied, but the type of this pattern is \
/* FP:suggestions.rs-3103 */                          `{{type error}}`",
/* FP:suggestions.rs-3104 */                     ));
/* FP:suggestions.rs-3105 */                     err.downgrade_to_delayed_bug();
/* FP:suggestions.rs-3106 */                 }
/* FP:suggestions.rs-3107 */                 let mut local = true;
/* FP:suggestions.rs-3108 */                 match tcx.parent_hir_node(hir_id) {
/* FP:suggestions.rs-3109 */                     Node::LetStmt(hir::LetStmt { ty: Some(ty), .. }) => {
/* FP:suggestions.rs-3110 */                         err.span_suggestion_verbose(
/* FP:suggestions.rs-3111 */                             ty.span.shrink_to_lo(),
/* FP:suggestions.rs-3112 */                             "consider borrowing here",
/* FP:suggestions.rs-3113 */                             "&",
/* FP:suggestions.rs-3114 */                             Applicability::MachineApplicable,
/* FP:suggestions.rs-3115 */                         );
/* FP:suggestions.rs-3116 */                     }
/* FP:suggestions.rs-3117 */                     Node::LetStmt(hir::LetStmt {
/* FP:suggestions.rs-3118 */                         init: Some(hir::Expr { kind: hir::ExprKind::Index(..), span, .. }),
/* FP:suggestions.rs-3119 */                         ..
/* FP:suggestions.rs-3120 */                     }) => {
/* FP:suggestions.rs-3121 */                         // When encountering an assignment of an unsized trait, like
/* FP:suggestions.rs-3122 */                         // `let x = ""[..];`, provide a suggestion to borrow the initializer in
/* FP:suggestions.rs-3123 */                         // order to use have a slice instead.
/* FP:suggestions.rs-3124 */                         err.span_suggestion_verbose(
/* FP:suggestions.rs-3125 */                             span.shrink_to_lo(),
/* FP:suggestions.rs-3126 */                             "consider borrowing here",
/* FP:suggestions.rs-3127 */                             "&",
/* FP:suggestions.rs-3128 */                             Applicability::MachineApplicable,
/* FP:suggestions.rs-3129 */                         );
/* FP:suggestions.rs-3130 */                     }
/* FP:suggestions.rs-3131 */                     Node::LetStmt(hir::LetStmt { init: Some(expr), .. }) => {
/* FP:suggestions.rs-3132 */                         // When encountering an assignment of an unsized trait, like `let x = *"";`,
/* FP:suggestions.rs-3133 */                         // we check if the RHS is a deref operation, to suggest removing it.
/* FP:suggestions.rs-3134 */                         suggest_remove_deref(err, &expr);
/* FP:suggestions.rs-3135 */                     }
/* FP:suggestions.rs-3136 */                     Node::Param(param) => {
/* FP:suggestions.rs-3137 */                         err.span_suggestion_verbose(
/* FP:suggestions.rs-3138 */                             param.ty_span.shrink_to_lo(),
/* FP:suggestions.rs-3139 */                             "function arguments must have a statically known size, borrowed types \
/* FP:suggestions.rs-3140 */                             always have a known size",
/* FP:suggestions.rs-3141 */                             "&",
/* FP:suggestions.rs-3142 */                             Applicability::MachineApplicable,
/* FP:suggestions.rs-3143 */                         );
/* FP:suggestions.rs-3144 */                         local = false;
/* FP:suggestions.rs-3145 */                     }
/* FP:suggestions.rs-3146 */                     _ => {}
/* FP:suggestions.rs-3147 */                 }
/* FP:suggestions.rs-3148 */                 if local {
/* FP:suggestions.rs-3149 */                     err.note("all local variables must have a statically known size");
/* FP:suggestions.rs-3150 */                 }
/* FP:suggestions.rs-3151 */             }
/* FP:suggestions.rs-3152 */             ObligationCauseCode::SizedArgumentType(hir_id) => {
/* FP:suggestions.rs-3153 */                 let mut ty = None;
/* FP:suggestions.rs-3154 */                 let borrowed_msg = "function arguments must have a statically known size, borrowed \
/* FP:suggestions.rs-3155 */                                     types always have a known size";
/* FP:suggestions.rs-3156 */                 if let Some(hir_id) = hir_id
/* FP:suggestions.rs-3157 */                     && let hir::Node::Param(param) = self.tcx.hir_node(hir_id)
/* FP:suggestions.rs-3158 */                     && let Some(decl) = self.tcx.parent_hir_node(hir_id).fn_decl()
/* FP:suggestions.rs-3159 */                     && let Some(t) = decl.inputs.iter().find(|t| param.ty_span.contains(t.span))
/* FP:suggestions.rs-3160 */                 {
/* FP:suggestions.rs-3161 */                     // We use `contains` because the type might be surrounded by parentheses,
/* FP:suggestions.rs-3162 */                     // which makes `ty_span` and `t.span` disagree with each other, but one
/* FP:suggestions.rs-3163 */                     // fully contains the other: `foo: (dyn Foo + Bar)`
/* FP:suggestions.rs-3164 */                     //                                 ^-------------^
/* FP:suggestions.rs-3165 */                     //                                 ||
/* FP:suggestions.rs-3166 */                     //                                 |t.span
/* FP:suggestions.rs-3167 */                     //                                 param._ty_span
/* FP:suggestions.rs-3168 */                     ty = Some(t);
/* FP:suggestions.rs-3169 */                 } else if let Some(hir_id) = hir_id
/* FP:suggestions.rs-3170 */                     && let hir::Node::Ty(t) = self.tcx.hir_node(hir_id)
/* FP:suggestions.rs-3171 */                 {
/* FP:suggestions.rs-3172 */                     ty = Some(t);
/* FP:suggestions.rs-3173 */                 }
/* FP:suggestions.rs-3174 */                 if let Some(ty) = ty {
/* FP:suggestions.rs-3175 */                     match ty.kind {
/* FP:suggestions.rs-3176 */                         hir::TyKind::TraitObject(traits, _) => {
/* FP:suggestions.rs-3177 */                             let (span, kw) = match traits {
/* FP:suggestions.rs-3178 */                                 [first, ..] if first.span.lo() == ty.span.lo() => {
/* FP:suggestions.rs-3179 */                                     // Missing `dyn` in front of trait object.
/* FP:suggestions.rs-3180 */                                     (ty.span.shrink_to_lo(), "dyn ")
/* FP:suggestions.rs-3181 */                                 }
/* FP:suggestions.rs-3182 */                                 [first, ..] => (ty.span.until(first.span), ""),
/* FP:suggestions.rs-3183 */                                 [] => span_bug!(ty.span, "trait object with no traits: {ty:?}"),
/* FP:suggestions.rs-3184 */                             };
/* FP:suggestions.rs-3185 */                             let needs_parens = traits.len() != 1;
/* FP:suggestions.rs-3186 */                             // Don't recommend impl Trait as a closure argument
/* FP:suggestions.rs-3187 */                             if let Some(hir_id) = hir_id
/* FP:suggestions.rs-3188 */                                 && matches!(
/* FP:suggestions.rs-3189 */                                     self.tcx.parent_hir_node(hir_id),
/* FP:suggestions.rs-3190 */                                     hir::Node::Item(hir::Item {
/* FP:suggestions.rs-3191 */                                         kind: hir::ItemKind::Fn { .. },
/* FP:suggestions.rs-3192 */                                         ..
/* FP:suggestions.rs-3193 */                                     })
/* FP:suggestions.rs-3194 */                                 )
/* FP:suggestions.rs-3195 */                             {
/* FP:suggestions.rs-3196 */                                 err.span_suggestion_verbose(
/* FP:suggestions.rs-3197 */                                     span,
/* FP:suggestions.rs-3198 */                                     "you can use `impl Trait` as the argument type",
/* FP:suggestions.rs-3199 */                                     "impl ",
/* FP:suggestions.rs-3200 */                                     Applicability::MaybeIncorrect,
/* FP:suggestions.rs-3201 */                                 );
/* FP:suggestions.rs-3202 */                             }
/* FP:suggestions.rs-3203 */                             let sugg = if !needs_parens {
/* FP:suggestions.rs-3204 */                                 vec![(span.shrink_to_lo(), format!("&{kw}"))]
/* FP:suggestions.rs-3205 */                             } else {
/* FP:suggestions.rs-3206 */                                 vec![
/* FP:suggestions.rs-3207 */                                     (span.shrink_to_lo(), format!("&({kw}")),
/* FP:suggestions.rs-3208 */                                     (ty.span.shrink_to_hi(), ")".to_string()),
/* FP:suggestions.rs-3209 */                                 ]
/* FP:suggestions.rs-3210 */                             };
/* FP:suggestions.rs-3211 */                             err.multipart_suggestion_verbose(
/* FP:suggestions.rs-3212 */                                 borrowed_msg,
/* FP:suggestions.rs-3213 */                                 sugg,
/* FP:suggestions.rs-3214 */                                 Applicability::MachineApplicable,
/* FP:suggestions.rs-3215 */                             );
/* FP:suggestions.rs-3216 */                         }
/* FP:suggestions.rs-3217 */                         hir::TyKind::Slice(_ty) => {
/* FP:suggestions.rs-3218 */                             err.span_suggestion_verbose(
/* FP:suggestions.rs-3219 */                                 ty.span.shrink_to_lo(),
/* FP:suggestions.rs-3220 */                                 "function arguments must have a statically known size, borrowed \
/* FP:suggestions.rs-3221 */                                  slices always have a known size",
/* FP:suggestions.rs-3222 */                                 "&",
/* FP:suggestions.rs-3223 */                                 Applicability::MachineApplicable,
/* FP:suggestions.rs-3224 */                             );
/* FP:suggestions.rs-3225 */                         }
/* FP:suggestions.rs-3226 */                         hir::TyKind::Path(_) => {
/* FP:suggestions.rs-3227 */                             err.span_suggestion_verbose(
/* FP:suggestions.rs-3228 */                                 ty.span.shrink_to_lo(),
/* FP:suggestions.rs-3229 */                                 borrowed_msg,
/* FP:suggestions.rs-3230 */                                 "&",
/* FP:suggestions.rs-3231 */                                 Applicability::MachineApplicable,
/* FP:suggestions.rs-3232 */                             );
/* FP:suggestions.rs-3233 */                         }
/* FP:suggestions.rs-3234 */                         _ => {}
/* FP:suggestions.rs-3235 */                     }
/* FP:suggestions.rs-3236 */                 } else {
/* FP:suggestions.rs-3237 */                     err.note("all function arguments must have a statically known size");
/* FP:suggestions.rs-3238 */                 }
/* FP:suggestions.rs-3239 */                 if tcx.sess.opts.unstable_features.is_nightly_build()
/* FP:suggestions.rs-3240 */                     && !tcx.features().unsized_fn_params()
/* FP:suggestions.rs-3241 */                 {
/* FP:suggestions.rs-3242 */                     err.help("unsized fn params are gated as an unstable feature");
/* FP:suggestions.rs-3243 */                 }
/* FP:suggestions.rs-3244 */             }
/* FP:suggestions.rs-3245 */             ObligationCauseCode::SizedReturnType | ObligationCauseCode::SizedCallReturnType => {
/* FP:suggestions.rs-3246 */                 err.note("the return type of a function must have a statically known size");
/* FP:suggestions.rs-3247 */             }
/* FP:suggestions.rs-3248 */             ObligationCauseCode::SizedYieldType => {
/* FP:suggestions.rs-3249 */                 err.note("the yield type of a coroutine must have a statically known size");
/* FP:suggestions.rs-3250 */             }
/* FP:suggestions.rs-3251 */             ObligationCauseCode::AssignmentLhsSized => {
/* FP:suggestions.rs-3252 */                 err.note("the left-hand-side of an assignment must have a statically known size");
/* FP:suggestions.rs-3253 */             }
/* FP:suggestions.rs-3254 */             ObligationCauseCode::TupleInitializerSized => {
/* FP:suggestions.rs-3255 */                 err.note("tuples must have a statically known size to be initialized");
/* FP:suggestions.rs-3256 */             }
/* FP:suggestions.rs-3257 */             ObligationCauseCode::StructInitializerSized => {
/* FP:suggestions.rs-3258 */                 err.note("structs must have a statically known size to be initialized");
/* FP:suggestions.rs-3259 */             }
/* FP:suggestions.rs-3260 */             ObligationCauseCode::FieldSized { adt_kind: ref item, last, span } => {
/* FP:suggestions.rs-3261 */                 match *item {
/* FP:suggestions.rs-3262 */                     AdtKind::Struct => {
/* FP:suggestions.rs-3263 */                         if last {
/* FP:suggestions.rs-3264 */                             err.note(
/* FP:suggestions.rs-3265 */                                 "the last field of a packed struct may only have a \
/* FP:suggestions.rs-3266 */                                 dynamically sized type if it does not need drop to be run",
/* FP:suggestions.rs-3267 */                             );
/* FP:suggestions.rs-3268 */                         } else {
/* FP:suggestions.rs-3269 */                             err.note(
/* FP:suggestions.rs-3270 */                                 "only the last field of a struct may have a dynamically sized type",
/* FP:suggestions.rs-3271 */                             );
/* FP:suggestions.rs-3272 */                         }
/* FP:suggestions.rs-3273 */                     }
/* FP:suggestions.rs-3274 */                     AdtKind::Union => {
/* FP:suggestions.rs-3275 */                         err.note("no field of a union may have a dynamically sized type");
/* FP:suggestions.rs-3276 */                     }
/* FP:suggestions.rs-3277 */                     AdtKind::Enum => {
/* FP:suggestions.rs-3278 */                         err.note("no field of an enum variant may have a dynamically sized type");
/* FP:suggestions.rs-3279 */                     }
/* FP:suggestions.rs-3280 */                 }
/* FP:suggestions.rs-3281 */                 err.help("change the field's type to have a statically known size");
/* FP:suggestions.rs-3282 */                 err.span_suggestion_verbose(
/* FP:suggestions.rs-3283 */                     span.shrink_to_lo(),
/* FP:suggestions.rs-3284 */                     "borrowed types always have a statically known size",
/* FP:suggestions.rs-3285 */                     "&",
/* FP:suggestions.rs-3286 */                     Applicability::MachineApplicable,
/* FP:suggestions.rs-3287 */                 );
/* FP:suggestions.rs-3288 */                 err.multipart_suggestion_verbose(
/* FP:suggestions.rs-3289 */                     "the `Box` type always has a statically known size and allocates its contents \
/* FP:suggestions.rs-3290 */                      in the heap",
/* FP:suggestions.rs-3291 */                     vec![
/* FP:suggestions.rs-3292 */                         (span.shrink_to_lo(), "Box<".to_string()),
/* FP:suggestions.rs-3293 */                         (span.shrink_to_hi(), ">".to_string()),
/* FP:suggestions.rs-3294 */                     ],
/* FP:suggestions.rs-3295 */                     Applicability::MachineApplicable,
/* FP:suggestions.rs-3296 */                 );
/* FP:suggestions.rs-3297 */             }
/* FP:suggestions.rs-3298 */             ObligationCauseCode::SizedConstOrStatic => {
/* FP:suggestions.rs-3299 */                 err.note("statics and constants must have a statically known size");
/* FP:suggestions.rs-3300 */             }
/* FP:suggestions.rs-3301 */             ObligationCauseCode::InlineAsmSized => {
/* FP:suggestions.rs-3302 */                 err.note("all inline asm arguments must have a statically known size");
/* FP:suggestions.rs-3303 */             }
/* FP:suggestions.rs-3304 */             ObligationCauseCode::SizedClosureCapture(closure_def_id) => {
/* FP:suggestions.rs-3305 */                 err.note(
/* FP:suggestions.rs-3306 */                     "all values captured by value by a closure must have a statically known size",
/* FP:suggestions.rs-3307 */                 );
/* FP:suggestions.rs-3308 */                 let hir::ExprKind::Closure(closure) =
/* FP:suggestions.rs-3309 */                     tcx.hir_node_by_def_id(closure_def_id).expect_expr().kind
/* FP:suggestions.rs-3310 */                 else {
/* FP:suggestions.rs-3311 */                     bug!("expected closure in SizedClosureCapture obligation");
/* FP:suggestions.rs-3312 */                 };
/* FP:suggestions.rs-3313 */                 if let hir::CaptureBy::Value { .. } = closure.capture_clause
/* FP:suggestions.rs-3314 */                     && let Some(span) = closure.fn_arg_span
/* FP:suggestions.rs-3315 */                 {
/* FP:suggestions.rs-3316 */                     err.span_label(span, "this closure captures all values by move");
/* FP:suggestions.rs-3317 */                 }
/* FP:suggestions.rs-3318 */             }
/* FP:suggestions.rs-3319 */             ObligationCauseCode::SizedCoroutineInterior(coroutine_def_id) => {
/* FP:suggestions.rs-3320 */                 let what = match tcx.coroutine_kind(coroutine_def_id) {
/* FP:suggestions.rs-3321 */                     None
/* FP:suggestions.rs-3322 */                     | Some(hir::CoroutineKind::Coroutine(_))
/* FP:suggestions.rs-3323 */                     | Some(hir::CoroutineKind::Desugared(hir::CoroutineDesugaring::Gen, _)) => {
/* FP:suggestions.rs-3324 */                         "yield"
/* FP:suggestions.rs-3325 */                     }
/* FP:suggestions.rs-3326 */                     Some(hir::CoroutineKind::Desugared(hir::CoroutineDesugaring::Async, _)) => {
/* FP:suggestions.rs-3327 */                         "await"
/* FP:suggestions.rs-3328 */                     }
/* FP:suggestions.rs-3329 */                     Some(hir::CoroutineKind::Desugared(hir::CoroutineDesugaring::AsyncGen, _)) => {
/* FP:suggestions.rs-3330 */                         "yield`/`await"
/* FP:suggestions.rs-3331 */                     }
/* FP:suggestions.rs-3332 */                 };
/* FP:suggestions.rs-3333 */                 err.note(format!(
/* FP:suggestions.rs-3334 */                     "all values live across `{what}` must have a statically known size"
/* FP:suggestions.rs-3335 */                 ));
/* FP:suggestions.rs-3336 */             }
/* FP:suggestions.rs-3337 */             ObligationCauseCode::SharedStatic => {
/* FP:suggestions.rs-3338 */                 err.note("shared static variables must have a type that implements `Sync`");
/* FP:suggestions.rs-3339 */             }
/* FP:suggestions.rs-3340 */             ObligationCauseCode::BuiltinDerived(ref data) => {
/* FP:suggestions.rs-3341 */                 let parent_trait_ref = self.resolve_vars_if_possible(data.parent_trait_pred);
/* FP:suggestions.rs-3342 */                 let ty = parent_trait_ref.skip_binder().self_ty();
/* FP:suggestions.rs-3343 */                 if parent_trait_ref.references_error() {
/* FP:suggestions.rs-3344 */                     // NOTE(eddyb) this was `.cancel()`, but `err`
/* FP:suggestions.rs-3345 */                     // is borrowed, so we can't fully defuse it.
/* FP:suggestions.rs-3346 */                     err.downgrade_to_delayed_bug();
/* FP:suggestions.rs-3347 */                     return;
/* FP:suggestions.rs-3348 */                 }
/* FP:suggestions.rs-3349 */ 
/* FP:suggestions.rs-3350 */                 // If the obligation for a tuple is set directly by a Coroutine or Closure,
/* FP:suggestions.rs-3351 */                 // then the tuple must be the one containing capture types.
/* FP:suggestions.rs-3352 */                 let is_upvar_tys_infer_tuple = if !matches!(ty.kind(), ty::Tuple(..)) {
/* FP:suggestions.rs-3353 */                     false
/* FP:suggestions.rs-3354 */                 } else if let ObligationCauseCode::BuiltinDerived(data) = &*data.parent_code {
/* FP:suggestions.rs-3355 */                     let parent_trait_ref = self.resolve_vars_if_possible(data.parent_trait_pred);
/* FP:suggestions.rs-3356 */                     let nested_ty = parent_trait_ref.skip_binder().self_ty();
/* FP:suggestions.rs-3357 */                     matches!(nested_ty.kind(), ty::Coroutine(..))
/* FP:suggestions.rs-3358 */                         || matches!(nested_ty.kind(), ty::Closure(..))
/* FP:suggestions.rs-3359 */                 } else {
/* FP:suggestions.rs-3360 */                     false
/* FP:suggestions.rs-3361 */                 };
/* FP:suggestions.rs-3362 */ 
/* FP:suggestions.rs-3363 */                 let is_builtin_async_fn_trait =
/* FP:suggestions.rs-3364 */                     tcx.async_fn_trait_kind_from_def_id(data.parent_trait_pred.def_id()).is_some();
/* FP:suggestions.rs-3365 */ 
/* FP:suggestions.rs-3366 */                 if !is_upvar_tys_infer_tuple && !is_builtin_async_fn_trait {
/* FP:suggestions.rs-3367 */                     let mut msg = || {
/* FP:suggestions.rs-3368 */                         let ty_str = tcx.short_string(ty, err.long_ty_path());
/* FP:suggestions.rs-3369 */                         format!("required because it appears within the type `{ty_str}`")
/* FP:suggestions.rs-3370 */                     };
/* FP:suggestions.rs-3371 */                     match ty.kind() {
/* FP:suggestions.rs-3372 */                         ty::Adt(def, _) => {
/* FP:suggestions.rs-3373 */                             let msg = msg();
/* FP:suggestions.rs-3374 */                             match tcx.opt_item_ident(def.did()) {
/* FP:suggestions.rs-3375 */                                 Some(ident) => {
/* FP:suggestions.rs-3376 */                                     err.span_note(ident.span, msg);
/* FP:suggestions.rs-3377 */                                 }
/* FP:suggestions.rs-3378 */                                 None => {
/* FP:suggestions.rs-3379 */                                     err.note(msg);
/* FP:suggestions.rs-3380 */                                 }
/* FP:suggestions.rs-3381 */                             }
/* FP:suggestions.rs-3382 */                         }
/* FP:suggestions.rs-3383 */                         ty::Alias(ty::Opaque, ty::AliasTy { def_id, .. }) => {
/* FP:suggestions.rs-3384 */                             // If the previous type is async fn, this is the future generated by the body of an async function.
/* FP:suggestions.rs-3385 */                             // Avoid printing it twice (it was already printed in the `ty::Coroutine` arm below).
/* FP:suggestions.rs-3386 */                             let is_future = tcx.ty_is_opaque_future(ty);
/* FP:suggestions.rs-3387 */                             debug!(
/* FP:suggestions.rs-3388 */                                 ?obligated_types,
/* FP:suggestions.rs-3389 */                                 ?is_future,
/* FP:suggestions.rs-3390 */                                 "note_obligation_cause_code: check for async fn"
/* FP:suggestions.rs-3391 */                             );
/* FP:suggestions.rs-3392 */                             if is_future
/* FP:suggestions.rs-3393 */                                 && obligated_types.last().is_some_and(|ty| match ty.kind() {
/* FP:suggestions.rs-3394 */                                     ty::Coroutine(last_def_id, ..) => {
/* FP:suggestions.rs-3395 */                                         tcx.coroutine_is_async(*last_def_id)
/* FP:suggestions.rs-3396 */                                     }
/* FP:suggestions.rs-3397 */                                     _ => false,
/* FP:suggestions.rs-3398 */                                 })
/* FP:suggestions.rs-3399 */                             {
/* FP:suggestions.rs-3400 */                                 // See comment above; skip printing twice.
/* FP:suggestions.rs-3401 */                             } else {
/* FP:suggestions.rs-3402 */                                 let msg = msg();
/* FP:suggestions.rs-3403 */                                 err.span_note(tcx.def_span(def_id), msg);
/* FP:suggestions.rs-3404 */                             }
/* FP:suggestions.rs-3405 */                         }
/* FP:suggestions.rs-3406 */                         ty::Coroutine(def_id, _) => {
/* FP:suggestions.rs-3407 */                             let sp = tcx.def_span(def_id);
/* FP:suggestions.rs-3408 */ 
/* FP:suggestions.rs-3409 */                             // Special-case this to say "async block" instead of `[static coroutine]`.
/* FP:suggestions.rs-3410 */                             let kind = tcx.coroutine_kind(def_id).unwrap();
/* FP:suggestions.rs-3411 */                             err.span_note(
/* FP:suggestions.rs-3412 */                                 sp,
/* FP:suggestions.rs-3413 */                                 with_forced_trimmed_paths!(format!(
/* FP:suggestions.rs-3414 */                                     "required because it's used within this {kind:#}",
/* FP:suggestions.rs-3415 */                                 )),
/* FP:suggestions.rs-3416 */                             );
/* FP:suggestions.rs-3417 */                         }
/* FP:suggestions.rs-3418 */                         ty::CoroutineWitness(..) => {
/* FP:suggestions.rs-3419 */                             // Skip printing coroutine-witnesses, since we'll drill into
/* FP:suggestions.rs-3420 */                             // the bad field in another derived obligation cause.
/* FP:suggestions.rs-3421 */                         }
/* FP:suggestions.rs-3422 */                         ty::Closure(def_id, _) | ty::CoroutineClosure(def_id, _) => {
/* FP:suggestions.rs-3423 */                             err.span_note(
/* FP:suggestions.rs-3424 */                                 tcx.def_span(def_id),
/* FP:suggestions.rs-3425 */                                 "required because it's used within this closure",
/* FP:suggestions.rs-3426 */                             );
/* FP:suggestions.rs-3427 */                         }
/* FP:suggestions.rs-3428 */                         ty::Str => {
/* FP:suggestions.rs-3429 */                             err.note("`str` is considered to contain a `[u8]` slice for auto trait purposes");
/* FP:suggestions.rs-3430 */                         }
/* FP:suggestions.rs-3431 */                         _ => {
/* FP:suggestions.rs-3432 */                             let msg = msg();
/* FP:suggestions.rs-3433 */                             err.note(msg);
/* FP:suggestions.rs-3434 */                         }
/* FP:suggestions.rs-3435 */                     };
/* FP:suggestions.rs-3436 */                 }
/* FP:suggestions.rs-3437 */ 
/* FP:suggestions.rs-3438 */                 obligated_types.push(ty);
/* FP:suggestions.rs-3439 */ 
/* FP:suggestions.rs-3440 */                 let parent_predicate = parent_trait_ref;
/* FP:suggestions.rs-3441 */                 if !self.is_recursive_obligation(obligated_types, &data.parent_code) {
/* FP:suggestions.rs-3442 */                     // #74711: avoid a stack overflow
/* FP:suggestions.rs-3443 */                     ensure_sufficient_stack(|| {
/* FP:suggestions.rs-3444 */                         self.note_obligation_cause_code(
/* FP:suggestions.rs-3445 */                             body_id,
/* FP:suggestions.rs-3446 */                             err,
/* FP:suggestions.rs-3447 */                             parent_predicate,
/* FP:suggestions.rs-3448 */                             param_env,
/* FP:suggestions.rs-3449 */                             &data.parent_code,
/* FP:suggestions.rs-3450 */                             obligated_types,
/* FP:suggestions.rs-3451 */                             seen_requirements,
/* FP:suggestions.rs-3452 */                         )
/* FP:suggestions.rs-3453 */                     });
/* FP:suggestions.rs-3454 */                 } else {
/* FP:suggestions.rs-3455 */                     ensure_sufficient_stack(|| {
/* FP:suggestions.rs-3456 */                         self.note_obligation_cause_code(
/* FP:suggestions.rs-3457 */                             body_id,
/* FP:suggestions.rs-3458 */                             err,
/* FP:suggestions.rs-3459 */                             parent_predicate,
/* FP:suggestions.rs-3460 */                             param_env,
/* FP:suggestions.rs-3461 */                             cause_code.peel_derives(),
/* FP:suggestions.rs-3462 */                             obligated_types,
/* FP:suggestions.rs-3463 */                             seen_requirements,
/* FP:suggestions.rs-3464 */                         )
/* FP:suggestions.rs-3465 */                     });
/* FP:suggestions.rs-3466 */                 }
/* FP:suggestions.rs-3467 */             }
/* FP:suggestions.rs-3468 */             ObligationCauseCode::ImplDerived(ref data) => {
/* FP:suggestions.rs-3469 */                 let mut parent_trait_pred =
/* FP:suggestions.rs-3470 */                     self.resolve_vars_if_possible(data.derived.parent_trait_pred);
/* FP:suggestions.rs-3471 */                 let parent_def_id = parent_trait_pred.def_id();
/* FP:suggestions.rs-3472 */                 if tcx.is_diagnostic_item(sym::FromResidual, parent_def_id)
/* FP:suggestions.rs-3473 */                     && !tcx.features().enabled(sym::try_trait_v2)
/* FP:suggestions.rs-3474 */                 {
/* FP:suggestions.rs-3475 */                     // If `#[feature(try_trait_v2)]` is not enabled, then there's no point on
/* FP:suggestions.rs-3476 */                     // talking about `FromResidual<Result<A, B>>`, as the end user has nothing they
/* FP:suggestions.rs-3477 */                     // can do about it. As far as they are concerned, `?` is compiler magic.
/* FP:suggestions.rs-3478 */                     return;
/* FP:suggestions.rs-3479 */                 }
/* FP:suggestions.rs-3480 */                 let self_ty_str =
/* FP:suggestions.rs-3481 */                     tcx.short_string(parent_trait_pred.skip_binder().self_ty(), err.long_ty_path());
/* FP:suggestions.rs-3482 */                 let trait_name = tcx.short_string(
/* FP:suggestions.rs-3483 */                     parent_trait_pred.print_modifiers_and_trait_path(),
/* FP:suggestions.rs-3484 */                     err.long_ty_path(),
/* FP:suggestions.rs-3485 */                 );
/* FP:suggestions.rs-3486 */                 let msg = format!("required for `{self_ty_str}` to implement `{trait_name}`");
/* FP:suggestions.rs-3487 */                 let mut is_auto_trait = false;
/* FP:suggestions.rs-3488 */                 match tcx.hir_get_if_local(data.impl_or_alias_def_id) {
/* FP:suggestions.rs-3489 */                     Some(Node::Item(hir::Item {
/* FP:suggestions.rs-3490 */                         kind: hir::ItemKind::Trait(_, is_auto, _, ident, ..),
/* FP:suggestions.rs-3491 */                         ..
/* FP:suggestions.rs-3492 */                     })) => {
/* FP:suggestions.rs-3493 */                         // FIXME: we should do something else so that it works even on crate foreign
/* FP:suggestions.rs-3494 */                         // auto traits.
/* FP:suggestions.rs-3495 */                         is_auto_trait = matches!(is_auto, hir::IsAuto::Yes);
/* FP:suggestions.rs-3496 */                         err.span_note(ident.span, msg);
/* FP:suggestions.rs-3497 */                     }
/* FP:suggestions.rs-3498 */                     Some(Node::Item(hir::Item {
/* FP:suggestions.rs-3499 */                         kind: hir::ItemKind::Impl(hir::Impl { of_trait, self_ty, generics, .. }),
/* FP:suggestions.rs-3500 */                         ..
/* FP:suggestions.rs-3501 */                     })) => {
/* FP:suggestions.rs-3502 */                         let mut spans = Vec::with_capacity(2);
/* FP:suggestions.rs-3503 */                         if let Some(of_trait) = of_trait {
/* FP:suggestions.rs-3504 */                             spans.push(of_trait.trait_ref.path.span);
/* FP:suggestions.rs-3505 */                         }
/* FP:suggestions.rs-3506 */                         spans.push(self_ty.span);
/* FP:suggestions.rs-3507 */                         let mut spans: MultiSpan = spans.into();
/* FP:suggestions.rs-3508 */                         if matches!(
/* FP:suggestions.rs-3509 */                             self_ty.span.ctxt().outer_expn_data().kind,
/* FP:suggestions.rs-3510 */                             ExpnKind::Macro(MacroKind::Derive, _)
/* FP:suggestions.rs-3511 */                         ) || matches!(
/* FP:suggestions.rs-3512 */                             of_trait.map(|t| t.trait_ref.path.span.ctxt().outer_expn_data().kind),
/* FP:suggestions.rs-3513 */                             Some(ExpnKind::Macro(MacroKind::Derive, _))
/* FP:suggestions.rs-3514 */                         ) {
/* FP:suggestions.rs-3515 */                             spans.push_span_label(
/* FP:suggestions.rs-3516 */                                 data.span,
/* FP:suggestions.rs-3517 */                                 "unsatisfied trait bound introduced in this `derive` macro",
/* FP:suggestions.rs-3518 */                             );
/* FP:suggestions.rs-3519 */                         } else if !data.span.is_dummy() && !data.span.overlaps(self_ty.span) {
/* FP:suggestions.rs-3520 */                             spans.push_span_label(
/* FP:suggestions.rs-3521 */                                 data.span,
/* FP:suggestions.rs-3522 */                                 "unsatisfied trait bound introduced here",
/* FP:suggestions.rs-3523 */                             );
/* FP:suggestions.rs-3524 */                         }
/* FP:suggestions.rs-3525 */                         err.span_note(spans, msg);
/* FP:suggestions.rs-3526 */                         point_at_assoc_type_restriction(
/* FP:suggestions.rs-3527 */                             tcx,
/* FP:suggestions.rs-3528 */                             err,
/* FP:suggestions.rs-3529 */                             &self_ty_str,
/* FP:suggestions.rs-3530 */                             &trait_name,
/* FP:suggestions.rs-3531 */                             predicate,
/* FP:suggestions.rs-3532 */                             &generics,
/* FP:suggestions.rs-3533 */                             &data,
/* FP:suggestions.rs-3534 */                         );
/* FP:suggestions.rs-3535 */                     }
/* FP:suggestions.rs-3536 */                     _ => {
/* FP:suggestions.rs-3537 */                         err.note(msg);
/* FP:suggestions.rs-3538 */                     }
/* FP:suggestions.rs-3539 */                 };
/* FP:suggestions.rs-3540 */ 
/* FP:suggestions.rs-3541 */                 let mut parent_predicate = parent_trait_pred;
/* FP:suggestions.rs-3542 */                 let mut data = &data.derived;
/* FP:suggestions.rs-3543 */                 let mut count = 0;
/* FP:suggestions.rs-3544 */                 seen_requirements.insert(parent_def_id);
/* FP:suggestions.rs-3545 */                 if is_auto_trait {
/* FP:suggestions.rs-3546 */                     // We don't want to point at the ADT saying "required because it appears within
/* FP:suggestions.rs-3547 */                     // the type `X`", like we would otherwise do in test `supertrait-auto-trait.rs`.
/* FP:suggestions.rs-3548 */                     while let ObligationCauseCode::BuiltinDerived(derived) = &*data.parent_code {
/* FP:suggestions.rs-3549 */                         let child_trait_ref =
/* FP:suggestions.rs-3550 */                             self.resolve_vars_if_possible(derived.parent_trait_pred);
/* FP:suggestions.rs-3551 */                         let child_def_id = child_trait_ref.def_id();
/* FP:suggestions.rs-3552 */                         if seen_requirements.insert(child_def_id) {
/* FP:suggestions.rs-3553 */                             break;
/* FP:suggestions.rs-3554 */                         }
/* FP:suggestions.rs-3555 */                         data = derived;
/* FP:suggestions.rs-3556 */                         parent_predicate = child_trait_ref.upcast(tcx);
/* FP:suggestions.rs-3557 */                         parent_trait_pred = child_trait_ref;
/* FP:suggestions.rs-3558 */                     }
/* FP:suggestions.rs-3559 */                 }
/* FP:suggestions.rs-3560 */                 while let ObligationCauseCode::ImplDerived(child) = &*data.parent_code {
/* FP:suggestions.rs-3561 */                     // Skip redundant recursive obligation notes. See `ui/issue-20413.rs`.
/* FP:suggestions.rs-3562 */                     let child_trait_pred =
/* FP:suggestions.rs-3563 */                         self.resolve_vars_if_possible(child.derived.parent_trait_pred);
/* FP:suggestions.rs-3564 */                     let child_def_id = child_trait_pred.def_id();
/* FP:suggestions.rs-3565 */                     if seen_requirements.insert(child_def_id) {
/* FP:suggestions.rs-3566 */                         break;
/* FP:suggestions.rs-3567 */                     }
/* FP:suggestions.rs-3568 */                     count += 1;
/* FP:suggestions.rs-3569 */                     data = &child.derived;
/* FP:suggestions.rs-3570 */                     parent_predicate = child_trait_pred.upcast(tcx);
/* FP:suggestions.rs-3571 */                     parent_trait_pred = child_trait_pred;
/* FP:suggestions.rs-3572 */                 }
/* FP:suggestions.rs-3573 */                 if count > 0 {
/* FP:suggestions.rs-3574 */                     err.note(format!(
/* FP:suggestions.rs-3575 */                         "{} redundant requirement{} hidden",
/* FP:suggestions.rs-3576 */                         count,
/* FP:suggestions.rs-3577 */                         pluralize!(count)
/* FP:suggestions.rs-3578 */                     ));
/* FP:suggestions.rs-3579 */                     let self_ty = tcx.short_string(
/* FP:suggestions.rs-3580 */                         parent_trait_pred.skip_binder().self_ty(),
/* FP:suggestions.rs-3581 */                         err.long_ty_path(),
/* FP:suggestions.rs-3582 */                     );
/* FP:suggestions.rs-3583 */                     let trait_path = tcx.short_string(
/* FP:suggestions.rs-3584 */                         parent_trait_pred.print_modifiers_and_trait_path(),
/* FP:suggestions.rs-3585 */                         err.long_ty_path(),
/* FP:suggestions.rs-3586 */                     );
/* FP:suggestions.rs-3587 */                     err.note(format!("required for `{self_ty}` to implement `{trait_path}`"));
/* FP:suggestions.rs-3588 */                 }
/* FP:suggestions.rs-3589 */                 // #74711: avoid a stack overflow
/* FP:suggestions.rs-3590 */                 ensure_sufficient_stack(|| {
/* FP:suggestions.rs-3591 */                     self.note_obligation_cause_code(
/* FP:suggestions.rs-3592 */                         body_id,
/* FP:suggestions.rs-3593 */                         err,
/* FP:suggestions.rs-3594 */                         parent_predicate,
/* FP:suggestions.rs-3595 */                         param_env,
/* FP:suggestions.rs-3596 */                         &data.parent_code,
/* FP:suggestions.rs-3597 */                         obligated_types,
/* FP:suggestions.rs-3598 */                         seen_requirements,
/* FP:suggestions.rs-3599 */                     )
/* FP:suggestions.rs-3600 */                 });
/* FP:suggestions.rs-3601 */             }
/* FP:suggestions.rs-3602 */             ObligationCauseCode::ImplDerivedHost(ref data) => {
/* FP:suggestions.rs-3603 */                 let self_ty = tcx.short_string(
/* FP:suggestions.rs-3604 */                     self.resolve_vars_if_possible(data.derived.parent_host_pred.self_ty()),
/* FP:suggestions.rs-3605 */                     err.long_ty_path(),
/* FP:suggestions.rs-3606 */                 );
/* FP:suggestions.rs-3607 */                 let trait_path = tcx.short_string(
/* FP:suggestions.rs-3608 */                     data.derived
/* FP:suggestions.rs-3609 */                         .parent_host_pred
/* FP:suggestions.rs-3610 */                         .map_bound(|pred| pred.trait_ref)
/* FP:suggestions.rs-3611 */                         .print_only_trait_path(),
/* FP:suggestions.rs-3612 */                     err.long_ty_path(),
/* FP:suggestions.rs-3613 */                 );
/* FP:suggestions.rs-3614 */                 let msg = format!(
/* FP:suggestions.rs-3615 */                     "required for `{self_ty}` to implement `{} {trait_path}`",
/* FP:suggestions.rs-3616 */                     data.derived.parent_host_pred.skip_binder().constness,
/* FP:suggestions.rs-3617 */                 );
/* FP:suggestions.rs-3618 */                 match tcx.hir_get_if_local(data.impl_def_id) {
/* FP:suggestions.rs-3619 */                     Some(Node::Item(hir::Item {
/* FP:suggestions.rs-3620 */                         kind: hir::ItemKind::Impl(hir::Impl { of_trait, self_ty, .. }),
/* FP:suggestions.rs-3621 */                         ..
/* FP:suggestions.rs-3622 */                     })) => {
/* FP:suggestions.rs-3623 */                         let mut spans = vec![self_ty.span];
/* FP:suggestions.rs-3624 */                         spans.extend(of_trait.map(|t| t.trait_ref.path.span));
/* FP:suggestions.rs-3625 */                         let mut spans: MultiSpan = spans.into();
/* FP:suggestions.rs-3626 */                         spans.push_span_label(data.span, "unsatisfied trait bound introduced here");
/* FP:suggestions.rs-3627 */                         err.span_note(spans, msg);
/* FP:suggestions.rs-3628 */                     }
/* FP:suggestions.rs-3629 */                     _ => {
/* FP:suggestions.rs-3630 */                         err.note(msg);
/* FP:suggestions.rs-3631 */                     }
/* FP:suggestions.rs-3632 */                 }
/* FP:suggestions.rs-3633 */                 ensure_sufficient_stack(|| {
/* FP:suggestions.rs-3634 */                     self.note_obligation_cause_code(
/* FP:suggestions.rs-3635 */                         body_id,
/* FP:suggestions.rs-3636 */                         err,
/* FP:suggestions.rs-3637 */                         data.derived.parent_host_pred,
/* FP:suggestions.rs-3638 */                         param_env,
/* FP:suggestions.rs-3639 */                         &data.derived.parent_code,
/* FP:suggestions.rs-3640 */                         obligated_types,
/* FP:suggestions.rs-3641 */                         seen_requirements,
/* FP:suggestions.rs-3642 */                     )
/* FP:suggestions.rs-3643 */                 });
/* FP:suggestions.rs-3644 */             }
/* FP:suggestions.rs-3645 */             ObligationCauseCode::BuiltinDerivedHost(ref data) => {
/* FP:suggestions.rs-3646 */                 ensure_sufficient_stack(|| {
/* FP:suggestions.rs-3647 */                     self.note_obligation_cause_code(
/* FP:suggestions.rs-3648 */                         body_id,
/* FP:suggestions.rs-3649 */                         err,
/* FP:suggestions.rs-3650 */                         data.parent_host_pred,
/* FP:suggestions.rs-3651 */                         param_env,
/* FP:suggestions.rs-3652 */                         &data.parent_code,
/* FP:suggestions.rs-3653 */                         obligated_types,
/* FP:suggestions.rs-3654 */                         seen_requirements,
/* FP:suggestions.rs-3655 */                     )
/* FP:suggestions.rs-3656 */                 });
/* FP:suggestions.rs-3657 */             }
/* FP:suggestions.rs-3658 */             ObligationCauseCode::WellFormedDerived(ref data) => {
/* FP:suggestions.rs-3659 */                 let parent_trait_ref = self.resolve_vars_if_possible(data.parent_trait_pred);
/* FP:suggestions.rs-3660 */                 let parent_predicate = parent_trait_ref;
/* FP:suggestions.rs-3661 */                 // #74711: avoid a stack overflow
/* FP:suggestions.rs-3662 */                 ensure_sufficient_stack(|| {
/* FP:suggestions.rs-3663 */                     self.note_obligation_cause_code(
/* FP:suggestions.rs-3664 */                         body_id,
/* FP:suggestions.rs-3665 */                         err,
/* FP:suggestions.rs-3666 */                         parent_predicate,
/* FP:suggestions.rs-3667 */                         param_env,
/* FP:suggestions.rs-3668 */                         &data.parent_code,
/* FP:suggestions.rs-3669 */                         obligated_types,
/* FP:suggestions.rs-3670 */                         seen_requirements,
/* FP:suggestions.rs-3671 */                     )
/* FP:suggestions.rs-3672 */                 });
/* FP:suggestions.rs-3673 */             }
/* FP:suggestions.rs-3674 */             ObligationCauseCode::TypeAlias(ref nested, span, def_id) => {
/* FP:suggestions.rs-3675 */                 // #74711: avoid a stack overflow
/* FP:suggestions.rs-3676 */                 ensure_sufficient_stack(|| {
/* FP:suggestions.rs-3677 */                     self.note_obligation_cause_code(
/* FP:suggestions.rs-3678 */                         body_id,
/* FP:suggestions.rs-3679 */                         err,
/* FP:suggestions.rs-3680 */                         predicate,
/* FP:suggestions.rs-3681 */                         param_env,
/* FP:suggestions.rs-3682 */                         nested,
/* FP:suggestions.rs-3683 */                         obligated_types,
/* FP:suggestions.rs-3684 */                         seen_requirements,
/* FP:suggestions.rs-3685 */                     )
/* FP:suggestions.rs-3686 */                 });
/* FP:suggestions.rs-3687 */                 let mut multispan = MultiSpan::from(span);
/* FP:suggestions.rs-3688 */                 multispan.push_span_label(span, "required by this bound");
/* FP:suggestions.rs-3689 */                 err.span_note(
/* FP:suggestions.rs-3690 */                     multispan,
/* FP:suggestions.rs-3691 */                     format!("required by a bound on the type alias `{}`", tcx.item_name(def_id)),
/* FP:suggestions.rs-3692 */                 );
/* FP:suggestions.rs-3693 */             }
/* FP:suggestions.rs-3694 */             ObligationCauseCode::FunctionArg {
/* FP:suggestions.rs-3695 */                 arg_hir_id, call_hir_id, ref parent_code, ..
/* FP:suggestions.rs-3696 */             } => {
/* FP:suggestions.rs-3697 */                 self.note_function_argument_obligation(
/* FP:suggestions.rs-3698 */                     body_id,
/* FP:suggestions.rs-3699 */                     err,
/* FP:suggestions.rs-3700 */                     arg_hir_id,
/* FP:suggestions.rs-3701 */                     parent_code,
/* FP:suggestions.rs-3702 */                     param_env,
/* FP:suggestions.rs-3703 */                     predicate,
/* FP:suggestions.rs-3704 */                     call_hir_id,
/* FP:suggestions.rs-3705 */                 );
/* FP:suggestions.rs-3706 */                 ensure_sufficient_stack(|| {
/* FP:suggestions.rs-3707 */                     self.note_obligation_cause_code(
/* FP:suggestions.rs-3708 */                         body_id,
/* FP:suggestions.rs-3709 */                         err,
/* FP:suggestions.rs-3710 */                         predicate,
/* FP:suggestions.rs-3711 */                         param_env,
/* FP:suggestions.rs-3712 */                         parent_code,
/* FP:suggestions.rs-3713 */                         obligated_types,
/* FP:suggestions.rs-3714 */                         seen_requirements,
/* FP:suggestions.rs-3715 */                     )
/* FP:suggestions.rs-3716 */                 });
/* FP:suggestions.rs-3717 */             }
/* FP:suggestions.rs-3718 */             // Suppress `compare_type_predicate_entailment` errors for RPITITs, since they
/* FP:suggestions.rs-3719 */             // should be implied by the parent method.
/* FP:suggestions.rs-3720 */             ObligationCauseCode::CompareImplItem { trait_item_def_id, .. }
/* FP:suggestions.rs-3721 */                 if tcx.is_impl_trait_in_trait(trait_item_def_id) => {}
/* FP:suggestions.rs-3722 */             ObligationCauseCode::CompareImplItem { trait_item_def_id, kind, .. } => {
/* FP:suggestions.rs-3723 */                 let item_name = tcx.item_name(trait_item_def_id);
/* FP:suggestions.rs-3724 */                 let msg = format!(
/* FP:suggestions.rs-3725 */                     "the requirement `{predicate}` appears on the `impl`'s {kind} \
/* FP:suggestions.rs-3726 */                      `{item_name}` but not on the corresponding trait's {kind}",
/* FP:suggestions.rs-3727 */                 );
/* FP:suggestions.rs-3728 */                 let sp = tcx
/* FP:suggestions.rs-3729 */                     .opt_item_ident(trait_item_def_id)
/* FP:suggestions.rs-3730 */                     .map(|i| i.span)
/* FP:suggestions.rs-3731 */                     .unwrap_or_else(|| tcx.def_span(trait_item_def_id));
/* FP:suggestions.rs-3732 */                 let mut assoc_span: MultiSpan = sp.into();
/* FP:suggestions.rs-3733 */                 assoc_span.push_span_label(
/* FP:suggestions.rs-3734 */                     sp,
/* FP:suggestions.rs-3735 */                     format!("this trait's {kind} doesn't have the requirement `{predicate}`"),
/* FP:suggestions.rs-3736 */                 );
/* FP:suggestions.rs-3737 */                 if let Some(ident) = tcx
/* FP:suggestions.rs-3738 */                     .opt_associated_item(trait_item_def_id)
/* FP:suggestions.rs-3739 */                     .and_then(|i| tcx.opt_item_ident(i.container_id(tcx)))
/* FP:suggestions.rs-3740 */                 {
/* FP:suggestions.rs-3741 */                     assoc_span.push_span_label(ident.span, "in this trait");
/* FP:suggestions.rs-3742 */                 }
/* FP:suggestions.rs-3743 */                 err.span_note(assoc_span, msg);
/* FP:suggestions.rs-3744 */             }
/* FP:suggestions.rs-3745 */             ObligationCauseCode::TrivialBound => {
/* FP:suggestions.rs-3746 */                 err.help("see issue #48214");
/* FP:suggestions.rs-3747 */                 tcx.disabled_nightly_features(err, [(String::new(), sym::trivial_bounds)]);
/* FP:suggestions.rs-3748 */             }
/* FP:suggestions.rs-3749 */             ObligationCauseCode::OpaqueReturnType(expr_info) => {
/* FP:suggestions.rs-3750 */                 let (expr_ty, expr) = if let Some((expr_ty, hir_id)) = expr_info {
/* FP:suggestions.rs-3751 */                     let expr_ty = tcx.short_string(expr_ty, err.long_ty_path());
/* FP:suggestions.rs-3752 */                     let expr = tcx.hir_expect_expr(hir_id);
/* FP:suggestions.rs-3753 */                     (expr_ty, expr)
/* FP:suggestions.rs-3754 */                 } else if let Some(body_id) = tcx.hir_node_by_def_id(body_id).body_id()
/* FP:suggestions.rs-3755 */                     && let body = tcx.hir_body(body_id)
/* FP:suggestions.rs-3756 */                     && let hir::ExprKind::Block(block, _) = body.value.kind
/* FP:suggestions.rs-3757 */                     && let Some(expr) = block.expr
/* FP:suggestions.rs-3758 */                     && let Some(expr_ty) = self
/* FP:suggestions.rs-3759 */                         .typeck_results
/* FP:suggestions.rs-3760 */                         .as_ref()
/* FP:suggestions.rs-3761 */                         .and_then(|typeck| typeck.node_type_opt(expr.hir_id))
/* FP:suggestions.rs-3762 */                     && let Some(pred) = predicate.as_clause()
/* FP:suggestions.rs-3763 */                     && let ty::ClauseKind::Trait(pred) = pred.kind().skip_binder()
/* FP:suggestions.rs-3764 */                     && self.can_eq(param_env, pred.self_ty(), expr_ty)
/* FP:suggestions.rs-3765 */                 {
/* FP:suggestions.rs-3766 */                     let expr_ty = tcx.short_string(expr_ty, err.long_ty_path());
/* FP:suggestions.rs-3767 */                     (expr_ty, expr)
/* FP:suggestions.rs-3768 */                 } else {
/* FP:suggestions.rs-3769 */                     return;
/* FP:suggestions.rs-3770 */                 };
/* FP:suggestions.rs-3771 */                 err.span_label(
/* FP:suggestions.rs-3772 */                     expr.span,
/* FP:suggestions.rs-3773 */                     with_forced_trimmed_paths!(format!(
/* FP:suggestions.rs-3774 */                         "return type was inferred to be `{expr_ty}` here",
/* FP:suggestions.rs-3775 */                     )),
/* FP:suggestions.rs-3776 */                 );
/* FP:suggestions.rs-3777 */                 suggest_remove_deref(err, &expr);
/* FP:suggestions.rs-3778 */             }
/* FP:suggestions.rs-3779 */             ObligationCauseCode::UnsizedNonPlaceExpr(span) => {
/* FP:suggestions.rs-3780 */                 err.span_note(
/* FP:suggestions.rs-3781 */                     span,
/* FP:suggestions.rs-3782 */                     "unsized values must be place expressions and cannot be put in temporaries",
/* FP:suggestions.rs-3783 */                 );
/* FP:suggestions.rs-3784 */             }
/* FP:suggestions.rs-3785 */         }
/* FP:suggestions.rs-3786 */     }
/* FP:suggestions.rs-3787 */ 
/* FP:suggestions.rs-3788 */     #[instrument(
/* FP:suggestions.rs-3789 */         level = "debug", skip(self, err), fields(trait_pred.self_ty = ?trait_pred.self_ty())
/* FP:suggestions.rs-3790 */     )]
/* FP:suggestions.rs-3791 */     pub(super) fn suggest_await_before_try(
/* FP:suggestions.rs-3792 */         &self,
/* FP:suggestions.rs-3793 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-3794 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-3795 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-3796 */         span: Span,
/* FP:suggestions.rs-3797 */     ) {
/* FP:suggestions.rs-3798 */         let future_trait = self.tcx.require_lang_item(LangItem::Future, span);
/* FP:suggestions.rs-3799 */ 
/* FP:suggestions.rs-3800 */         let self_ty = self.resolve_vars_if_possible(trait_pred.self_ty());
/* FP:suggestions.rs-3801 */         let impls_future = self.type_implements_trait(
/* FP:suggestions.rs-3802 */             future_trait,
/* FP:suggestions.rs-3803 */             [self.tcx.instantiate_bound_regions_with_erased(self_ty)],
/* FP:suggestions.rs-3804 */             obligation.param_env,
/* FP:suggestions.rs-3805 */         );
/* FP:suggestions.rs-3806 */         if !impls_future.must_apply_modulo_regions() {
/* FP:suggestions.rs-3807 */             return;
/* FP:suggestions.rs-3808 */         }
/* FP:suggestions.rs-3809 */ 
/* FP:suggestions.rs-3810 */         let item_def_id = self.tcx.associated_item_def_ids(future_trait)[0];
/* FP:suggestions.rs-3811 */         // `<T as Future>::Output`
/* FP:suggestions.rs-3812 */         let projection_ty = trait_pred.map_bound(|trait_pred| {
/* FP:suggestions.rs-3813 */             Ty::new_projection(
/* FP:suggestions.rs-3814 */                 self.tcx,
/* FP:suggestions.rs-3815 */                 item_def_id,
/* FP:suggestions.rs-3816 */                 // Future::Output has no args
/* FP:suggestions.rs-3817 */                 [trait_pred.self_ty()],
/* FP:suggestions.rs-3818 */             )
/* FP:suggestions.rs-3819 */         });
/* FP:suggestions.rs-3820 */         let InferOk { value: projection_ty, .. } =
/* FP:suggestions.rs-3821 */             self.at(&obligation.cause, obligation.param_env).normalize(projection_ty);
/* FP:suggestions.rs-3822 */ 
/* FP:suggestions.rs-3823 */         debug!(
/* FP:suggestions.rs-3824 */             normalized_projection_type = ?self.resolve_vars_if_possible(projection_ty)
/* FP:suggestions.rs-3825 */         );
/* FP:suggestions.rs-3826 */         let try_obligation = self.mk_trait_obligation_with_new_self_ty(
/* FP:suggestions.rs-3827 */             obligation.param_env,
/* FP:suggestions.rs-3828 */             trait_pred.map_bound(|trait_pred| (trait_pred, projection_ty.skip_binder())),
/* FP:suggestions.rs-3829 */         );
/* FP:suggestions.rs-3830 */         debug!(try_trait_obligation = ?try_obligation);
/* FP:suggestions.rs-3831 */         if self.predicate_may_hold(&try_obligation)
/* FP:suggestions.rs-3832 */             && let Ok(snippet) = self.tcx.sess.source_map().span_to_snippet(span)
/* FP:suggestions.rs-3833 */             && snippet.ends_with('?')
/* FP:suggestions.rs-3834 */         {
/* FP:suggestions.rs-3835 */             match self.tcx.coroutine_kind(obligation.cause.body_id) {
/* FP:suggestions.rs-3836 */                 Some(hir::CoroutineKind::Desugared(hir::CoroutineDesugaring::Async, _)) => {
/* FP:suggestions.rs-3837 */                     err.span_suggestion_verbose(
/* FP:suggestions.rs-3838 */                         span.with_hi(span.hi() - BytePos(1)).shrink_to_hi(),
/* FP:suggestions.rs-3839 */                         "consider `await`ing on the `Future`",
/* FP:suggestions.rs-3840 */                         ".await",
/* FP:suggestions.rs-3841 */                         Applicability::MaybeIncorrect,
/* FP:suggestions.rs-3842 */                     );
/* FP:suggestions.rs-3843 */                 }
/* FP:suggestions.rs-3844 */                 _ => {
/* FP:suggestions.rs-3845 */                     let mut span: MultiSpan = span.with_lo(span.hi() - BytePos(1)).into();
/* FP:suggestions.rs-3846 */                     span.push_span_label(
/* FP:suggestions.rs-3847 */                         self.tcx.def_span(obligation.cause.body_id),
/* FP:suggestions.rs-3848 */                         "this is not `async`",
/* FP:suggestions.rs-3849 */                     );
/* FP:suggestions.rs-3850 */                     err.span_note(
/* FP:suggestions.rs-3851 */                         span,
/* FP:suggestions.rs-3852 */                         "this implements `Future` and its output type supports \
/* FP:suggestions.rs-3853 */                         `?`, but the future cannot be awaited in a synchronous function",
/* FP:suggestions.rs-3854 */                     );
/* FP:suggestions.rs-3855 */                 }
/* FP:suggestions.rs-3856 */             }
/* FP:suggestions.rs-3857 */         }
/* FP:suggestions.rs-3858 */     }
/* FP:suggestions.rs-3859 */ 
/* FP:suggestions.rs-3860 */     pub(super) fn suggest_floating_point_literal(
/* FP:suggestions.rs-3861 */         &self,
/* FP:suggestions.rs-3862 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-3863 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-3864 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-3865 */     ) {
/* FP:suggestions.rs-3866 */         let rhs_span = match obligation.cause.code() {
/* FP:suggestions.rs-3867 */             ObligationCauseCode::BinOp { rhs_span, rhs_is_lit, .. } if *rhs_is_lit => rhs_span,
/* FP:suggestions.rs-3868 */             _ => return,
/* FP:suggestions.rs-3869 */         };
/* FP:suggestions.rs-3870 */         if let ty::Float(_) = trait_pred.skip_binder().self_ty().kind()
/* FP:suggestions.rs-3871 */             && let ty::Infer(InferTy::IntVar(_)) =
/* FP:suggestions.rs-3872 */                 trait_pred.skip_binder().trait_ref.args.type_at(1).kind()
/* FP:suggestions.rs-3873 */         {
/* FP:suggestions.rs-3874 */             err.span_suggestion_verbose(
/* FP:suggestions.rs-3875 */                 rhs_span.shrink_to_hi(),
/* FP:suggestions.rs-3876 */                 "consider using a floating-point literal by writing it with `.0`",
/* FP:suggestions.rs-3877 */                 ".0",
/* FP:suggestions.rs-3878 */                 Applicability::MaybeIncorrect,
/* FP:suggestions.rs-3879 */             );
/* FP:suggestions.rs-3880 */         }
/* FP:suggestions.rs-3881 */     }
/* FP:suggestions.rs-3882 */ 
/* FP:suggestions.rs-3883 */     pub fn can_suggest_derive(
/* FP:suggestions.rs-3884 */         &self,
/* FP:suggestions.rs-3885 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-3886 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-3887 */     ) -> bool {
/* FP:suggestions.rs-3888 */         if trait_pred.polarity() == ty::PredicatePolarity::Negative {
/* FP:suggestions.rs-3889 */             return false;
/* FP:suggestions.rs-3890 */         }
/* FP:suggestions.rs-3891 */         let Some(diagnostic_name) = self.tcx.get_diagnostic_name(trait_pred.def_id()) else {
/* FP:suggestions.rs-3892 */             return false;
/* FP:suggestions.rs-3893 */         };
/* FP:suggestions.rs-3894 */         let (adt, args) = match trait_pred.skip_binder().self_ty().kind() {
/* FP:suggestions.rs-3895 */             ty::Adt(adt, args) if adt.did().is_local() => (adt, args),
/* FP:suggestions.rs-3896 */             _ => return false,
/* FP:suggestions.rs-3897 */         };
/* FP:suggestions.rs-3898 */         let is_derivable_trait = match diagnostic_name {
/* FP:suggestions.rs-3899 */             sym::Default => !adt.is_enum(),
/* FP:suggestions.rs-3900 */             sym::PartialEq | sym::PartialOrd => {
/* FP:suggestions.rs-3901 */                 let rhs_ty = trait_pred.skip_binder().trait_ref.args.type_at(1);
/* FP:suggestions.rs-3902 */                 trait_pred.skip_binder().self_ty() == rhs_ty
/* FP:suggestions.rs-3903 */             }
/* FP:suggestions.rs-3904 */             sym::Eq | sym::Ord | sym::Clone | sym::Copy | sym::Hash | sym::Debug => true,
/* FP:suggestions.rs-3905 */             _ => false,
/* FP:suggestions.rs-3906 */         };
/* FP:suggestions.rs-3907 */         is_derivable_trait &&
/* FP:suggestions.rs-3908 */             // Ensure all fields impl the trait.
/* FP:suggestions.rs-3909 */             adt.all_fields().all(|field| {
/* FP:suggestions.rs-3910 */                 let field_ty = ty::GenericArg::from(field.ty(self.tcx, args));
/* FP:suggestions.rs-3911 */                 let trait_args = match diagnostic_name {
/* FP:suggestions.rs-3912 */                     sym::PartialEq | sym::PartialOrd => {
/* FP:suggestions.rs-3913 */                         Some(field_ty)
/* FP:suggestions.rs-3914 */                     }
/* FP:suggestions.rs-3915 */                     _ => None,
/* FP:suggestions.rs-3916 */                 };
/* FP:suggestions.rs-3917 */                 let trait_pred = trait_pred.map_bound_ref(|tr| ty::TraitPredicate {
/* FP:suggestions.rs-3918 */                     trait_ref: ty::TraitRef::new(self.tcx,
/* FP:suggestions.rs-3919 */                         trait_pred.def_id(),
/* FP:suggestions.rs-3920 */                         [field_ty].into_iter().chain(trait_args),
/* FP:suggestions.rs-3921 */                     ),
/* FP:suggestions.rs-3922 */                     ..*tr
/* FP:suggestions.rs-3923 */                 });
/* FP:suggestions.rs-3924 */                 let field_obl = Obligation::new(
/* FP:suggestions.rs-3925 */                     self.tcx,
/* FP:suggestions.rs-3926 */                     obligation.cause.clone(),
/* FP:suggestions.rs-3927 */                     obligation.param_env,
/* FP:suggestions.rs-3928 */                     trait_pred,
/* FP:suggestions.rs-3929 */                 );
/* FP:suggestions.rs-3930 */                 self.predicate_must_hold_modulo_regions(&field_obl)
/* FP:suggestions.rs-3931 */             })
/* FP:suggestions.rs-3932 */     }
/* FP:suggestions.rs-3933 */ 
/* FP:suggestions.rs-3934 */     pub fn suggest_derive(
/* FP:suggestions.rs-3935 */         &self,
/* FP:suggestions.rs-3936 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-3937 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-3938 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-3939 */     ) {
/* FP:suggestions.rs-3940 */         let Some(diagnostic_name) = self.tcx.get_diagnostic_name(trait_pred.def_id()) else {
/* FP:suggestions.rs-3941 */             return;
/* FP:suggestions.rs-3942 */         };
/* FP:suggestions.rs-3943 */         let adt = match trait_pred.skip_binder().self_ty().kind() {
/* FP:suggestions.rs-3944 */             ty::Adt(adt, _) if adt.did().is_local() => adt,
/* FP:suggestions.rs-3945 */             _ => return,
/* FP:suggestions.rs-3946 */         };
/* FP:suggestions.rs-3947 */         if self.can_suggest_derive(obligation, trait_pred) {
/* FP:suggestions.rs-3948 */             err.span_suggestion_verbose(
/* FP:suggestions.rs-3949 */                 self.tcx.def_span(adt.did()).shrink_to_lo(),
/* FP:suggestions.rs-3950 */                 format!(
/* FP:suggestions.rs-3951 */                     "consider annotating `{}` with `#[derive({})]`",
/* FP:suggestions.rs-3952 */                     trait_pred.skip_binder().self_ty(),
/* FP:suggestions.rs-3953 */                     diagnostic_name,
/* FP:suggestions.rs-3954 */                 ),
/* FP:suggestions.rs-3955 */                 // FIXME(const_trait_impl) derive_const as suggestion?
/* FP:suggestions.rs-3956 */                 format!("#[derive({diagnostic_name})]\n"),
/* FP:suggestions.rs-3957 */                 Applicability::MaybeIncorrect,
/* FP:suggestions.rs-3958 */             );
/* FP:suggestions.rs-3959 */         }
/* FP:suggestions.rs-3960 */     }
/* FP:suggestions.rs-3961 */ 
/* FP:suggestions.rs-3962 */     pub(super) fn suggest_dereferencing_index(
/* FP:suggestions.rs-3963 */         &self,
/* FP:suggestions.rs-3964 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-3965 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-3966 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-3967 */     ) {
/* FP:suggestions.rs-3968 */         if let ObligationCauseCode::ImplDerived(_) = obligation.cause.code()
/* FP:suggestions.rs-3969 */             && self
/* FP:suggestions.rs-3970 */                 .tcx
/* FP:suggestions.rs-3971 */                 .is_diagnostic_item(sym::SliceIndex, trait_pred.skip_binder().trait_ref.def_id)
/* FP:suggestions.rs-3972 */             && let ty::Slice(_) = trait_pred.skip_binder().trait_ref.args.type_at(1).kind()
/* FP:suggestions.rs-3973 */             && let ty::Ref(_, inner_ty, _) = trait_pred.skip_binder().self_ty().kind()
/* FP:suggestions.rs-3974 */             && let ty::Uint(ty::UintTy::Usize) = inner_ty.kind()
/* FP:suggestions.rs-3975 */         {
/* FP:suggestions.rs-3976 */             err.span_suggestion_verbose(
/* FP:suggestions.rs-3977 */                 obligation.cause.span.shrink_to_lo(),
/* FP:suggestions.rs-3978 */                 "dereference this index",
/* FP:suggestions.rs-3979 */                 '*',
/* FP:suggestions.rs-3980 */                 Applicability::MachineApplicable,
/* FP:suggestions.rs-3981 */             );
/* FP:suggestions.rs-3982 */         }
/* FP:suggestions.rs-3983 */     }
/* FP:suggestions.rs-3984 */ 
/* FP:suggestions.rs-3985 */     fn note_function_argument_obligation<G: EmissionGuarantee>(
/* FP:suggestions.rs-3986 */         &self,
/* FP:suggestions.rs-3987 */         body_id: LocalDefId,
/* FP:suggestions.rs-3988 */         err: &mut Diag<'_, G>,
/* FP:suggestions.rs-3989 */         arg_hir_id: HirId,
/* FP:suggestions.rs-3990 */         parent_code: &ObligationCauseCode<'tcx>,
/* FP:suggestions.rs-3991 */         param_env: ty::ParamEnv<'tcx>,
/* FP:suggestions.rs-3992 */         failed_pred: ty::Predicate<'tcx>,
/* FP:suggestions.rs-3993 */         call_hir_id: HirId,
/* FP:suggestions.rs-3994 */     ) {
/* FP:suggestions.rs-3995 */         let tcx = self.tcx;
/* FP:suggestions.rs-3996 */         if let Node::Expr(expr) = tcx.hir_node(arg_hir_id)
/* FP:suggestions.rs-3997 */             && let Some(typeck_results) = &self.typeck_results
/* FP:suggestions.rs-3998 */         {
/* FP:suggestions.rs-3999 */             if let hir::Expr { kind: hir::ExprKind::MethodCall(_, rcvr, _, _), .. } = expr
/* FP:suggestions.rs-4000 */                 && let Some(ty) = typeck_results.node_type_opt(rcvr.hir_id)
/* FP:suggestions.rs-4001 */                 && let Some(failed_pred) = failed_pred.as_trait_clause()
/* FP:suggestions.rs-4002 */                 && let pred = failed_pred.map_bound(|pred| pred.with_replaced_self_ty(tcx, ty))
/* FP:suggestions.rs-4003 */                 && self.predicate_must_hold_modulo_regions(&Obligation::misc(
/* FP:suggestions.rs-4004 */                     tcx, expr.span, body_id, param_env, pred,
/* FP:suggestions.rs-4005 */                 ))
/* FP:suggestions.rs-4006 */                 && expr.span.hi() != rcvr.span.hi()
/* FP:suggestions.rs-4007 */             {
/* FP:suggestions.rs-4008 */                 err.span_suggestion_verbose(
/* FP:suggestions.rs-4009 */                     expr.span.with_lo(rcvr.span.hi()),
/* FP:suggestions.rs-4010 */                     format!(
/* FP:suggestions.rs-4011 */                         "consider removing this method call, as the receiver has type `{ty}` and \
/* FP:suggestions.rs-4012 */                          `{pred}` trivially holds",
/* FP:suggestions.rs-4013 */                     ),
/* FP:suggestions.rs-4014 */                     "",
/* FP:suggestions.rs-4015 */                     Applicability::MaybeIncorrect,
/* FP:suggestions.rs-4016 */                 );
/* FP:suggestions.rs-4017 */             }
/* FP:suggestions.rs-4018 */             if let hir::Expr { kind: hir::ExprKind::Block(block, _), .. } = expr {
/* FP:suggestions.rs-4019 */                 let inner_expr = expr.peel_blocks();
/* FP:suggestions.rs-4020 */                 let ty = typeck_results
/* FP:suggestions.rs-4021 */                     .expr_ty_adjusted_opt(inner_expr)
/* FP:suggestions.rs-4022 */                     .unwrap_or(Ty::new_misc_error(tcx));
/* FP:suggestions.rs-4023 */                 let span = inner_expr.span;
/* FP:suggestions.rs-4024 */                 if Some(span) != err.span.primary_span()
/* FP:suggestions.rs-4025 */                     && !span.in_external_macro(tcx.sess.source_map())
/* FP:suggestions.rs-4026 */                 {
/* FP:suggestions.rs-4027 */                     err.span_label(
/* FP:suggestions.rs-4028 */                         span,
/* FP:suggestions.rs-4029 */                         if ty.references_error() {
/* FP:suggestions.rs-4030 */                             String::new()
/* FP:suggestions.rs-4031 */                         } else {
/* FP:suggestions.rs-4032 */                             let ty = with_forced_trimmed_paths!(self.ty_to_string(ty));
/* FP:suggestions.rs-4033 */                             format!("this tail expression is of type `{ty}`")
/* FP:suggestions.rs-4034 */                         },
/* FP:suggestions.rs-4035 */                     );
/* FP:suggestions.rs-4036 */                     if let ty::PredicateKind::Clause(clause) = failed_pred.kind().skip_binder()
/* FP:suggestions.rs-4037 */                         && let ty::ClauseKind::Trait(pred) = clause
/* FP:suggestions.rs-4038 */                         && tcx.fn_trait_kind_from_def_id(pred.def_id()).is_some()
/* FP:suggestions.rs-4039 */                     {
/* FP:suggestions.rs-4040 */                         if let [stmt, ..] = block.stmts
/* FP:suggestions.rs-4041 */                             && let hir::StmtKind::Semi(value) = stmt.kind
/* FP:suggestions.rs-4042 */                             && let hir::ExprKind::Closure(hir::Closure {
/* FP:suggestions.rs-4043 */                                 body, fn_decl_span, ..
/* FP:suggestions.rs-4044 */                             }) = value.kind
/* FP:suggestions.rs-4045 */                             && let body = tcx.hir_body(*body)
/* FP:suggestions.rs-4046 */                             && !matches!(body.value.kind, hir::ExprKind::Block(..))
/* FP:suggestions.rs-4047 */                         {
/* FP:suggestions.rs-4048 */                             // Check if the failed predicate was an expectation of a closure type
/* FP:suggestions.rs-4049 */                             // and if there might have been a `{ |args|` typo instead of `|args| {`.
/* FP:suggestions.rs-4050 */                             err.multipart_suggestion(
/* FP:suggestions.rs-4051 */                                 "you might have meant to open the closure body instead of placing \
/* FP:suggestions.rs-4052 */                                  a closure within a block",
/* FP:suggestions.rs-4053 */                                 vec![
/* FP:suggestions.rs-4054 */                                     (expr.span.with_hi(value.span.lo()), String::new()),
/* FP:suggestions.rs-4055 */                                     (fn_decl_span.shrink_to_hi(), " {".to_string()),
/* FP:suggestions.rs-4056 */                                 ],
/* FP:suggestions.rs-4057 */                                 Applicability::MaybeIncorrect,
/* FP:suggestions.rs-4058 */                             );
/* FP:suggestions.rs-4059 */                         } else {
/* FP:suggestions.rs-4060 */                             // Maybe the bare block was meant to be a closure.
/* FP:suggestions.rs-4061 */                             err.span_suggestion_verbose(
/* FP:suggestions.rs-4062 */                                 expr.span.shrink_to_lo(),
/* FP:suggestions.rs-4063 */                                 "you might have meant to create the closure instead of a block",
/* FP:suggestions.rs-4064 */                                 format!(
/* FP:suggestions.rs-4065 */                                     "|{}| ",
/* FP:suggestions.rs-4066 */                                     (0..pred.trait_ref.args.len() - 1)
/* FP:suggestions.rs-4067 */                                         .map(|_| "_")
/* FP:suggestions.rs-4068 */                                         .collect::<Vec<_>>()
/* FP:suggestions.rs-4069 */                                         .join(", ")
/* FP:suggestions.rs-4070 */                                 ),
/* FP:suggestions.rs-4071 */                                 Applicability::MaybeIncorrect,
/* FP:suggestions.rs-4072 */                             );
/* FP:suggestions.rs-4073 */                         }
/* FP:suggestions.rs-4074 */                     }
/* FP:suggestions.rs-4075 */                 }
/* FP:suggestions.rs-4076 */             }
/* FP:suggestions.rs-4077 */ 
/* FP:suggestions.rs-4078 */             // FIXME: visit the ty to see if there's any closure involved, and if there is,
/* FP:suggestions.rs-4079 */             // check whether its evaluated return type is the same as the one corresponding
/* FP:suggestions.rs-4080 */             // to an associated type (as seen from `trait_pred`) in the predicate. Like in
/* FP:suggestions.rs-4081 */             // trait_pred `S: Sum<<Self as Iterator>::Item>` and predicate `i32: Sum<&()>`
/* FP:suggestions.rs-4082 */             let mut type_diffs = vec![];
/* FP:suggestions.rs-4083 */             if let ObligationCauseCode::WhereClauseInExpr(def_id, _, _, idx) = parent_code
/* FP:suggestions.rs-4084 */                 && let Some(node_args) = typeck_results.node_args_opt(call_hir_id)
/* FP:suggestions.rs-4085 */                 && let where_clauses =
/* FP:suggestions.rs-4086 */                     self.tcx.predicates_of(def_id).instantiate(self.tcx, node_args)
/* FP:suggestions.rs-4087 */                 && let Some(where_pred) = where_clauses.predicates.get(*idx)
/* FP:suggestions.rs-4088 */             {
/* FP:suggestions.rs-4089 */                 if let Some(where_pred) = where_pred.as_trait_clause()
/* FP:suggestions.rs-4090 */                     && let Some(failed_pred) = failed_pred.as_trait_clause()
/* FP:suggestions.rs-4091 */                     && where_pred.def_id() == failed_pred.def_id()
/* FP:suggestions.rs-4092 */                 {
/* FP:suggestions.rs-4093 */                     self.enter_forall(where_pred, |where_pred| {
/* FP:suggestions.rs-4094 */                         let failed_pred = self.instantiate_binder_with_fresh_vars(
/* FP:suggestions.rs-4095 */                             expr.span,
/* FP:suggestions.rs-4096 */                             BoundRegionConversionTime::FnCall,
/* FP:suggestions.rs-4097 */                             failed_pred,
/* FP:suggestions.rs-4098 */                         );
/* FP:suggestions.rs-4099 */ 
/* FP:suggestions.rs-4100 */                         let zipped =
/* FP:suggestions.rs-4101 */                             iter::zip(where_pred.trait_ref.args, failed_pred.trait_ref.args);
/* FP:suggestions.rs-4102 */                         for (expected, actual) in zipped {
/* FP:suggestions.rs-4103 */                             self.probe(|_| {
/* FP:suggestions.rs-4104 */                                 match self
/* FP:suggestions.rs-4105 */                                     .at(&ObligationCause::misc(expr.span, body_id), param_env)
/* FP:suggestions.rs-4106 */                                     // Doesn't actually matter if we define opaque types here, this is just used for
/* FP:suggestions.rs-4107 */                                     // diagnostics, and the result is never kept around.
/* FP:suggestions.rs-4108 */                                     .eq(DefineOpaqueTypes::Yes, expected, actual)
/* FP:suggestions.rs-4109 */                                 {
/* FP:suggestions.rs-4110 */                                     Ok(_) => (), // We ignore nested obligations here for now.
/* FP:suggestions.rs-4111 */                                     Err(err) => type_diffs.push(err),
/* FP:suggestions.rs-4112 */                                 }
/* FP:suggestions.rs-4113 */                             })
/* FP:suggestions.rs-4114 */                         }
/* FP:suggestions.rs-4115 */                     })
/* FP:suggestions.rs-4116 */                 } else if let Some(where_pred) = where_pred.as_projection_clause()
/* FP:suggestions.rs-4117 */                     && let Some(failed_pred) = failed_pred.as_projection_clause()
/* FP:suggestions.rs-4118 */                     && let Some(found) = failed_pred.skip_binder().term.as_type()
/* FP:suggestions.rs-4119 */                 {
/* FP:suggestions.rs-4120 */                     type_diffs = vec![TypeError::Sorts(ty::error::ExpectedFound {
/* FP:suggestions.rs-4121 */                         expected: where_pred
/* FP:suggestions.rs-4122 */                             .skip_binder()
/* FP:suggestions.rs-4123 */                             .projection_term
/* FP:suggestions.rs-4124 */                             .expect_ty(self.tcx)
/* FP:suggestions.rs-4125 */                             .to_ty(self.tcx),
/* FP:suggestions.rs-4126 */                         found,
/* FP:suggestions.rs-4127 */                     })];
/* FP:suggestions.rs-4128 */                 }
/* FP:suggestions.rs-4129 */             }
/* FP:suggestions.rs-4130 */             if let hir::ExprKind::Path(hir::QPath::Resolved(None, path)) = expr.kind
/* FP:suggestions.rs-4131 */                 && let hir::Path { res: Res::Local(hir_id), .. } = path
/* FP:suggestions.rs-4132 */                 && let hir::Node::Pat(binding) = self.tcx.hir_node(*hir_id)
/* FP:suggestions.rs-4133 */                 && let hir::Node::LetStmt(local) = self.tcx.parent_hir_node(binding.hir_id)
/* FP:suggestions.rs-4134 */                 && let Some(binding_expr) = local.init
/* FP:suggestions.rs-4135 */             {
/* FP:suggestions.rs-4136 */                 // If the expression we're calling on is a binding, we want to point at the
/* FP:suggestions.rs-4137 */                 // `let` when talking about the type. Otherwise we'll point at every part
/* FP:suggestions.rs-4138 */                 // of the method chain with the type.
/* FP:suggestions.rs-4139 */                 self.point_at_chain(binding_expr, typeck_results, type_diffs, param_env, err);
/* FP:suggestions.rs-4140 */             } else {
/* FP:suggestions.rs-4141 */                 self.point_at_chain(expr, typeck_results, type_diffs, param_env, err);
/* FP:suggestions.rs-4142 */             }
/* FP:suggestions.rs-4143 */         }
/* FP:suggestions.rs-4144 */         let call_node = tcx.hir_node(call_hir_id);
/* FP:suggestions.rs-4145 */         if let Node::Expr(hir::Expr { kind: hir::ExprKind::MethodCall(path, rcvr, ..), .. }) =
/* FP:suggestions.rs-4146 */             call_node
/* FP:suggestions.rs-4147 */         {
/* FP:suggestions.rs-4148 */             if Some(rcvr.span) == err.span.primary_span() {
/* FP:suggestions.rs-4149 */                 err.replace_span_with(path.ident.span, true);
/* FP:suggestions.rs-4150 */             }
/* FP:suggestions.rs-4151 */         }
/* FP:suggestions.rs-4152 */ 
/* FP:suggestions.rs-4153 */         if let Node::Expr(expr) = call_node {
/* FP:suggestions.rs-4154 */             if let hir::ExprKind::Call(hir::Expr { span, .. }, _)
/* FP:suggestions.rs-4155 */             | hir::ExprKind::MethodCall(
/* FP:suggestions.rs-4156 */                 hir::PathSegment { ident: Ident { span, .. }, .. },
/* FP:suggestions.rs-4157 */                 ..,
/* FP:suggestions.rs-4158 */             ) = expr.kind
/* FP:suggestions.rs-4159 */             {
/* FP:suggestions.rs-4160 */                 if Some(*span) != err.span.primary_span() {
/* FP:suggestions.rs-4161 */                     let msg = if span.is_desugaring(DesugaringKind::FormatLiteral { source: true })
/* FP:suggestions.rs-4162 */                     {
/* FP:suggestions.rs-4163 */                         "required by this formatting parameter"
/* FP:suggestions.rs-4164 */                     } else if span.is_desugaring(DesugaringKind::FormatLiteral { source: false }) {
/* FP:suggestions.rs-4165 */                         "required by a formatting parameter in this expression"
/* FP:suggestions.rs-4166 */                     } else {
/* FP:suggestions.rs-4167 */                         "required by a bound introduced by this call"
/* FP:suggestions.rs-4168 */                     };
/* FP:suggestions.rs-4169 */                     err.span_label(*span, msg);
/* FP:suggestions.rs-4170 */                 }
/* FP:suggestions.rs-4171 */             }
/* FP:suggestions.rs-4172 */ 
/* FP:suggestions.rs-4173 */             if let hir::ExprKind::MethodCall(_, expr, ..) = expr.kind {
/* FP:suggestions.rs-4174 */                 self.suggest_option_method_if_applicable(failed_pred, param_env, err, expr);
/* FP:suggestions.rs-4175 */             }
/* FP:suggestions.rs-4176 */         }
/* FP:suggestions.rs-4177 */     }
/* FP:suggestions.rs-4178 */ 
/* FP:suggestions.rs-4179 */     fn suggest_option_method_if_applicable<G: EmissionGuarantee>(
/* FP:suggestions.rs-4180 */         &self,
/* FP:suggestions.rs-4181 */         failed_pred: ty::Predicate<'tcx>,
/* FP:suggestions.rs-4182 */         param_env: ty::ParamEnv<'tcx>,
/* FP:suggestions.rs-4183 */         err: &mut Diag<'_, G>,
/* FP:suggestions.rs-4184 */         expr: &hir::Expr<'_>,
/* FP:suggestions.rs-4185 */     ) {
/* FP:suggestions.rs-4186 */         let tcx = self.tcx;
/* FP:suggestions.rs-4187 */         let infcx = self.infcx;
/* FP:suggestions.rs-4188 */         let Some(typeck_results) = self.typeck_results.as_ref() else { return };
/* FP:suggestions.rs-4189 */ 
/* FP:suggestions.rs-4190 */         // Make sure we're dealing with the `Option` type.
/* FP:suggestions.rs-4191 */         let Some(option_ty_adt) = typeck_results.expr_ty_adjusted(expr).ty_adt_def() else {
/* FP:suggestions.rs-4192 */             return;
/* FP:suggestions.rs-4193 */         };
/* FP:suggestions.rs-4194 */         if !tcx.is_diagnostic_item(sym::Option, option_ty_adt.did()) {
/* FP:suggestions.rs-4195 */             return;
/* FP:suggestions.rs-4196 */         }
/* FP:suggestions.rs-4197 */ 
/* FP:suggestions.rs-4198 */         // Given the predicate `fn(&T): FnOnce<(U,)>`, extract `fn(&T)` and `(U,)`,
/* FP:suggestions.rs-4199 */         // then suggest `Option::as_deref(_mut)` if `U` can deref to `T`
/* FP:suggestions.rs-4200 */         if let ty::PredicateKind::Clause(ty::ClauseKind::Trait(ty::TraitPredicate { trait_ref, .. }))
/* FP:suggestions.rs-4201 */             = failed_pred.kind().skip_binder()
/* FP:suggestions.rs-4202 */             && tcx.is_fn_trait(trait_ref.def_id)
/* FP:suggestions.rs-4203 */             && let [self_ty, found_ty] = trait_ref.args.as_slice()
/* FP:suggestions.rs-4204 */             && let Some(fn_ty) = self_ty.as_type().filter(|ty| ty.is_fn())
/* FP:suggestions.rs-4205 */             && let fn_sig @ ty::FnSig {
/* FP:suggestions.rs-4206 */                 abi: ExternAbi::Rust,
/* FP:suggestions.rs-4207 */                 c_variadic: false,
/* FP:suggestions.rs-4208 */                 safety: hir::Safety::Safe,
/* FP:suggestions.rs-4209 */                 ..
/* FP:suggestions.rs-4210 */             } = fn_ty.fn_sig(tcx).skip_binder()
/* FP:suggestions.rs-4211 */ 
/* FP:suggestions.rs-4212 */             // Extract first param of fn sig with peeled refs, e.g. `fn(&T)` -> `T`
/* FP:suggestions.rs-4213 */             && let Some(&ty::Ref(_, target_ty, needs_mut)) = fn_sig.inputs().first().map(|t| t.kind())
/* FP:suggestions.rs-4214 */             && !target_ty.has_escaping_bound_vars()
/* FP:suggestions.rs-4215 */ 
/* FP:suggestions.rs-4216 */             // Extract first tuple element out of fn trait, e.g. `FnOnce<(U,)>` -> `U`
/* FP:suggestions.rs-4217 */             && let Some(ty::Tuple(tys)) = found_ty.as_type().map(Ty::kind)
/* FP:suggestions.rs-4218 */             && let &[found_ty] = tys.as_slice()
/* FP:suggestions.rs-4219 */             && !found_ty.has_escaping_bound_vars()
/* FP:suggestions.rs-4220 */ 
/* FP:suggestions.rs-4221 */             // Extract `<U as Deref>::Target` assoc type and check that it is `T`
/* FP:suggestions.rs-4222 */             && let Some(deref_target_did) = tcx.lang_items().deref_target()
/* FP:suggestions.rs-4223 */             && let projection = Ty::new_projection_from_args(tcx,deref_target_did, tcx.mk_args(&[ty::GenericArg::from(found_ty)]))
/* FP:suggestions.rs-4224 */             && let InferOk { value: deref_target, obligations } = infcx.at(&ObligationCause::dummy(), param_env).normalize(projection)
/* FP:suggestions.rs-4225 */             && obligations.iter().all(|obligation| infcx.predicate_must_hold_modulo_regions(obligation))
/* FP:suggestions.rs-4226 */             && infcx.can_eq(param_env, deref_target, target_ty)
/* FP:suggestions.rs-4227 */         {
/* FP:suggestions.rs-4228 */             let help = if let hir::Mutability::Mut = needs_mut
/* FP:suggestions.rs-4229 */                 && let Some(deref_mut_did) = tcx.lang_items().deref_mut_trait()
/* FP:suggestions.rs-4230 */                 && infcx
/* FP:suggestions.rs-4231 */                     .type_implements_trait(deref_mut_did, iter::once(found_ty), param_env)
/* FP:suggestions.rs-4232 */                     .must_apply_modulo_regions()
/* FP:suggestions.rs-4233 */             {
/* FP:suggestions.rs-4234 */                 Some(("call `Option::as_deref_mut()` first", ".as_deref_mut()"))
/* FP:suggestions.rs-4235 */             } else if let hir::Mutability::Not = needs_mut {
/* FP:suggestions.rs-4236 */                 Some(("call `Option::as_deref()` first", ".as_deref()"))
/* FP:suggestions.rs-4237 */             } else {
/* FP:suggestions.rs-4238 */                 None
/* FP:suggestions.rs-4239 */             };
/* FP:suggestions.rs-4240 */ 
/* FP:suggestions.rs-4241 */             if let Some((msg, sugg)) = help {
/* FP:suggestions.rs-4242 */                 err.span_suggestion_with_style(
/* FP:suggestions.rs-4243 */                     expr.span.shrink_to_hi(),
/* FP:suggestions.rs-4244 */                     msg,
/* FP:suggestions.rs-4245 */                     sugg,
/* FP:suggestions.rs-4246 */                     Applicability::MaybeIncorrect,
/* FP:suggestions.rs-4247 */                     SuggestionStyle::ShowAlways,
/* FP:suggestions.rs-4248 */                 );
/* FP:suggestions.rs-4249 */             }
/* FP:suggestions.rs-4250 */         }
/* FP:suggestions.rs-4251 */     }
/* FP:suggestions.rs-4252 */ 
/* FP:suggestions.rs-4253 */     fn look_for_iterator_item_mistakes<G: EmissionGuarantee>(
/* FP:suggestions.rs-4254 */         &self,
/* FP:suggestions.rs-4255 */         assocs_in_this_method: &[Option<(Span, (DefId, Ty<'tcx>))>],
/* FP:suggestions.rs-4256 */         typeck_results: &TypeckResults<'tcx>,
/* FP:suggestions.rs-4257 */         type_diffs: &[TypeError<'tcx>],
/* FP:suggestions.rs-4258 */         param_env: ty::ParamEnv<'tcx>,
/* FP:suggestions.rs-4259 */         path_segment: &hir::PathSegment<'_>,
/* FP:suggestions.rs-4260 */         args: &[hir::Expr<'_>],
/* FP:suggestions.rs-4261 */         err: &mut Diag<'_, G>,
/* FP:suggestions.rs-4262 */     ) {
/* FP:suggestions.rs-4263 */         let tcx = self.tcx;
/* FP:suggestions.rs-4264 */         // Special case for iterator chains, we look at potential failures of `Iterator::Item`
/* FP:suggestions.rs-4265 */         // not being `: Clone` and `Iterator::map` calls with spurious trailing `;`.
/* FP:suggestions.rs-4266 */         for entry in assocs_in_this_method {
/* FP:suggestions.rs-4267 */             let Some((_span, (def_id, ty))) = entry else {
/* FP:suggestions.rs-4268 */                 continue;
/* FP:suggestions.rs-4269 */             };
/* FP:suggestions.rs-4270 */             for diff in type_diffs {
/* FP:suggestions.rs-4271 */                 let TypeError::Sorts(expected_found) = diff else {
/* FP:suggestions.rs-4272 */                     continue;
/* FP:suggestions.rs-4273 */                 };
/* FP:suggestions.rs-4274 */                 if tcx.is_diagnostic_item(sym::IteratorItem, *def_id)
/* FP:suggestions.rs-4275 */                     && path_segment.ident.name == sym::map
/* FP:suggestions.rs-4276 */                     && self.can_eq(param_env, expected_found.found, *ty)
/* FP:suggestions.rs-4277 */                     && let [arg] = args
/* FP:suggestions.rs-4278 */                     && let hir::ExprKind::Closure(closure) = arg.kind
/* FP:suggestions.rs-4279 */                 {
/* FP:suggestions.rs-4280 */                     let body = tcx.hir_body(closure.body);
/* FP:suggestions.rs-4281 */                     if let hir::ExprKind::Block(block, None) = body.value.kind
/* FP:suggestions.rs-4282 */                         && let None = block.expr
/* FP:suggestions.rs-4283 */                         && let [.., stmt] = block.stmts
/* FP:suggestions.rs-4284 */                         && let hir::StmtKind::Semi(expr) = stmt.kind
/* FP:suggestions.rs-4285 */                         // FIXME: actually check the expected vs found types, but right now
/* FP:suggestions.rs-4286 */                         // the expected is a projection that we need to resolve.
/* FP:suggestions.rs-4287 */                         // && let Some(tail_ty) = typeck_results.expr_ty_opt(expr)
/* FP:suggestions.rs-4288 */                         && expected_found.found.is_unit()
/* FP:suggestions.rs-4289 */                         // FIXME: this happens with macro calls. Need to figure out why the stmt
/* FP:suggestions.rs-4290 */                         // `println!();` doesn't include the `;` in its `Span`. (#133845)
/* FP:suggestions.rs-4291 */                         // We filter these out to avoid ICEs with debug assertions on caused by
/* FP:suggestions.rs-4292 */                         // empty suggestions.
/* FP:suggestions.rs-4293 */                         && expr.span.hi() != stmt.span.hi()
/* FP:suggestions.rs-4294 */                     {
/* FP:suggestions.rs-4295 */                         err.span_suggestion_verbose(
/* FP:suggestions.rs-4296 */                             expr.span.shrink_to_hi().with_hi(stmt.span.hi()),
/* FP:suggestions.rs-4297 */                             "consider removing this semicolon",
/* FP:suggestions.rs-4298 */                             String::new(),
/* FP:suggestions.rs-4299 */                             Applicability::MachineApplicable,
/* FP:suggestions.rs-4300 */                         );
/* FP:suggestions.rs-4301 */                     }
/* FP:suggestions.rs-4302 */                     let expr = if let hir::ExprKind::Block(block, None) = body.value.kind
/* FP:suggestions.rs-4303 */                         && let Some(expr) = block.expr
/* FP:suggestions.rs-4304 */                     {
/* FP:suggestions.rs-4305 */                         expr
/* FP:suggestions.rs-4306 */                     } else {
/* FP:suggestions.rs-4307 */                         body.value
/* FP:suggestions.rs-4308 */                     };
/* FP:suggestions.rs-4309 */                     if let hir::ExprKind::MethodCall(path_segment, rcvr, [], span) = expr.kind
/* FP:suggestions.rs-4310 */                         && path_segment.ident.name == sym::clone
/* FP:suggestions.rs-4311 */                         && let Some(expr_ty) = typeck_results.expr_ty_opt(expr)
/* FP:suggestions.rs-4312 */                         && let Some(rcvr_ty) = typeck_results.expr_ty_opt(rcvr)
/* FP:suggestions.rs-4313 */                         && self.can_eq(param_env, expr_ty, rcvr_ty)
/* FP:suggestions.rs-4314 */                         && let ty::Ref(_, ty, _) = expr_ty.kind()
/* FP:suggestions.rs-4315 */                     {
/* FP:suggestions.rs-4316 */                         err.span_label(
/* FP:suggestions.rs-4317 */                             span,
/* FP:suggestions.rs-4318 */                             format!(
/* FP:suggestions.rs-4319 */                                 "this method call is cloning the reference `{expr_ty}`, not \
/* FP:suggestions.rs-4320 */                                  `{ty}` which doesn't implement `Clone`",
/* FP:suggestions.rs-4321 */                             ),
/* FP:suggestions.rs-4322 */                         );
/* FP:suggestions.rs-4323 */                         let ty::Param(..) = ty.kind() else {
/* FP:suggestions.rs-4324 */                             continue;
/* FP:suggestions.rs-4325 */                         };
/* FP:suggestions.rs-4326 */                         let node =
/* FP:suggestions.rs-4327 */                             tcx.hir_node_by_def_id(tcx.hir_get_parent_item(expr.hir_id).def_id);
/* FP:suggestions.rs-4328 */ 
/* FP:suggestions.rs-4329 */                         let pred = ty::Binder::dummy(ty::TraitPredicate {
/* FP:suggestions.rs-4330 */                             trait_ref: ty::TraitRef::new(
/* FP:suggestions.rs-4331 */                                 tcx,
/* FP:suggestions.rs-4332 */                                 tcx.require_lang_item(LangItem::Clone, span),
/* FP:suggestions.rs-4333 */                                 [*ty],
/* FP:suggestions.rs-4334 */                             ),
/* FP:suggestions.rs-4335 */                             polarity: ty::PredicatePolarity::Positive,
/* FP:suggestions.rs-4336 */                         });
/* FP:suggestions.rs-4337 */                         let Some(generics) = node.generics() else {
/* FP:suggestions.rs-4338 */                             continue;
/* FP:suggestions.rs-4339 */                         };
/* FP:suggestions.rs-4340 */                         let Some(body_id) = node.body_id() else {
/* FP:suggestions.rs-4341 */                             continue;
/* FP:suggestions.rs-4342 */                         };
/* FP:suggestions.rs-4343 */                         suggest_restriction(
/* FP:suggestions.rs-4344 */                             tcx,
/* FP:suggestions.rs-4345 */                             tcx.hir_body_owner_def_id(body_id),
/* FP:suggestions.rs-4346 */                             generics,
/* FP:suggestions.rs-4347 */                             &format!("type parameter `{ty}`"),
/* FP:suggestions.rs-4348 */                             err,
/* FP:suggestions.rs-4349 */                             node.fn_sig(),
/* FP:suggestions.rs-4350 */                             None,
/* FP:suggestions.rs-4351 */                             pred,
/* FP:suggestions.rs-4352 */                             None,
/* FP:suggestions.rs-4353 */                         );
/* FP:suggestions.rs-4354 */                     }
/* FP:suggestions.rs-4355 */                 }
/* FP:suggestions.rs-4356 */             }
/* FP:suggestions.rs-4357 */         }
/* FP:suggestions.rs-4358 */     }
/* FP:suggestions.rs-4359 */ 
/* FP:suggestions.rs-4360 */     fn point_at_chain<G: EmissionGuarantee>(
/* FP:suggestions.rs-4361 */         &self,
/* FP:suggestions.rs-4362 */         expr: &hir::Expr<'_>,
/* FP:suggestions.rs-4363 */         typeck_results: &TypeckResults<'tcx>,
/* FP:suggestions.rs-4364 */         type_diffs: Vec<TypeError<'tcx>>,
/* FP:suggestions.rs-4365 */         param_env: ty::ParamEnv<'tcx>,
/* FP:suggestions.rs-4366 */         err: &mut Diag<'_, G>,
/* FP:suggestions.rs-4367 */     ) {
/* FP:suggestions.rs-4368 */         let mut primary_spans = vec![];
/* FP:suggestions.rs-4369 */         let mut span_labels = vec![];
/* FP:suggestions.rs-4370 */ 
/* FP:suggestions.rs-4371 */         let tcx = self.tcx;
/* FP:suggestions.rs-4372 */ 
/* FP:suggestions.rs-4373 */         let mut print_root_expr = true;
/* FP:suggestions.rs-4374 */         let mut assocs = vec![];
/* FP:suggestions.rs-4375 */         let mut expr = expr;
/* FP:suggestions.rs-4376 */         let mut prev_ty = self.resolve_vars_if_possible(
/* FP:suggestions.rs-4377 */             typeck_results.expr_ty_adjusted_opt(expr).unwrap_or(Ty::new_misc_error(tcx)),
/* FP:suggestions.rs-4378 */         );
/* FP:suggestions.rs-4379 */         while let hir::ExprKind::MethodCall(path_segment, rcvr_expr, args, span) = expr.kind {
/* FP:suggestions.rs-4380 */             // Point at every method call in the chain with the resulting type.
/* FP:suggestions.rs-4381 */             // vec![1, 2, 3].iter().map(mapper).sum<i32>()
/* FP:suggestions.rs-4382 */             //               ^^^^^^ ^^^^^^^^^^^
/* FP:suggestions.rs-4383 */             expr = rcvr_expr;
/* FP:suggestions.rs-4384 */             let assocs_in_this_method =
/* FP:suggestions.rs-4385 */                 self.probe_assoc_types_at_expr(&type_diffs, span, prev_ty, expr.hir_id, param_env);
/* FP:suggestions.rs-4386 */             self.look_for_iterator_item_mistakes(
/* FP:suggestions.rs-4387 */                 &assocs_in_this_method,
/* FP:suggestions.rs-4388 */                 typeck_results,
/* FP:suggestions.rs-4389 */                 &type_diffs,
/* FP:suggestions.rs-4390 */                 param_env,
/* FP:suggestions.rs-4391 */                 path_segment,
/* FP:suggestions.rs-4392 */                 args,
/* FP:suggestions.rs-4393 */                 err,
/* FP:suggestions.rs-4394 */             );
/* FP:suggestions.rs-4395 */             assocs.push(assocs_in_this_method);
/* FP:suggestions.rs-4396 */             prev_ty = self.resolve_vars_if_possible(
/* FP:suggestions.rs-4397 */                 typeck_results.expr_ty_adjusted_opt(expr).unwrap_or(Ty::new_misc_error(tcx)),
/* FP:suggestions.rs-4398 */             );
/* FP:suggestions.rs-4399 */ 
/* FP:suggestions.rs-4400 */             if let hir::ExprKind::Path(hir::QPath::Resolved(None, path)) = expr.kind
/* FP:suggestions.rs-4401 */                 && let hir::Path { res: Res::Local(hir_id), .. } = path
/* FP:suggestions.rs-4402 */                 && let hir::Node::Pat(binding) = self.tcx.hir_node(*hir_id)
/* FP:suggestions.rs-4403 */             {
/* FP:suggestions.rs-4404 */                 let parent = self.tcx.parent_hir_node(binding.hir_id);
/* FP:suggestions.rs-4405 */                 // We've reached the root of the method call chain...
/* FP:suggestions.rs-4406 */                 if let hir::Node::LetStmt(local) = parent
/* FP:suggestions.rs-4407 */                     && let Some(binding_expr) = local.init
/* FP:suggestions.rs-4408 */                 {
/* FP:suggestions.rs-4409 */                     // ...and it is a binding. Get the binding creation and continue the chain.
/* FP:suggestions.rs-4410 */                     expr = binding_expr;
/* FP:suggestions.rs-4411 */                 }
/* FP:suggestions.rs-4412 */                 if let hir::Node::Param(param) = parent {
/* FP:suggestions.rs-4413 */                     // ...and it is an fn argument.
/* FP:suggestions.rs-4414 */                     let prev_ty = self.resolve_vars_if_possible(
/* FP:suggestions.rs-4415 */                         typeck_results
/* FP:suggestions.rs-4416 */                             .node_type_opt(param.hir_id)
/* FP:suggestions.rs-4417 */                             .unwrap_or(Ty::new_misc_error(tcx)),
/* FP:suggestions.rs-4418 */                     );
/* FP:suggestions.rs-4419 */                     let assocs_in_this_method = self.probe_assoc_types_at_expr(
/* FP:suggestions.rs-4420 */                         &type_diffs,
/* FP:suggestions.rs-4421 */                         param.ty_span,
/* FP:suggestions.rs-4422 */                         prev_ty,
/* FP:suggestions.rs-4423 */                         param.hir_id,
/* FP:suggestions.rs-4424 */                         param_env,
/* FP:suggestions.rs-4425 */                     );
/* FP:suggestions.rs-4426 */                     if assocs_in_this_method.iter().any(|a| a.is_some()) {
/* FP:suggestions.rs-4427 */                         assocs.push(assocs_in_this_method);
/* FP:suggestions.rs-4428 */                         print_root_expr = false;
/* FP:suggestions.rs-4429 */                     }
/* FP:suggestions.rs-4430 */                     break;
/* FP:suggestions.rs-4431 */                 }
/* FP:suggestions.rs-4432 */             }
/* FP:suggestions.rs-4433 */         }
/* FP:suggestions.rs-4434 */         // We want the type before deref coercions, otherwise we talk about `&[_]`
/* FP:suggestions.rs-4435 */         // instead of `Vec<_>`.
/* FP:suggestions.rs-4436 */         if let Some(ty) = typeck_results.expr_ty_opt(expr)
/* FP:suggestions.rs-4437 */             && print_root_expr
/* FP:suggestions.rs-4438 */         {
/* FP:suggestions.rs-4439 */             let ty = with_forced_trimmed_paths!(self.ty_to_string(ty));
/* FP:suggestions.rs-4440 */             // Point at the root expression
/* FP:suggestions.rs-4441 */             // vec![1, 2, 3].iter().map(mapper).sum<i32>()
/* FP:suggestions.rs-4442 */             // ^^^^^^^^^^^^^
/* FP:suggestions.rs-4443 */             span_labels.push((expr.span, format!("this expression has type `{ty}`")));
/* FP:suggestions.rs-4444 */         };
/* FP:suggestions.rs-4445 */         // Only show this if it is not a "trivial" expression (not a method
/* FP:suggestions.rs-4446 */         // chain) and there are associated types to talk about.
/* FP:suggestions.rs-4447 */         let mut assocs = assocs.into_iter().peekable();
/* FP:suggestions.rs-4448 */         while let Some(assocs_in_method) = assocs.next() {
/* FP:suggestions.rs-4449 */             let Some(prev_assoc_in_method) = assocs.peek() else {
/* FP:suggestions.rs-4450 */                 for entry in assocs_in_method {
/* FP:suggestions.rs-4451 */                     let Some((span, (assoc, ty))) = entry else {
/* FP:suggestions.rs-4452 */                         continue;
/* FP:suggestions.rs-4453 */                     };
/* FP:suggestions.rs-4454 */                     if primary_spans.is_empty()
/* FP:suggestions.rs-4455 */                         || type_diffs.iter().any(|diff| {
/* FP:suggestions.rs-4456 */                             let TypeError::Sorts(expected_found) = diff else {
/* FP:suggestions.rs-4457 */                                 return false;
/* FP:suggestions.rs-4458 */                             };
/* FP:suggestions.rs-4459 */                             self.can_eq(param_env, expected_found.found, ty)
/* FP:suggestions.rs-4460 */                         })
/* FP:suggestions.rs-4461 */                     {
/* FP:suggestions.rs-4462 */                         // FIXME: this doesn't quite work for `Iterator::collect`
/* FP:suggestions.rs-4463 */                         // because we have `Vec<i32>` and `()`, but we'd want `i32`
/* FP:suggestions.rs-4464 */                         // to point at the `.into_iter()` call, but as long as we
/* FP:suggestions.rs-4465 */                         // still point at the other method calls that might have
/* FP:suggestions.rs-4466 */                         // introduced the issue, this is fine for now.
/* FP:suggestions.rs-4467 */                         primary_spans.push(span);
/* FP:suggestions.rs-4468 */                     }
/* FP:suggestions.rs-4469 */                     span_labels.push((
/* FP:suggestions.rs-4470 */                         span,
/* FP:suggestions.rs-4471 */                         with_forced_trimmed_paths!(format!(
/* FP:suggestions.rs-4472 */                             "`{}` is `{ty}` here",
/* FP:suggestions.rs-4473 */                             self.tcx.def_path_str(assoc),
/* FP:suggestions.rs-4474 */                         )),
/* FP:suggestions.rs-4475 */                     ));
/* FP:suggestions.rs-4476 */                 }
/* FP:suggestions.rs-4477 */                 break;
/* FP:suggestions.rs-4478 */             };
/* FP:suggestions.rs-4479 */             for (entry, prev_entry) in
/* FP:suggestions.rs-4480 */                 assocs_in_method.into_iter().zip(prev_assoc_in_method.into_iter())
/* FP:suggestions.rs-4481 */             {
/* FP:suggestions.rs-4482 */                 match (entry, prev_entry) {
/* FP:suggestions.rs-4483 */                     (Some((span, (assoc, ty))), Some((_, (_, prev_ty)))) => {
/* FP:suggestions.rs-4484 */                         let ty_str = with_forced_trimmed_paths!(self.ty_to_string(ty));
/* FP:suggestions.rs-4485 */ 
/* FP:suggestions.rs-4486 */                         let assoc = with_forced_trimmed_paths!(self.tcx.def_path_str(assoc));
/* FP:suggestions.rs-4487 */                         if !self.can_eq(param_env, ty, *prev_ty) {
/* FP:suggestions.rs-4488 */                             if type_diffs.iter().any(|diff| {
/* FP:suggestions.rs-4489 */                                 let TypeError::Sorts(expected_found) = diff else {
/* FP:suggestions.rs-4490 */                                     return false;
/* FP:suggestions.rs-4491 */                                 };
/* FP:suggestions.rs-4492 */                                 self.can_eq(param_env, expected_found.found, ty)
/* FP:suggestions.rs-4493 */                             }) {
/* FP:suggestions.rs-4494 */                                 primary_spans.push(span);
/* FP:suggestions.rs-4495 */                             }
/* FP:suggestions.rs-4496 */                             span_labels
/* FP:suggestions.rs-4497 */                                 .push((span, format!("`{assoc}` changed to `{ty_str}` here")));
/* FP:suggestions.rs-4498 */                         } else {
/* FP:suggestions.rs-4499 */                             span_labels.push((span, format!("`{assoc}` remains `{ty_str}` here")));
/* FP:suggestions.rs-4500 */                         }
/* FP:suggestions.rs-4501 */                     }
/* FP:suggestions.rs-4502 */                     (Some((span, (assoc, ty))), None) => {
/* FP:suggestions.rs-4503 */                         span_labels.push((
/* FP:suggestions.rs-4504 */                             span,
/* FP:suggestions.rs-4505 */                             with_forced_trimmed_paths!(format!(
/* FP:suggestions.rs-4506 */                                 "`{}` is `{}` here",
/* FP:suggestions.rs-4507 */                                 self.tcx.def_path_str(assoc),
/* FP:suggestions.rs-4508 */                                 self.ty_to_string(ty),
/* FP:suggestions.rs-4509 */                             )),
/* FP:suggestions.rs-4510 */                         ));
/* FP:suggestions.rs-4511 */                     }
/* FP:suggestions.rs-4512 */                     (None, Some(_)) | (None, None) => {}
/* FP:suggestions.rs-4513 */                 }
/* FP:suggestions.rs-4514 */             }
/* FP:suggestions.rs-4515 */         }
/* FP:suggestions.rs-4516 */         if !primary_spans.is_empty() {
/* FP:suggestions.rs-4517 */             let mut multi_span: MultiSpan = primary_spans.into();
/* FP:suggestions.rs-4518 */             for (span, label) in span_labels {
/* FP:suggestions.rs-4519 */                 multi_span.push_span_label(span, label);
/* FP:suggestions.rs-4520 */             }
/* FP:suggestions.rs-4521 */             err.span_note(
/* FP:suggestions.rs-4522 */                 multi_span,
/* FP:suggestions.rs-4523 */                 "the method call chain might not have had the expected associated types",
/* FP:suggestions.rs-4524 */             );
/* FP:suggestions.rs-4525 */         }
/* FP:suggestions.rs-4526 */     }
/* FP:suggestions.rs-4527 */ 
/* FP:suggestions.rs-4528 */     fn probe_assoc_types_at_expr(
/* FP:suggestions.rs-4529 */         &self,
/* FP:suggestions.rs-4530 */         type_diffs: &[TypeError<'tcx>],
/* FP:suggestions.rs-4531 */         span: Span,
/* FP:suggestions.rs-4532 */         prev_ty: Ty<'tcx>,
/* FP:suggestions.rs-4533 */         body_id: HirId,
/* FP:suggestions.rs-4534 */         param_env: ty::ParamEnv<'tcx>,
/* FP:suggestions.rs-4535 */     ) -> Vec<Option<(Span, (DefId, Ty<'tcx>))>> {
/* FP:suggestions.rs-4536 */         let ocx = ObligationCtxt::new(self.infcx);
/* FP:suggestions.rs-4537 */         let mut assocs_in_this_method = Vec::with_capacity(type_diffs.len());
/* FP:suggestions.rs-4538 */         for diff in type_diffs {
/* FP:suggestions.rs-4539 */             let TypeError::Sorts(expected_found) = diff else {
/* FP:suggestions.rs-4540 */                 continue;
/* FP:suggestions.rs-4541 */             };
/* FP:suggestions.rs-4542 */             let ty::Alias(ty::Projection, proj) = expected_found.expected.kind() else {
/* FP:suggestions.rs-4543 */                 continue;
/* FP:suggestions.rs-4544 */             };
/* FP:suggestions.rs-4545 */ 
/* FP:suggestions.rs-4546 */             // Make `Self` be equivalent to the type of the call chain
/* FP:suggestions.rs-4547 */             // expression we're looking at now, so that we can tell what
/* FP:suggestions.rs-4548 */             // for example `Iterator::Item` is at this point in the chain.
/* FP:suggestions.rs-4549 */             let args = GenericArgs::for_item(self.tcx, proj.def_id, |param, _| {
/* FP:suggestions.rs-4550 */                 if param.index == 0 {
/* FP:suggestions.rs-4551 */                     debug_assert_matches!(param.kind, ty::GenericParamDefKind::Type { .. });
/* FP:suggestions.rs-4552 */                     return prev_ty.into();
/* FP:suggestions.rs-4553 */                 }
/* FP:suggestions.rs-4554 */                 self.var_for_def(span, param)
/* FP:suggestions.rs-4555 */             });
/* FP:suggestions.rs-4556 */             // This will hold the resolved type of the associated type, if the
/* FP:suggestions.rs-4557 */             // current expression implements the trait that associated type is
/* FP:suggestions.rs-4558 */             // in. For example, this would be what `Iterator::Item` is here.
/* FP:suggestions.rs-4559 */             let ty = self.infcx.next_ty_var(span);
/* FP:suggestions.rs-4560 */             // This corresponds to `<ExprTy as Iterator>::Item = _`.
/* FP:suggestions.rs-4561 */             let projection = ty::Binder::dummy(ty::PredicateKind::Clause(
/* FP:suggestions.rs-4562 */                 ty::ClauseKind::Projection(ty::ProjectionPredicate {
/* FP:suggestions.rs-4563 */                     projection_term: ty::AliasTerm::new_from_args(self.tcx, proj.def_id, args),
/* FP:suggestions.rs-4564 */                     term: ty.into(),
/* FP:suggestions.rs-4565 */                 }),
/* FP:suggestions.rs-4566 */             ));
/* FP:suggestions.rs-4567 */             let body_def_id = self.tcx.hir_enclosing_body_owner(body_id);
/* FP:suggestions.rs-4568 */             // Add `<ExprTy as Iterator>::Item = _` obligation.
/* FP:suggestions.rs-4569 */             ocx.register_obligation(Obligation::misc(
/* FP:suggestions.rs-4570 */                 self.tcx,
/* FP:suggestions.rs-4571 */                 span,
/* FP:suggestions.rs-4572 */                 body_def_id,
/* FP:suggestions.rs-4573 */                 param_env,
/* FP:suggestions.rs-4574 */                 projection,
/* FP:suggestions.rs-4575 */             ));
/* FP:suggestions.rs-4576 */             if ocx.select_where_possible().is_empty()
/* FP:suggestions.rs-4577 */                 && let ty = self.resolve_vars_if_possible(ty)
/* FP:suggestions.rs-4578 */                 && !ty.is_ty_var()
/* FP:suggestions.rs-4579 */             {
/* FP:suggestions.rs-4580 */                 assocs_in_this_method.push(Some((span, (proj.def_id, ty))));
/* FP:suggestions.rs-4581 */             } else {
/* FP:suggestions.rs-4582 */                 // `<ExprTy as Iterator>` didn't select, so likely we've
/* FP:suggestions.rs-4583 */                 // reached the end of the iterator chain, like the originating
/* FP:suggestions.rs-4584 */                 // `Vec<_>` or the `ty` couldn't be determined.
/* FP:suggestions.rs-4585 */                 // Keep the space consistent for later zipping.
/* FP:suggestions.rs-4586 */                 assocs_in_this_method.push(None);
/* FP:suggestions.rs-4587 */             }
/* FP:suggestions.rs-4588 */         }
/* FP:suggestions.rs-4589 */         assocs_in_this_method
/* FP:suggestions.rs-4590 */     }
/* FP:suggestions.rs-4591 */ 
/* FP:suggestions.rs-4592 */     /// If the type that failed selection is an array or a reference to an array,
/* FP:suggestions.rs-4593 */     /// but the trait is implemented for slices, suggest that the user converts
/* FP:suggestions.rs-4594 */     /// the array into a slice.
/* FP:suggestions.rs-4595 */     pub(super) fn suggest_convert_to_slice(
/* FP:suggestions.rs-4596 */         &self,
/* FP:suggestions.rs-4597 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-4598 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-4599 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-4600 */         candidate_impls: &[ImplCandidate<'tcx>],
/* FP:suggestions.rs-4601 */         span: Span,
/* FP:suggestions.rs-4602 */     ) {
/* FP:suggestions.rs-4603 */         // We can only suggest the slice coercion for function and binary operation arguments,
/* FP:suggestions.rs-4604 */         // since the suggestion would make no sense in turbofish or call
/* FP:suggestions.rs-4605 */         let (ObligationCauseCode::BinOp { .. } | ObligationCauseCode::FunctionArg { .. }) =
/* FP:suggestions.rs-4606 */             obligation.cause.code()
/* FP:suggestions.rs-4607 */         else {
/* FP:suggestions.rs-4608 */             return;
/* FP:suggestions.rs-4609 */         };
/* FP:suggestions.rs-4610 */ 
/* FP:suggestions.rs-4611 */         // Three cases where we can make a suggestion:
/* FP:suggestions.rs-4612 */         // 1. `[T; _]` (array of T)
/* FP:suggestions.rs-4613 */         // 2. `&[T; _]` (reference to array of T)
/* FP:suggestions.rs-4614 */         // 3. `&mut [T; _]` (mutable reference to array of T)
/* FP:suggestions.rs-4615 */         let (element_ty, mut mutability) = match *trait_pred.skip_binder().self_ty().kind() {
/* FP:suggestions.rs-4616 */             ty::Array(element_ty, _) => (element_ty, None),
/* FP:suggestions.rs-4617 */ 
/* FP:suggestions.rs-4618 */             ty::Ref(_, pointee_ty, mutability) => match *pointee_ty.kind() {
/* FP:suggestions.rs-4619 */                 ty::Array(element_ty, _) => (element_ty, Some(mutability)),
/* FP:suggestions.rs-4620 */                 _ => return,
/* FP:suggestions.rs-4621 */             },
/* FP:suggestions.rs-4622 */ 
/* FP:suggestions.rs-4623 */             _ => return,
/* FP:suggestions.rs-4624 */         };
/* FP:suggestions.rs-4625 */ 
/* FP:suggestions.rs-4626 */         // Go through all the candidate impls to see if any of them is for
/* FP:suggestions.rs-4627 */         // slices of `element_ty` with `mutability`.
/* FP:suggestions.rs-4628 */         let mut is_slice = |candidate: Ty<'tcx>| match *candidate.kind() {
/* FP:suggestions.rs-4629 */             ty::RawPtr(t, m) | ty::Ref(_, t, m) => {
/* FP:suggestions.rs-4630 */                 if matches!(*t.kind(), ty::Slice(e) if e == element_ty)
/* FP:suggestions.rs-4631 */                     && m == mutability.unwrap_or(m)
/* FP:suggestions.rs-4632 */                 {
/* FP:suggestions.rs-4633 */                     // Use the candidate's mutability going forward.
/* FP:suggestions.rs-4634 */                     mutability = Some(m);
/* FP:suggestions.rs-4635 */                     true
/* FP:suggestions.rs-4636 */                 } else {
/* FP:suggestions.rs-4637 */                     false
/* FP:suggestions.rs-4638 */                 }
/* FP:suggestions.rs-4639 */             }
/* FP:suggestions.rs-4640 */             _ => false,
/* FP:suggestions.rs-4641 */         };
/* FP:suggestions.rs-4642 */ 
/* FP:suggestions.rs-4643 */         // Grab the first candidate that matches, if any, and make a suggestion.
/* FP:suggestions.rs-4644 */         if let Some(slice_ty) = candidate_impls
/* FP:suggestions.rs-4645 */             .iter()
/* FP:suggestions.rs-4646 */             .map(|trait_ref| trait_ref.trait_ref.self_ty())
/* FP:suggestions.rs-4647 */             .find(|t| is_slice(*t))
/* FP:suggestions.rs-4648 */         {
/* FP:suggestions.rs-4649 */             let msg = format!("convert the array to a `{slice_ty}` slice instead");
/* FP:suggestions.rs-4650 */ 
/* FP:suggestions.rs-4651 */             if let Ok(snippet) = self.tcx.sess.source_map().span_to_snippet(span) {
/* FP:suggestions.rs-4652 */                 let mut suggestions = vec![];
/* FP:suggestions.rs-4653 */                 if snippet.starts_with('&') {
/* FP:suggestions.rs-4654 */                 } else if let Some(hir::Mutability::Mut) = mutability {
/* FP:suggestions.rs-4655 */                     suggestions.push((span.shrink_to_lo(), "&mut ".into()));
/* FP:suggestions.rs-4656 */                 } else {
/* FP:suggestions.rs-4657 */                     suggestions.push((span.shrink_to_lo(), "&".into()));
/* FP:suggestions.rs-4658 */                 }
/* FP:suggestions.rs-4659 */                 suggestions.push((span.shrink_to_hi(), "[..]".into()));
/* FP:suggestions.rs-4660 */                 err.multipart_suggestion_verbose(msg, suggestions, Applicability::MaybeIncorrect);
/* FP:suggestions.rs-4661 */             } else {
/* FP:suggestions.rs-4662 */                 err.span_help(span, msg);
/* FP:suggestions.rs-4663 */             }
/* FP:suggestions.rs-4664 */         }
/* FP:suggestions.rs-4665 */     }
/* FP:suggestions.rs-4666 */ 
/* FP:suggestions.rs-4667 */     /// If the type failed selection but the trait is implemented for `(T,)`, suggest that the user
/* FP:suggestions.rs-4668 */     /// creates a unary tuple
/* FP:suggestions.rs-4669 */     ///
/* FP:suggestions.rs-4670 */     /// This is a common gotcha when using libraries that emulate variadic functions with traits for tuples.
/* FP:suggestions.rs-4671 */     pub(super) fn suggest_tuple_wrapping(
/* FP:suggestions.rs-4672 */         &self,
/* FP:suggestions.rs-4673 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-4674 */         root_obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-4675 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-4676 */     ) {
/* FP:suggestions.rs-4677 */         let ObligationCauseCode::FunctionArg { arg_hir_id, .. } = obligation.cause.code() else {
/* FP:suggestions.rs-4678 */             return;
/* FP:suggestions.rs-4679 */         };
/* FP:suggestions.rs-4680 */ 
/* FP:suggestions.rs-4681 */         let Some(root_pred) = root_obligation.predicate.as_trait_clause() else { return };
/* FP:suggestions.rs-4682 */ 
/* FP:suggestions.rs-4683 */         let trait_ref = root_pred.map_bound(|root_pred| {
/* FP:suggestions.rs-4684 */             root_pred.trait_ref.with_replaced_self_ty(
/* FP:suggestions.rs-4685 */                 self.tcx,
/* FP:suggestions.rs-4686 */                 Ty::new_tup(self.tcx, &[root_pred.trait_ref.self_ty()]),
/* FP:suggestions.rs-4687 */             )
/* FP:suggestions.rs-4688 */         });
/* FP:suggestions.rs-4689 */ 
/* FP:suggestions.rs-4690 */         let obligation =
/* FP:suggestions.rs-4691 */             Obligation::new(self.tcx, obligation.cause.clone(), obligation.param_env, trait_ref);
/* FP:suggestions.rs-4692 */ 
/* FP:suggestions.rs-4693 */         if self.predicate_must_hold_modulo_regions(&obligation) {
/* FP:suggestions.rs-4694 */             let arg_span = self.tcx.hir_span(*arg_hir_id);
/* FP:suggestions.rs-4695 */             err.multipart_suggestion_verbose(
/* FP:suggestions.rs-4696 */                 format!("use a unary tuple instead"),
/* FP:suggestions.rs-4697 */                 vec![(arg_span.shrink_to_lo(), "(".into()), (arg_span.shrink_to_hi(), ",)".into())],
/* FP:suggestions.rs-4698 */                 Applicability::MaybeIncorrect,
/* FP:suggestions.rs-4699 */             );
/* FP:suggestions.rs-4700 */         }
/* FP:suggestions.rs-4701 */     }
/* FP:suggestions.rs-4702 */ 
/* FP:suggestions.rs-4703 */     pub(super) fn explain_hrtb_projection(
/* FP:suggestions.rs-4704 */         &self,
/* FP:suggestions.rs-4705 */         diag: &mut Diag<'_>,
/* FP:suggestions.rs-4706 */         pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-4707 */         param_env: ty::ParamEnv<'tcx>,
/* FP:suggestions.rs-4708 */         cause: &ObligationCause<'tcx>,
/* FP:suggestions.rs-4709 */     ) {
/* FP:suggestions.rs-4710 */         if pred.skip_binder().has_escaping_bound_vars() && pred.skip_binder().has_non_region_infer()
/* FP:suggestions.rs-4711 */         {
/* FP:suggestions.rs-4712 */             self.probe(|_| {
/* FP:suggestions.rs-4713 */                 let ocx = ObligationCtxt::new(self);
/* FP:suggestions.rs-4714 */                 self.enter_forall(pred, |pred| {
/* FP:suggestions.rs-4715 */                     let pred = ocx.normalize(&ObligationCause::dummy(), param_env, pred);
/* FP:suggestions.rs-4716 */                     ocx.register_obligation(Obligation::new(
/* FP:suggestions.rs-4717 */                         self.tcx,
/* FP:suggestions.rs-4718 */                         ObligationCause::dummy(),
/* FP:suggestions.rs-4719 */                         param_env,
/* FP:suggestions.rs-4720 */                         pred,
/* FP:suggestions.rs-4721 */                     ));
/* FP:suggestions.rs-4722 */                 });
/* FP:suggestions.rs-4723 */                 if !ocx.select_where_possible().is_empty() {
/* FP:suggestions.rs-4724 */                     // encountered errors.
/* FP:suggestions.rs-4725 */                     return;
/* FP:suggestions.rs-4726 */                 }
/* FP:suggestions.rs-4727 */ 
/* FP:suggestions.rs-4728 */                 if let ObligationCauseCode::FunctionArg {
/* FP:suggestions.rs-4729 */                     call_hir_id,
/* FP:suggestions.rs-4730 */                     arg_hir_id,
/* FP:suggestions.rs-4731 */                     parent_code: _,
/* FP:suggestions.rs-4732 */                 } = cause.code()
/* FP:suggestions.rs-4733 */                 {
/* FP:suggestions.rs-4734 */                     let arg_span = self.tcx.hir_span(*arg_hir_id);
/* FP:suggestions.rs-4735 */                     let mut sp: MultiSpan = arg_span.into();
/* FP:suggestions.rs-4736 */ 
/* FP:suggestions.rs-4737 */                     sp.push_span_label(
/* FP:suggestions.rs-4738 */                         arg_span,
/* FP:suggestions.rs-4739 */                         "the trait solver is unable to infer the \
/* FP:suggestions.rs-4740 */                         generic types that should be inferred from this argument",
/* FP:suggestions.rs-4741 */                     );
/* FP:suggestions.rs-4742 */                     sp.push_span_label(
/* FP:suggestions.rs-4743 */                         self.tcx.hir_span(*call_hir_id),
/* FP:suggestions.rs-4744 */                         "add turbofish arguments to this call to \
/* FP:suggestions.rs-4745 */                         specify the types manually, even if it's redundant",
/* FP:suggestions.rs-4746 */                     );
/* FP:suggestions.rs-4747 */                     diag.span_note(
/* FP:suggestions.rs-4748 */                         sp,
/* FP:suggestions.rs-4749 */                         "this is a known limitation of the trait solver that \
/* FP:suggestions.rs-4750 */                         will be lifted in the future",
/* FP:suggestions.rs-4751 */                     );
/* FP:suggestions.rs-4752 */                 } else {
/* FP:suggestions.rs-4753 */                     let mut sp: MultiSpan = cause.span.into();
/* FP:suggestions.rs-4754 */                     sp.push_span_label(
/* FP:suggestions.rs-4755 */                         cause.span,
/* FP:suggestions.rs-4756 */                         "try adding turbofish arguments to this expression to \
/* FP:suggestions.rs-4757 */                         specify the types manually, even if it's redundant",
/* FP:suggestions.rs-4758 */                     );
/* FP:suggestions.rs-4759 */                     diag.span_note(
/* FP:suggestions.rs-4760 */                         sp,
/* FP:suggestions.rs-4761 */                         "this is a known limitation of the trait solver that \
/* FP:suggestions.rs-4762 */                         will be lifted in the future",
/* FP:suggestions.rs-4763 */                     );
/* FP:suggestions.rs-4764 */                 }
/* FP:suggestions.rs-4765 */             });
/* FP:suggestions.rs-4766 */         }
/* FP:suggestions.rs-4767 */     }
/* FP:suggestions.rs-4768 */ 
/* FP:suggestions.rs-4769 */     pub(super) fn suggest_desugaring_async_fn_in_trait(
/* FP:suggestions.rs-4770 */         &self,
/* FP:suggestions.rs-4771 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-4772 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-4773 */     ) {
/* FP:suggestions.rs-4774 */         // Don't suggest if RTN is active -- we should prefer a where-clause bound instead.
/* FP:suggestions.rs-4775 */         if self.tcx.features().return_type_notation() {
/* FP:suggestions.rs-4776 */             return;
/* FP:suggestions.rs-4777 */         }
/* FP:suggestions.rs-4778 */ 
/* FP:suggestions.rs-4779 */         let trait_def_id = trait_pred.def_id();
/* FP:suggestions.rs-4780 */ 
/* FP:suggestions.rs-4781 */         // Only suggest specifying auto traits
/* FP:suggestions.rs-4782 */         if !self.tcx.trait_is_auto(trait_def_id) {
/* FP:suggestions.rs-4783 */             return;
/* FP:suggestions.rs-4784 */         }
/* FP:suggestions.rs-4785 */ 
/* FP:suggestions.rs-4786 */         // Look for an RPITIT
/* FP:suggestions.rs-4787 */         let ty::Alias(ty::Projection, alias_ty) = trait_pred.self_ty().skip_binder().kind() else {
/* FP:suggestions.rs-4788 */             return;
/* FP:suggestions.rs-4789 */         };
/* FP:suggestions.rs-4790 */         let Some(ty::ImplTraitInTraitData::Trait { fn_def_id, opaque_def_id }) =
/* FP:suggestions.rs-4791 */             self.tcx.opt_rpitit_info(alias_ty.def_id)
/* FP:suggestions.rs-4792 */         else {
/* FP:suggestions.rs-4793 */             return;
/* FP:suggestions.rs-4794 */         };
/* FP:suggestions.rs-4795 */ 
/* FP:suggestions.rs-4796 */         let auto_trait = self.tcx.def_path_str(trait_def_id);
/* FP:suggestions.rs-4797 */         // ... which is a local function
/* FP:suggestions.rs-4798 */         let Some(fn_def_id) = fn_def_id.as_local() else {
/* FP:suggestions.rs-4799 */             // If it's not local, we can at least mention that the method is async, if it is.
/* FP:suggestions.rs-4800 */             if self.tcx.asyncness(fn_def_id).is_async() {
/* FP:suggestions.rs-4801 */                 err.span_note(
/* FP:suggestions.rs-4802 */                     self.tcx.def_span(fn_def_id),
/* FP:suggestions.rs-4803 */                     format!(
/* FP:suggestions.rs-4804 */                         "`{}::{}` is an `async fn` in trait, which does not \
/* FP:suggestions.rs-4805 */                     automatically imply that its future is `{auto_trait}`",
/* FP:suggestions.rs-4806 */                         alias_ty.trait_ref(self.tcx),
/* FP:suggestions.rs-4807 */                         self.tcx.item_name(fn_def_id)
/* FP:suggestions.rs-4808 */                     ),
/* FP:suggestions.rs-4809 */                 );
/* FP:suggestions.rs-4810 */             }
/* FP:suggestions.rs-4811 */             return;
/* FP:suggestions.rs-4812 */         };
/* FP:suggestions.rs-4813 */         let hir::Node::TraitItem(item) = self.tcx.hir_node_by_def_id(fn_def_id) else {
/* FP:suggestions.rs-4814 */             return;
/* FP:suggestions.rs-4815 */         };
/* FP:suggestions.rs-4816 */ 
/* FP:suggestions.rs-4817 */         // ... whose signature is `async` (i.e. this is an AFIT)
/* FP:suggestions.rs-4818 */         let (sig, body) = item.expect_fn();
/* FP:suggestions.rs-4819 */         let hir::FnRetTy::Return(hir::Ty { kind: hir::TyKind::OpaqueDef(opaq_def, ..), .. }) =
/* FP:suggestions.rs-4820 */             sig.decl.output
/* FP:suggestions.rs-4821 */         else {
/* FP:suggestions.rs-4822 */             // This should never happen, but let's not ICE.
/* FP:suggestions.rs-4823 */             return;
/* FP:suggestions.rs-4824 */         };
/* FP:suggestions.rs-4825 */ 
/* FP:suggestions.rs-4826 */         // Check that this is *not* a nested `impl Future` RPIT in an async fn
/* FP:suggestions.rs-4827 */         // (i.e. `async fn foo() -> impl Future`)
/* FP:suggestions.rs-4828 */         if opaq_def.def_id.to_def_id() != opaque_def_id {
/* FP:suggestions.rs-4829 */             return;
/* FP:suggestions.rs-4830 */         }
/* FP:suggestions.rs-4831 */ 
/* FP:suggestions.rs-4832 */         let Some(sugg) = suggest_desugaring_async_fn_to_impl_future_in_trait(
/* FP:suggestions.rs-4833 */             self.tcx,
/* FP:suggestions.rs-4834 */             *sig,
/* FP:suggestions.rs-4835 */             *body,
/* FP:suggestions.rs-4836 */             opaque_def_id.expect_local(),
/* FP:suggestions.rs-4837 */             &format!(" + {auto_trait}"),
/* FP:suggestions.rs-4838 */         ) else {
/* FP:suggestions.rs-4839 */             return;
/* FP:suggestions.rs-4840 */         };
/* FP:suggestions.rs-4841 */ 
/* FP:suggestions.rs-4842 */         let function_name = self.tcx.def_path_str(fn_def_id);
/* FP:suggestions.rs-4843 */         err.multipart_suggestion(
/* FP:suggestions.rs-4844 */             format!(
/* FP:suggestions.rs-4845 */                 "`{auto_trait}` can be made part of the associated future's \
/* FP:suggestions.rs-4846 */                 guarantees for all implementations of `{function_name}`"
/* FP:suggestions.rs-4847 */             ),
/* FP:suggestions.rs-4848 */             sugg,
/* FP:suggestions.rs-4849 */             Applicability::MachineApplicable,
/* FP:suggestions.rs-4850 */         );
/* FP:suggestions.rs-4851 */     }
/* FP:suggestions.rs-4852 */ 
/* FP:suggestions.rs-4853 */     pub fn ty_kind_suggestion(
/* FP:suggestions.rs-4854 */         &self,
/* FP:suggestions.rs-4855 */         param_env: ty::ParamEnv<'tcx>,
/* FP:suggestions.rs-4856 */         ty: Ty<'tcx>,
/* FP:suggestions.rs-4857 */     ) -> Option<String> {
/* FP:suggestions.rs-4858 */         let tcx = self.infcx.tcx;
/* FP:suggestions.rs-4859 */         let implements_default = |ty| {
/* FP:suggestions.rs-4860 */             let Some(default_trait) = tcx.get_diagnostic_item(sym::Default) else {
/* FP:suggestions.rs-4861 */                 return false;
/* FP:suggestions.rs-4862 */             };
/* FP:suggestions.rs-4863 */             self.type_implements_trait(default_trait, [ty], param_env).must_apply_modulo_regions()
/* FP:suggestions.rs-4864 */         };
/* FP:suggestions.rs-4865 */ 
/* FP:suggestions.rs-4866 */         Some(match *ty.kind() {
/* FP:suggestions.rs-4867 */             ty::Never | ty::Error(_) => return None,
/* FP:suggestions.rs-4868 */             ty::Bool => "false".to_string(),
/* FP:suggestions.rs-4869 */             ty::Char => "\'x\'".to_string(),
/* FP:suggestions.rs-4870 */             ty::Int(_) | ty::Uint(_) => "42".into(),
/* FP:suggestions.rs-4871 */             ty::Float(_) => "3.14159".into(),
/* FP:suggestions.rs-4872 */             ty::Slice(_) => "[]".to_string(),
/* FP:suggestions.rs-4873 */             ty::Adt(def, _) if Some(def.did()) == tcx.get_diagnostic_item(sym::Vec) => {
/* FP:suggestions.rs-4874 */                 "vec![]".to_string()
/* FP:suggestions.rs-4875 */             }
/* FP:suggestions.rs-4876 */             ty::Adt(def, _) if Some(def.did()) == tcx.get_diagnostic_item(sym::String) => {
/* FP:suggestions.rs-4877 */                 "String::new()".to_string()
/* FP:suggestions.rs-4878 */             }
/* FP:suggestions.rs-4879 */             ty::Adt(def, args) if def.is_box() => {
/* FP:suggestions.rs-4880 */                 format!("Box::new({})", self.ty_kind_suggestion(param_env, args[0].expect_ty())?)
/* FP:suggestions.rs-4881 */             }
/* FP:suggestions.rs-4882 */             ty::Adt(def, _) if Some(def.did()) == tcx.get_diagnostic_item(sym::Option) => {
/* FP:suggestions.rs-4883 */                 "None".to_string()
/* FP:suggestions.rs-4884 */             }
/* FP:suggestions.rs-4885 */             ty::Adt(def, args) if Some(def.did()) == tcx.get_diagnostic_item(sym::Result) => {
/* FP:suggestions.rs-4886 */                 format!("Ok({})", self.ty_kind_suggestion(param_env, args[0].expect_ty())?)
/* FP:suggestions.rs-4887 */             }
/* FP:suggestions.rs-4888 */             ty::Adt(_, _) if implements_default(ty) => "Default::default()".to_string(),
/* FP:suggestions.rs-4889 */             ty::Ref(_, ty, mutability) => {
/* FP:suggestions.rs-4890 */                 if let (ty::Str, hir::Mutability::Not) = (ty.kind(), mutability) {
/* FP:suggestions.rs-4891 */                     "\"\"".to_string()
/* FP:suggestions.rs-4892 */                 } else {
/* FP:suggestions.rs-4893 */                     let ty = self.ty_kind_suggestion(param_env, ty)?;
/* FP:suggestions.rs-4894 */                     format!("&{}{ty}", mutability.prefix_str())
/* FP:suggestions.rs-4895 */                 }
/* FP:suggestions.rs-4896 */             }
/* FP:suggestions.rs-4897 */             ty::Array(ty, len) if let Some(len) = len.try_to_target_usize(tcx) => {
/* FP:suggestions.rs-4898 */                 if len == 0 {
/* FP:suggestions.rs-4899 */                     "[]".to_string()
/* FP:suggestions.rs-4900 */                 } else if self.type_is_copy_modulo_regions(param_env, ty) || len == 1 {
/* FP:suggestions.rs-4901 */                     // Can only suggest `[ty; 0]` if sz == 1 or copy
/* FP:suggestions.rs-4902 */                     format!("[{}; {}]", self.ty_kind_suggestion(param_env, ty)?, len)
/* FP:suggestions.rs-4903 */                 } else {
/* FP:suggestions.rs-4904 */                     "/* value */".to_string()
/* FP:suggestions.rs-4905 */                 }
/* FP:suggestions.rs-4906 */             }
/* FP:suggestions.rs-4907 */             ty::Tuple(tys) => format!(
/* FP:suggestions.rs-4908 */                 "({}{})",
/* FP:suggestions.rs-4909 */                 tys.iter()
/* FP:suggestions.rs-4910 */                     .map(|ty| self.ty_kind_suggestion(param_env, ty))
/* FP:suggestions.rs-4911 */                     .collect::<Option<Vec<String>>>()?
/* FP:suggestions.rs-4912 */                     .join(", "),
/* FP:suggestions.rs-4913 */                 if tys.len() == 1 { "," } else { "" }
/* FP:suggestions.rs-4914 */             ),
/* FP:suggestions.rs-4915 */             _ => "/* value */".to_string(),
/* FP:suggestions.rs-4916 */         })
/* FP:suggestions.rs-4917 */     }
/* FP:suggestions.rs-4918 */ 
/* FP:suggestions.rs-4919 */     // For E0277 when use `?` operator, suggest adding
/* FP:suggestions.rs-4920 */     // a suitable return type in `FnSig`, and a default
/* FP:suggestions.rs-4921 */     // return value at the end of the function's body.
/* FP:suggestions.rs-4922 */     pub(super) fn suggest_add_result_as_return_type(
/* FP:suggestions.rs-4923 */         &self,
/* FP:suggestions.rs-4924 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-4925 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-4926 */         trait_pred: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-4927 */     ) {
/* FP:suggestions.rs-4928 */         if ObligationCauseCode::QuestionMark != *obligation.cause.code().peel_derives() {
/* FP:suggestions.rs-4929 */             return;
/* FP:suggestions.rs-4930 */         }
/* FP:suggestions.rs-4931 */ 
/* FP:suggestions.rs-4932 */         // Only suggest for local function and associated method,
/* FP:suggestions.rs-4933 */         // because this suggest adding both return type in
/* FP:suggestions.rs-4934 */         // the `FnSig` and a default return value in the body, so it
/* FP:suggestions.rs-4935 */         // is not suitable for foreign function without a local body,
/* FP:suggestions.rs-4936 */         // and neither for trait method which may be also implemented
/* FP:suggestions.rs-4937 */         // in other place, so shouldn't change it's FnSig.
/* FP:suggestions.rs-4938 */         fn choose_suggest_items<'tcx, 'hir>(
/* FP:suggestions.rs-4939 */             tcx: TyCtxt<'tcx>,
/* FP:suggestions.rs-4940 */             node: hir::Node<'hir>,
/* FP:suggestions.rs-4941 */         ) -> Option<(&'hir hir::FnDecl<'hir>, hir::BodyId)> {
/* FP:suggestions.rs-4942 */             match node {
/* FP:suggestions.rs-4943 */                 hir::Node::Item(item)
/* FP:suggestions.rs-4944 */                     if let hir::ItemKind::Fn { sig, body: body_id, .. } = item.kind =>
/* FP:suggestions.rs-4945 */                 {
/* FP:suggestions.rs-4946 */                     Some((sig.decl, body_id))
/* FP:suggestions.rs-4947 */                 }
/* FP:suggestions.rs-4948 */                 hir::Node::ImplItem(item)
/* FP:suggestions.rs-4949 */                     if let hir::ImplItemKind::Fn(sig, body_id) = item.kind =>
/* FP:suggestions.rs-4950 */                 {
/* FP:suggestions.rs-4951 */                     let parent = tcx.parent_hir_node(item.hir_id());
/* FP:suggestions.rs-4952 */                     if let hir::Node::Item(item) = parent
/* FP:suggestions.rs-4953 */                         && let hir::ItemKind::Impl(imp) = item.kind
/* FP:suggestions.rs-4954 */                         && imp.of_trait.is_none()
/* FP:suggestions.rs-4955 */                     {
/* FP:suggestions.rs-4956 */                         return Some((sig.decl, body_id));
/* FP:suggestions.rs-4957 */                     }
/* FP:suggestions.rs-4958 */                     None
/* FP:suggestions.rs-4959 */                 }
/* FP:suggestions.rs-4960 */                 _ => None,
/* FP:suggestions.rs-4961 */             }
/* FP:suggestions.rs-4962 */         }
/* FP:suggestions.rs-4963 */ 
/* FP:suggestions.rs-4964 */         let node = self.tcx.hir_node_by_def_id(obligation.cause.body_id);
/* FP:suggestions.rs-4965 */         if let Some((fn_decl, body_id)) = choose_suggest_items(self.tcx, node)
/* FP:suggestions.rs-4966 */             && let hir::FnRetTy::DefaultReturn(ret_span) = fn_decl.output
/* FP:suggestions.rs-4967 */             && self.tcx.is_diagnostic_item(sym::FromResidual, trait_pred.def_id())
/* FP:suggestions.rs-4968 */             && trait_pred.skip_binder().trait_ref.args.type_at(0).is_unit()
/* FP:suggestions.rs-4969 */             && let ty::Adt(def, _) = trait_pred.skip_binder().trait_ref.args.type_at(1).kind()
/* FP:suggestions.rs-4970 */             && self.tcx.is_diagnostic_item(sym::Result, def.did())
/* FP:suggestions.rs-4971 */         {
/* FP:suggestions.rs-4972 */             let mut sugg_spans =
/* FP:suggestions.rs-4973 */                 vec![(ret_span, " -> Result<(), Box<dyn std::error::Error>>".to_string())];
/* FP:suggestions.rs-4974 */             let body = self.tcx.hir_body(body_id);
/* FP:suggestions.rs-4975 */             if let hir::ExprKind::Block(b, _) = body.value.kind
/* FP:suggestions.rs-4976 */                 && b.expr.is_none()
/* FP:suggestions.rs-4977 */             {
/* FP:suggestions.rs-4978 */                 // The span of '}' in the end of block.
/* FP:suggestions.rs-4979 */                 let span = self.tcx.sess.source_map().end_point(b.span);
/* FP:suggestions.rs-4980 */                 sugg_spans.push((
/* FP:suggestions.rs-4981 */                     span.shrink_to_lo(),
/* FP:suggestions.rs-4982 */                     format!(
/* FP:suggestions.rs-4983 */                         "{}{}",
/* FP:suggestions.rs-4984 */                         "    Ok(())\n",
/* FP:suggestions.rs-4985 */                         self.tcx.sess.source_map().indentation_before(span).unwrap_or_default(),
/* FP:suggestions.rs-4986 */                     ),
/* FP:suggestions.rs-4987 */                 ));
/* FP:suggestions.rs-4988 */             }
/* FP:suggestions.rs-4989 */             err.multipart_suggestion_verbose(
/* FP:suggestions.rs-4990 */                 format!("consider adding return type"),
/* FP:suggestions.rs-4991 */                 sugg_spans,
/* FP:suggestions.rs-4992 */                 Applicability::MaybeIncorrect,
/* FP:suggestions.rs-4993 */             );
/* FP:suggestions.rs-4994 */         }
/* FP:suggestions.rs-4995 */     }
/* FP:suggestions.rs-4996 */ 
/* FP:suggestions.rs-4997 */     #[instrument(level = "debug", skip_all)]
/* FP:suggestions.rs-4998 */     pub(super) fn suggest_unsized_bound_if_applicable(
/* FP:suggestions.rs-4999 */         &self,
/* FP:suggestions.rs-5000 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-5001 */         obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-5002 */     ) {
/* FP:suggestions.rs-5003 */         let ty::PredicateKind::Clause(ty::ClauseKind::Trait(pred)) =
/* FP:suggestions.rs-5004 */             obligation.predicate.kind().skip_binder()
/* FP:suggestions.rs-5005 */         else {
/* FP:suggestions.rs-5006 */             return;
/* FP:suggestions.rs-5007 */         };
/* FP:suggestions.rs-5008 */         let (ObligationCauseCode::WhereClause(item_def_id, span)
/* FP:suggestions.rs-5009 */         | ObligationCauseCode::WhereClauseInExpr(item_def_id, span, ..)) =
/* FP:suggestions.rs-5010 */             *obligation.cause.code().peel_derives()
/* FP:suggestions.rs-5011 */         else {
/* FP:suggestions.rs-5012 */             return;
/* FP:suggestions.rs-5013 */         };
/* FP:suggestions.rs-5014 */         if span.is_dummy() {
/* FP:suggestions.rs-5015 */             return;
/* FP:suggestions.rs-5016 */         }
/* FP:suggestions.rs-5017 */         debug!(?pred, ?item_def_id, ?span);
/* FP:suggestions.rs-5018 */ 
/* FP:suggestions.rs-5019 */         let (Some(node), true) = (
/* FP:suggestions.rs-5020 */             self.tcx.hir_get_if_local(item_def_id),
/* FP:suggestions.rs-5021 */             self.tcx.is_lang_item(pred.def_id(), LangItem::Sized),
/* FP:suggestions.rs-5022 */         ) else {
/* FP:suggestions.rs-5023 */             return;
/* FP:suggestions.rs-5024 */         };
/* FP:suggestions.rs-5025 */ 
/* FP:suggestions.rs-5026 */         let Some(generics) = node.generics() else {
/* FP:suggestions.rs-5027 */             return;
/* FP:suggestions.rs-5028 */         };
/* FP:suggestions.rs-5029 */         let sized_trait = self.tcx.lang_items().sized_trait();
/* FP:suggestions.rs-5030 */         debug!(?generics.params);
/* FP:suggestions.rs-5031 */         debug!(?generics.predicates);
/* FP:suggestions.rs-5032 */         let Some(param) = generics.params.iter().find(|param| param.span == span) else {
/* FP:suggestions.rs-5033 */             return;
/* FP:suggestions.rs-5034 */         };
/* FP:suggestions.rs-5035 */         // Check that none of the explicit trait bounds is `Sized`. Assume that an explicit
/* FP:suggestions.rs-5036 */         // `Sized` bound is there intentionally and we don't need to suggest relaxing it.
/* FP:suggestions.rs-5037 */         let explicitly_sized = generics
/* FP:suggestions.rs-5038 */             .bounds_for_param(param.def_id)
/* FP:suggestions.rs-5039 */             .flat_map(|bp| bp.bounds)
/* FP:suggestions.rs-5040 */             .any(|bound| bound.trait_ref().and_then(|tr| tr.trait_def_id()) == sized_trait);
/* FP:suggestions.rs-5041 */         if explicitly_sized {
/* FP:suggestions.rs-5042 */             return;
/* FP:suggestions.rs-5043 */         }
/* FP:suggestions.rs-5044 */         debug!(?param);
/* FP:suggestions.rs-5045 */         match node {
/* FP:suggestions.rs-5046 */             hir::Node::Item(
/* FP:suggestions.rs-5047 */                 item @ hir::Item {
/* FP:suggestions.rs-5048 */                     // Only suggest indirection for uses of type parameters in ADTs.
/* FP:suggestions.rs-5049 */                     kind:
/* FP:suggestions.rs-5050 */                         hir::ItemKind::Enum(..) | hir::ItemKind::Struct(..) | hir::ItemKind::Union(..),
/* FP:suggestions.rs-5051 */                     ..
/* FP:suggestions.rs-5052 */                 },
/* FP:suggestions.rs-5053 */             ) => {
/* FP:suggestions.rs-5054 */                 if self.suggest_indirection_for_unsized(err, item, param) {
/* FP:suggestions.rs-5055 */                     return;
/* FP:suggestions.rs-5056 */                 }
/* FP:suggestions.rs-5057 */             }
/* FP:suggestions.rs-5058 */             _ => {}
/* FP:suggestions.rs-5059 */         };
/* FP:suggestions.rs-5060 */ 
/* FP:suggestions.rs-5061 */         // Didn't add an indirection suggestion, so add a general suggestion to relax `Sized`.
/* FP:suggestions.rs-5062 */         let (span, separator, open_paren_sp) =
/* FP:suggestions.rs-5063 */             if let Some((s, open_paren_sp)) = generics.bounds_span_for_suggestions(param.def_id) {
/* FP:suggestions.rs-5064 */                 (s, " +", open_paren_sp)
/* FP:suggestions.rs-5065 */             } else {
/* FP:suggestions.rs-5066 */                 (param.name.ident().span.shrink_to_hi(), ":", None)
/* FP:suggestions.rs-5067 */             };
/* FP:suggestions.rs-5068 */ 
/* FP:suggestions.rs-5069 */         let mut suggs = vec![];
/* FP:suggestions.rs-5070 */         let suggestion = format!("{separator} ?Sized");
/* FP:suggestions.rs-5071 */ 
/* FP:suggestions.rs-5072 */         if let Some(open_paren_sp) = open_paren_sp {
/* FP:suggestions.rs-5073 */             suggs.push((open_paren_sp, "(".to_string()));
/* FP:suggestions.rs-5074 */             suggs.push((span, format!("){suggestion}")));
/* FP:suggestions.rs-5075 */         } else {
/* FP:suggestions.rs-5076 */             suggs.push((span, suggestion));
/* FP:suggestions.rs-5077 */         }
/* FP:suggestions.rs-5078 */ 
/* FP:suggestions.rs-5079 */         err.multipart_suggestion_verbose(
/* FP:suggestions.rs-5080 */             "consider relaxing the implicit `Sized` restriction",
/* FP:suggestions.rs-5081 */             suggs,
/* FP:suggestions.rs-5082 */             Applicability::MachineApplicable,
/* FP:suggestions.rs-5083 */         );
/* FP:suggestions.rs-5084 */     }
/* FP:suggestions.rs-5085 */ 
/* FP:suggestions.rs-5086 */     fn suggest_indirection_for_unsized(
/* FP:suggestions.rs-5087 */         &self,
/* FP:suggestions.rs-5088 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-5089 */         item: &hir::Item<'tcx>,
/* FP:suggestions.rs-5090 */         param: &hir::GenericParam<'tcx>,
/* FP:suggestions.rs-5091 */     ) -> bool {
/* FP:suggestions.rs-5092 */         // Suggesting `T: ?Sized` is only valid in an ADT if `T` is only used in a
/* FP:suggestions.rs-5093 */         // borrow. `struct S<'a, T: ?Sized>(&'a T);` is valid, `struct S<T: ?Sized>(T);`
/* FP:suggestions.rs-5094 */         // is not. Look for invalid "bare" parameter uses, and suggest using indirection.
/* FP:suggestions.rs-5095 */         let mut visitor = FindTypeParam { param: param.name.ident().name, .. };
/* FP:suggestions.rs-5096 */         visitor.visit_item(item);
/* FP:suggestions.rs-5097 */         if visitor.invalid_spans.is_empty() {
/* FP:suggestions.rs-5098 */             return false;
/* FP:suggestions.rs-5099 */         }
/* FP:suggestions.rs-5100 */         let mut multispan: MultiSpan = param.span.into();
/* FP:suggestions.rs-5101 */         multispan.push_span_label(
/* FP:suggestions.rs-5102 */             param.span,
/* FP:suggestions.rs-5103 */             format!("this could be changed to `{}: ?Sized`...", param.name.ident()),
/* FP:suggestions.rs-5104 */         );
/* FP:suggestions.rs-5105 */         for sp in visitor.invalid_spans {
/* FP:suggestions.rs-5106 */             multispan.push_span_label(
/* FP:suggestions.rs-5107 */                 sp,
/* FP:suggestions.rs-5108 */                 format!("...if indirection were used here: `Box<{}>`", param.name.ident()),
/* FP:suggestions.rs-5109 */             );
/* FP:suggestions.rs-5110 */         }
/* FP:suggestions.rs-5111 */         err.span_help(
/* FP:suggestions.rs-5112 */             multispan,
/* FP:suggestions.rs-5113 */             format!(
/* FP:suggestions.rs-5114 */                 "you could relax the implicit `Sized` bound on `{T}` if it were \
/* FP:suggestions.rs-5115 */                 used through indirection like `&{T}` or `Box<{T}>`",
/* FP:suggestions.rs-5116 */                 T = param.name.ident(),
/* FP:suggestions.rs-5117 */             ),
/* FP:suggestions.rs-5118 */         );
/* FP:suggestions.rs-5119 */         true
/* FP:suggestions.rs-5120 */     }
/* FP:suggestions.rs-5121 */     pub(crate) fn suggest_swapping_lhs_and_rhs<T>(
/* FP:suggestions.rs-5122 */         &self,
/* FP:suggestions.rs-5123 */         err: &mut Diag<'_>,
/* FP:suggestions.rs-5124 */         predicate: T,
/* FP:suggestions.rs-5125 */         param_env: ty::ParamEnv<'tcx>,
/* FP:suggestions.rs-5126 */         cause_code: &ObligationCauseCode<'tcx>,
/* FP:suggestions.rs-5127 */     ) where
/* FP:suggestions.rs-5128 */         T: Upcast<TyCtxt<'tcx>, ty::Predicate<'tcx>>,
/* FP:suggestions.rs-5129 */     {
/* FP:suggestions.rs-5130 */         let tcx = self.tcx;
/* FP:suggestions.rs-5131 */         let predicate = predicate.upcast(tcx);
/* FP:suggestions.rs-5132 */         match *cause_code {
/* FP:suggestions.rs-5133 */             ObligationCauseCode::BinOp { lhs_hir_id, rhs_hir_id, rhs_span, .. }
/* FP:suggestions.rs-5134 */                 if let Some(typeck_results) = &self.typeck_results
/* FP:suggestions.rs-5135 */                     && let hir::Node::Expr(lhs) = tcx.hir_node(lhs_hir_id)
/* FP:suggestions.rs-5136 */                     && let hir::Node::Expr(rhs) = tcx.hir_node(rhs_hir_id)
/* FP:suggestions.rs-5137 */                     && let Some(lhs_ty) = typeck_results.expr_ty_opt(lhs)
/* FP:suggestions.rs-5138 */                     && let Some(rhs_ty) = typeck_results.expr_ty_opt(rhs) =>
/* FP:suggestions.rs-5139 */             {
/* FP:suggestions.rs-5140 */                 if let Some(pred) = predicate.as_trait_clause()
/* FP:suggestions.rs-5141 */                     && tcx.is_lang_item(pred.def_id(), LangItem::PartialEq)
/* FP:suggestions.rs-5142 */                     && self
/* FP:suggestions.rs-5143 */                         .infcx
/* FP:suggestions.rs-5144 */                         .type_implements_trait(pred.def_id(), [rhs_ty, lhs_ty], param_env)
/* FP:suggestions.rs-5145 */                         .must_apply_modulo_regions()
/* FP:suggestions.rs-5146 */                 {
/* FP:suggestions.rs-5147 */                     let lhs_span = tcx.hir_span(lhs_hir_id);
/* FP:suggestions.rs-5148 */                     let sm = tcx.sess.source_map();
/* FP:suggestions.rs-5149 */                     if let Ok(rhs_snippet) = sm.span_to_snippet(rhs_span)
/* FP:suggestions.rs-5150 */                         && let Ok(lhs_snippet) = sm.span_to_snippet(lhs_span)
/* FP:suggestions.rs-5151 */                     {
/* FP:suggestions.rs-5152 */                         err.note(format!("`{rhs_ty}` implements `PartialEq<{lhs_ty}>`"));
/* FP:suggestions.rs-5153 */                         err.multipart_suggestion(
/* FP:suggestions.rs-5154 */                             "consider swapping the equality",
/* FP:suggestions.rs-5155 */                             vec![(lhs_span, rhs_snippet), (rhs_span, lhs_snippet)],
/* FP:suggestions.rs-5156 */                             Applicability::MaybeIncorrect,
/* FP:suggestions.rs-5157 */                         );
/* FP:suggestions.rs-5158 */                     }
/* FP:suggestions.rs-5159 */                 }
/* FP:suggestions.rs-5160 */             }
/* FP:suggestions.rs-5161 */             _ => {}
/* FP:suggestions.rs-5162 */         }
/* FP:suggestions.rs-5163 */     }
/* FP:suggestions.rs-5164 */ }
/* FP:suggestions.rs-5165 */ 
/* FP:suggestions.rs-5166 */ /// Add a hint to add a missing borrow or remove an unnecessary one.
/* FP:suggestions.rs-5167 */ fn hint_missing_borrow<'tcx>(
/* FP:suggestions.rs-5168 */     infcx: &InferCtxt<'tcx>,
/* FP:suggestions.rs-5169 */     param_env: ty::ParamEnv<'tcx>,
/* FP:suggestions.rs-5170 */     span: Span,
/* FP:suggestions.rs-5171 */     found: Ty<'tcx>,
/* FP:suggestions.rs-5172 */     expected: Ty<'tcx>,
/* FP:suggestions.rs-5173 */     found_node: Node<'_>,
/* FP:suggestions.rs-5174 */     err: &mut Diag<'_>,
/* FP:suggestions.rs-5175 */ ) {
/* FP:suggestions.rs-5176 */     if matches!(found_node, Node::TraitItem(..)) {
/* FP:suggestions.rs-5177 */         return;
/* FP:suggestions.rs-5178 */     }
/* FP:suggestions.rs-5179 */ 
/* FP:suggestions.rs-5180 */     let found_args = match found.kind() {
/* FP:suggestions.rs-5181 */         ty::FnPtr(sig_tys, _) => infcx.enter_forall(*sig_tys, |sig_tys| sig_tys.inputs().iter()),
/* FP:suggestions.rs-5182 */         kind => {
/* FP:suggestions.rs-5183 */             span_bug!(span, "found was converted to a FnPtr above but is now {:?}", kind)
/* FP:suggestions.rs-5184 */         }
/* FP:suggestions.rs-5185 */     };
/* FP:suggestions.rs-5186 */     let expected_args = match expected.kind() {
/* FP:suggestions.rs-5187 */         ty::FnPtr(sig_tys, _) => infcx.enter_forall(*sig_tys, |sig_tys| sig_tys.inputs().iter()),
/* FP:suggestions.rs-5188 */         kind => {
/* FP:suggestions.rs-5189 */             span_bug!(span, "expected was converted to a FnPtr above but is now {:?}", kind)
/* FP:suggestions.rs-5190 */         }
/* FP:suggestions.rs-5191 */     };
/* FP:suggestions.rs-5192 */ 
/* FP:suggestions.rs-5193 */     // This could be a variant constructor, for example.
/* FP:suggestions.rs-5194 */     let Some(fn_decl) = found_node.fn_decl() else {
/* FP:suggestions.rs-5195 */         return;
/* FP:suggestions.rs-5196 */     };
/* FP:suggestions.rs-5197 */ 
/* FP:suggestions.rs-5198 */     let args = fn_decl.inputs.iter();
/* FP:suggestions.rs-5199 */ 
/* FP:suggestions.rs-5200 */     let mut to_borrow = Vec::new();
/* FP:suggestions.rs-5201 */     let mut remove_borrow = Vec::new();
/* FP:suggestions.rs-5202 */ 
/* FP:suggestions.rs-5203 */     for ((found_arg, expected_arg), arg) in found_args.zip(expected_args).zip(args) {
/* FP:suggestions.rs-5204 */         let (found_ty, found_refs) = get_deref_type_and_refs(*found_arg);
/* FP:suggestions.rs-5205 */         let (expected_ty, expected_refs) = get_deref_type_and_refs(*expected_arg);
/* FP:suggestions.rs-5206 */ 
/* FP:suggestions.rs-5207 */         if infcx.can_eq(param_env, found_ty, expected_ty) {
/* FP:suggestions.rs-5208 */             // FIXME: This could handle more exotic cases like mutability mismatches too!
/* FP:suggestions.rs-5209 */             if found_refs.len() < expected_refs.len()
/* FP:suggestions.rs-5210 */                 && found_refs[..] == expected_refs[expected_refs.len() - found_refs.len()..]
/* FP:suggestions.rs-5211 */             {
/* FP:suggestions.rs-5212 */                 to_borrow.push((
/* FP:suggestions.rs-5213 */                     arg.span.shrink_to_lo(),
/* FP:suggestions.rs-5214 */                     expected_refs[..expected_refs.len() - found_refs.len()]
/* FP:suggestions.rs-5215 */                         .iter()
/* FP:suggestions.rs-5216 */                         .map(|mutbl| format!("&{}", mutbl.prefix_str()))
/* FP:suggestions.rs-5217 */                         .collect::<Vec<_>>()
/* FP:suggestions.rs-5218 */                         .join(""),
/* FP:suggestions.rs-5219 */                 ));
/* FP:suggestions.rs-5220 */             } else if found_refs.len() > expected_refs.len() {
/* FP:suggestions.rs-5221 */                 let mut span = arg.span.shrink_to_lo();
/* FP:suggestions.rs-5222 */                 let mut left = found_refs.len() - expected_refs.len();
/* FP:suggestions.rs-5223 */                 let mut ty = arg;
/* FP:suggestions.rs-5224 */                 while let hir::TyKind::Ref(_, mut_ty) = &ty.kind
/* FP:suggestions.rs-5225 */                     && left > 0
/* FP:suggestions.rs-5226 */                 {
/* FP:suggestions.rs-5227 */                     span = span.with_hi(mut_ty.ty.span.lo());
/* FP:suggestions.rs-5228 */                     ty = mut_ty.ty;
/* FP:suggestions.rs-5229 */                     left -= 1;
/* FP:suggestions.rs-5230 */                 }
/* FP:suggestions.rs-5231 */                 let sugg = if left == 0 {
/* FP:suggestions.rs-5232 */                     (span, String::new())
/* FP:suggestions.rs-5233 */                 } else {
/* FP:suggestions.rs-5234 */                     (arg.span, expected_arg.to_string())
/* FP:suggestions.rs-5235 */                 };
/* FP:suggestions.rs-5236 */                 remove_borrow.push(sugg);
/* FP:suggestions.rs-5237 */             }
/* FP:suggestions.rs-5238 */         }
/* FP:suggestions.rs-5239 */     }
/* FP:suggestions.rs-5240 */ 
/* FP:suggestions.rs-5241 */     if !to_borrow.is_empty() {
/* FP:suggestions.rs-5242 */         err.subdiagnostic(errors::AdjustSignatureBorrow::Borrow { to_borrow });
/* FP:suggestions.rs-5243 */     }
/* FP:suggestions.rs-5244 */ 
/* FP:suggestions.rs-5245 */     if !remove_borrow.is_empty() {
/* FP:suggestions.rs-5246 */         err.subdiagnostic(errors::AdjustSignatureBorrow::RemoveBorrow { remove_borrow });
/* FP:suggestions.rs-5247 */     }
/* FP:suggestions.rs-5248 */ }
/* FP:suggestions.rs-5249 */ 
/* FP:suggestions.rs-5250 */ /// Collect all the paths that reference `Self`.
/* FP:suggestions.rs-5251 */ /// Used to suggest replacing associated types with an explicit type in `where` clauses.
/* FP:suggestions.rs-5252 */ #[derive(Debug)]
/* FP:suggestions.rs-5253 */ pub struct SelfVisitor<'v> {
/* FP:suggestions.rs-5254 */     pub paths: Vec<&'v hir::Ty<'v>> = Vec::new(),
/* FP:suggestions.rs-5255 */     pub name: Option<Symbol>,
/* FP:suggestions.rs-5256 */ }
/* FP:suggestions.rs-5257 */ 
/* FP:suggestions.rs-5258 */ impl<'v> Visitor<'v> for SelfVisitor<'v> {
/* FP:suggestions.rs-5259 */     fn visit_ty(&mut self, ty: &'v hir::Ty<'v, AmbigArg>) {
/* FP:suggestions.rs-5260 */         if let hir::TyKind::Path(path) = ty.kind
/* FP:suggestions.rs-5261 */             && let hir::QPath::TypeRelative(inner_ty, segment) = path
/* FP:suggestions.rs-5262 */             && (Some(segment.ident.name) == self.name || self.name.is_none())
/* FP:suggestions.rs-5263 */             && let hir::TyKind::Path(inner_path) = inner_ty.kind
/* FP:suggestions.rs-5264 */             && let hir::QPath::Resolved(None, inner_path) = inner_path
/* FP:suggestions.rs-5265 */             && let Res::SelfTyAlias { .. } = inner_path.res
/* FP:suggestions.rs-5266 */         {
/* FP:suggestions.rs-5267 */             self.paths.push(ty.as_unambig_ty());
/* FP:suggestions.rs-5268 */         }
/* FP:suggestions.rs-5269 */         hir::intravisit::walk_ty(self, ty);
/* FP:suggestions.rs-5270 */     }
/* FP:suggestions.rs-5271 */ }
/* FP:suggestions.rs-5272 */ 
/* FP:suggestions.rs-5273 */ /// Collect all the returned expressions within the input expression.
/* FP:suggestions.rs-5274 */ /// Used to point at the return spans when we want to suggest some change to them.
/* FP:suggestions.rs-5275 */ #[derive(Default)]
/* FP:suggestions.rs-5276 */ pub struct ReturnsVisitor<'v> {
/* FP:suggestions.rs-5277 */     pub returns: Vec<&'v hir::Expr<'v>>,
/* FP:suggestions.rs-5278 */     in_block_tail: bool,
/* FP:suggestions.rs-5279 */ }
/* FP:suggestions.rs-5280 */ 
/* FP:suggestions.rs-5281 */ impl<'v> Visitor<'v> for ReturnsVisitor<'v> {
/* FP:suggestions.rs-5282 */     fn visit_expr(&mut self, ex: &'v hir::Expr<'v>) {
/* FP:suggestions.rs-5283 */         // Visit every expression to detect `return` paths, either through the function's tail
/* FP:suggestions.rs-5284 */         // expression or `return` statements. We walk all nodes to find `return` statements, but
/* FP:suggestions.rs-5285 */         // we only care about tail expressions when `in_block_tail` is `true`, which means that
/* FP:suggestions.rs-5286 */         // they're in the return path of the function body.
/* FP:suggestions.rs-5287 */         match ex.kind {
/* FP:suggestions.rs-5288 */             hir::ExprKind::Ret(Some(ex)) => {
/* FP:suggestions.rs-5289 */                 self.returns.push(ex);
/* FP:suggestions.rs-5290 */             }
/* FP:suggestions.rs-5291 */             hir::ExprKind::Block(block, _) if self.in_block_tail => {
/* FP:suggestions.rs-5292 */                 self.in_block_tail = false;
/* FP:suggestions.rs-5293 */                 for stmt in block.stmts {
/* FP:suggestions.rs-5294 */                     hir::intravisit::walk_stmt(self, stmt);
/* FP:suggestions.rs-5295 */                 }
/* FP:suggestions.rs-5296 */                 self.in_block_tail = true;
/* FP:suggestions.rs-5297 */                 if let Some(expr) = block.expr {
/* FP:suggestions.rs-5298 */                     self.visit_expr(expr);
/* FP:suggestions.rs-5299 */                 }
/* FP:suggestions.rs-5300 */             }
/* FP:suggestions.rs-5301 */             hir::ExprKind::If(_, then, else_opt) if self.in_block_tail => {
/* FP:suggestions.rs-5302 */                 self.visit_expr(then);
/* FP:suggestions.rs-5303 */                 if let Some(el) = else_opt {
/* FP:suggestions.rs-5304 */                     self.visit_expr(el);
/* FP:suggestions.rs-5305 */                 }
/* FP:suggestions.rs-5306 */             }
/* FP:suggestions.rs-5307 */             hir::ExprKind::Match(_, arms, _) if self.in_block_tail => {
/* FP:suggestions.rs-5308 */                 for arm in arms {
/* FP:suggestions.rs-5309 */                     self.visit_expr(arm.body);
/* FP:suggestions.rs-5310 */                 }
/* FP:suggestions.rs-5311 */             }
/* FP:suggestions.rs-5312 */             // We need to walk to find `return`s in the entire body.
/* FP:suggestions.rs-5313 */             _ if !self.in_block_tail => hir::intravisit::walk_expr(self, ex),
/* FP:suggestions.rs-5314 */             _ => self.returns.push(ex),
/* FP:suggestions.rs-5315 */         }
/* FP:suggestions.rs-5316 */     }
/* FP:suggestions.rs-5317 */ 
/* FP:suggestions.rs-5318 */     fn visit_body(&mut self, body: &hir::Body<'v>) {
/* FP:suggestions.rs-5319 */         assert!(!self.in_block_tail);
/* FP:suggestions.rs-5320 */         self.in_block_tail = true;
/* FP:suggestions.rs-5321 */         hir::intravisit::walk_body(self, body);
/* FP:suggestions.rs-5322 */     }
/* FP:suggestions.rs-5323 */ }
/* FP:suggestions.rs-5324 */ 
/* FP:suggestions.rs-5325 */ /// Collect all the awaited expressions within the input expression.
/* FP:suggestions.rs-5326 */ #[derive(Default)]
/* FP:suggestions.rs-5327 */ struct AwaitsVisitor {
/* FP:suggestions.rs-5328 */     awaits: Vec<HirId>,
/* FP:suggestions.rs-5329 */ }
/* FP:suggestions.rs-5330 */ 
/* FP:suggestions.rs-5331 */ impl<'v> Visitor<'v> for AwaitsVisitor {
/* FP:suggestions.rs-5332 */     fn visit_expr(&mut self, ex: &'v hir::Expr<'v>) {
/* FP:suggestions.rs-5333 */         if let hir::ExprKind::Yield(_, hir::YieldSource::Await { expr: Some(id) }) = ex.kind {
/* FP:suggestions.rs-5334 */             self.awaits.push(id)
/* FP:suggestions.rs-5335 */         }
/* FP:suggestions.rs-5336 */         hir::intravisit::walk_expr(self, ex)
/* FP:suggestions.rs-5337 */     }
/* FP:suggestions.rs-5338 */ }
/* FP:suggestions.rs-5339 */ 
/* FP:suggestions.rs-5340 */ /// Suggest a new type parameter name for diagnostic purposes.
/* FP:suggestions.rs-5341 */ ///
/* FP:suggestions.rs-5342 */ /// `name` is the preferred name you'd like to suggest if it's not in use already.
/* FP:suggestions.rs-5343 */ pub trait NextTypeParamName {
/* FP:suggestions.rs-5344 */     fn next_type_param_name(&self, name: Option<&str>) -> String;
/* FP:suggestions.rs-5345 */ }
/* FP:suggestions.rs-5346 */ 
/* FP:suggestions.rs-5347 */ impl NextTypeParamName for &[hir::GenericParam<'_>] {
/* FP:suggestions.rs-5348 */     fn next_type_param_name(&self, name: Option<&str>) -> String {
/* FP:suggestions.rs-5349 */         // Type names are usually single letters in uppercase. So convert the first letter of input string to uppercase.
/* FP:suggestions.rs-5350 */         let name = name.and_then(|n| n.chars().next()).map(|c| c.to_uppercase().to_string());
/* FP:suggestions.rs-5351 */         let name = name.as_deref();
/* FP:suggestions.rs-5352 */ 
/* FP:suggestions.rs-5353 */         // This is the list of possible parameter names that we might suggest.
/* FP:suggestions.rs-5354 */         let possible_names = [name.unwrap_or("T"), "T", "U", "V", "X", "Y", "Z", "A", "B", "C"];
/* FP:suggestions.rs-5355 */ 
/* FP:suggestions.rs-5356 */         // Filter out used names based on `filter_fn`.
/* FP:suggestions.rs-5357 */         let used_names: Vec<Symbol> = self
/* FP:suggestions.rs-5358 */             .iter()
/* FP:suggestions.rs-5359 */             .filter_map(|param| match param.name {
/* FP:suggestions.rs-5360 */                 hir::ParamName::Plain(ident) => Some(ident.name),
/* FP:suggestions.rs-5361 */                 _ => None,
/* FP:suggestions.rs-5362 */             })
/* FP:suggestions.rs-5363 */             .collect();
/* FP:suggestions.rs-5364 */ 
/* FP:suggestions.rs-5365 */         // Find a name from `possible_names` that is not in `used_names`.
/* FP:suggestions.rs-5366 */         possible_names
/* FP:suggestions.rs-5367 */             .iter()
/* FP:suggestions.rs-5368 */             .find(|n| !used_names.contains(&Symbol::intern(n)))
/* FP:suggestions.rs-5369 */             .unwrap_or(&"ParamName")
/* FP:suggestions.rs-5370 */             .to_string()
/* FP:suggestions.rs-5371 */     }
/* FP:suggestions.rs-5372 */ }
/* FP:suggestions.rs-5373 */ 
/* FP:suggestions.rs-5374 */ /// Collect the spans that we see the generic param `param_did`
/* FP:suggestions.rs-5375 */ struct ReplaceImplTraitVisitor<'a> {
/* FP:suggestions.rs-5376 */     ty_spans: &'a mut Vec<Span>,
/* FP:suggestions.rs-5377 */     param_did: DefId,
/* FP:suggestions.rs-5378 */ }
/* FP:suggestions.rs-5379 */ 
/* FP:suggestions.rs-5380 */ impl<'a, 'hir> hir::intravisit::Visitor<'hir> for ReplaceImplTraitVisitor<'a> {
/* FP:suggestions.rs-5381 */     fn visit_ty(&mut self, t: &'hir hir::Ty<'hir, AmbigArg>) {
/* FP:suggestions.rs-5382 */         if let hir::TyKind::Path(hir::QPath::Resolved(
/* FP:suggestions.rs-5383 */             None,
/* FP:suggestions.rs-5384 */             hir::Path { res: Res::Def(_, segment_did), .. },
/* FP:suggestions.rs-5385 */         )) = t.kind
/* FP:suggestions.rs-5386 */         {
/* FP:suggestions.rs-5387 */             if self.param_did == *segment_did {
/* FP:suggestions.rs-5388 */                 // `fn foo(t: impl Trait)`
/* FP:suggestions.rs-5389 */                 //            ^^^^^^^^^^ get this to suggest `T` instead
/* FP:suggestions.rs-5390 */ 
/* FP:suggestions.rs-5391 */                 // There might be more than one `impl Trait`.
/* FP:suggestions.rs-5392 */                 self.ty_spans.push(t.span);
/* FP:suggestions.rs-5393 */                 return;
/* FP:suggestions.rs-5394 */             }
/* FP:suggestions.rs-5395 */         }
/* FP:suggestions.rs-5396 */ 
/* FP:suggestions.rs-5397 */         hir::intravisit::walk_ty(self, t);
/* FP:suggestions.rs-5398 */     }
/* FP:suggestions.rs-5399 */ }
/* FP:suggestions.rs-5400 */ 
/* FP:suggestions.rs-5401 */ pub(super) fn get_explanation_based_on_obligation<'tcx>(
/* FP:suggestions.rs-5402 */     tcx: TyCtxt<'tcx>,
/* FP:suggestions.rs-5403 */     obligation: &PredicateObligation<'tcx>,
/* FP:suggestions.rs-5404 */     trait_predicate: ty::PolyTraitPredicate<'tcx>,
/* FP:suggestions.rs-5405 */     pre_message: String,
/* FP:suggestions.rs-5406 */     long_ty_path: &mut Option<PathBuf>,
/* FP:suggestions.rs-5407 */ ) -> String {
/* FP:suggestions.rs-5408 */     if let ObligationCauseCode::MainFunctionType = obligation.cause.code() {
/* FP:suggestions.rs-5409 */         "consider using `()`, or a `Result`".to_owned()
/* FP:suggestions.rs-5410 */     } else {
/* FP:suggestions.rs-5411 */         let ty_desc = match trait_predicate.self_ty().skip_binder().kind() {
/* FP:suggestions.rs-5412 */             ty::FnDef(_, _) => Some("fn item"),
/* FP:suggestions.rs-5413 */             ty::Closure(_, _) => Some("closure"),
/* FP:suggestions.rs-5414 */             _ => None,
/* FP:suggestions.rs-5415 */         };
/* FP:suggestions.rs-5416 */ 
/* FP:suggestions.rs-5417 */         let desc = match ty_desc {
/* FP:suggestions.rs-5418 */             Some(desc) => format!(" {desc}"),
/* FP:suggestions.rs-5419 */             None => String::new(),
/* FP:suggestions.rs-5420 */         };
/* FP:suggestions.rs-5421 */         if let ty::PredicatePolarity::Positive = trait_predicate.polarity() {
/* FP:suggestions.rs-5422 */             format!(
/* FP:suggestions.rs-5423 */                 "{pre_message}the trait `{}` is not implemented for{desc} `{}`",
/* FP:suggestions.rs-5424 */                 trait_predicate.print_modifiers_and_trait_path(),
/* FP:suggestions.rs-5425 */                 tcx.short_string(trait_predicate.self_ty().skip_binder(), long_ty_path),
/* FP:suggestions.rs-5426 */             )
/* FP:suggestions.rs-5427 */         } else {
/* FP:suggestions.rs-5428 */             // "the trait bound `T: !Send` is not satisfied" reads better than "`!Send` is
/* FP:suggestions.rs-5429 */             // not implemented for `T`".
/* FP:suggestions.rs-5430 */             // FIXME: add note explaining explicit negative trait bounds.
/* FP:suggestions.rs-5431 */             format!("{pre_message}the trait bound `{trait_predicate}` is not satisfied")
/* FP:suggestions.rs-5432 */         }
/* FP:suggestions.rs-5433 */     }
/* FP:suggestions.rs-5434 */ }
/* FP:suggestions.rs-5435 */ 
/* FP:suggestions.rs-5436 */ // Replace `param` with `replace_ty`
/* FP:suggestions.rs-5437 */ struct ReplaceImplTraitFolder<'tcx> {
/* FP:suggestions.rs-5438 */     tcx: TyCtxt<'tcx>,
/* FP:suggestions.rs-5439 */     param: &'tcx ty::GenericParamDef,
/* FP:suggestions.rs-5440 */     replace_ty: Ty<'tcx>,
/* FP:suggestions.rs-5441 */ }
/* FP:suggestions.rs-5442 */ 
/* FP:suggestions.rs-5443 */ impl<'tcx> TypeFolder<TyCtxt<'tcx>> for ReplaceImplTraitFolder<'tcx> {
/* FP:suggestions.rs-5444 */     fn fold_ty(&mut self, t: Ty<'tcx>) -> Ty<'tcx> {
/* FP:suggestions.rs-5445 */         if let ty::Param(ty::ParamTy { index, .. }) = t.kind() {
/* FP:suggestions.rs-5446 */             if self.param.index == *index {
/* FP:suggestions.rs-5447 */                 return self.replace_ty;
/* FP:suggestions.rs-5448 */             }
/* FP:suggestions.rs-5449 */         }
/* FP:suggestions.rs-5450 */         t.super_fold_with(self)
/* FP:suggestions.rs-5451 */     }
/* FP:suggestions.rs-5452 */ 
/* FP:suggestions.rs-5453 */     fn cx(&self) -> TyCtxt<'tcx> {
/* FP:suggestions.rs-5454 */         self.tcx
/* FP:suggestions.rs-5455 */     }
/* FP:suggestions.rs-5456 */ }
/* FP:suggestions.rs-5457 */ 
/* FP:suggestions.rs-5458 */ pub fn suggest_desugaring_async_fn_to_impl_future_in_trait<'tcx>(
/* FP:suggestions.rs-5459 */     tcx: TyCtxt<'tcx>,
/* FP:suggestions.rs-5460 */     sig: hir::FnSig<'tcx>,
/* FP:suggestions.rs-5461 */     body: hir::TraitFn<'tcx>,
/* FP:suggestions.rs-5462 */     opaque_def_id: LocalDefId,
/* FP:suggestions.rs-5463 */     add_bounds: &str,
/* FP:suggestions.rs-5464 */ ) -> Option<Vec<(Span, String)>> {
/* FP:suggestions.rs-5465 */     let hir::IsAsync::Async(async_span) = sig.header.asyncness else {
/* FP:suggestions.rs-5466 */         return None;
/* FP:suggestions.rs-5467 */     };
/* FP:suggestions.rs-5468 */     let async_span = tcx.sess.source_map().span_extend_while_whitespace(async_span);
/* FP:suggestions.rs-5469 */ 
/* FP:suggestions.rs-5470 */     let future = tcx.hir_node_by_def_id(opaque_def_id).expect_opaque_ty();
/* FP:suggestions.rs-5471 */     let [hir::GenericBound::Trait(trait_ref)] = future.bounds else {
/* FP:suggestions.rs-5472 */         // `async fn` should always lower to a single bound... but don't ICE.
/* FP:suggestions.rs-5473 */         return None;
/* FP:suggestions.rs-5474 */     };
/* FP:suggestions.rs-5475 */     let Some(hir::PathSegment { args: Some(args), .. }) = trait_ref.trait_ref.path.segments.last()
/* FP:suggestions.rs-5476 */     else {
/* FP:suggestions.rs-5477 */         // desugaring to a single path segment for `Future<...>`.
/* FP:suggestions.rs-5478 */         return None;
/* FP:suggestions.rs-5479 */     };
/* FP:suggestions.rs-5480 */     let Some(future_output_ty) = args.constraints.first().and_then(|constraint| constraint.ty())
/* FP:suggestions.rs-5481 */     else {
/* FP:suggestions.rs-5482 */         // Also should never happen.
/* FP:suggestions.rs-5483 */         return None;
/* FP:suggestions.rs-5484 */     };
/* FP:suggestions.rs-5485 */ 
/* FP:suggestions.rs-5486 */     let mut sugg = if future_output_ty.span.is_empty() {
/* FP:suggestions.rs-5487 */         vec![
/* FP:suggestions.rs-5488 */             (async_span, String::new()),
/* FP:suggestions.rs-5489 */             (
/* FP:suggestions.rs-5490 */                 future_output_ty.span,
/* FP:suggestions.rs-5491 */                 format!(" -> impl std::future::Future<Output = ()>{add_bounds}"),
/* FP:suggestions.rs-5492 */             ),
/* FP:suggestions.rs-5493 */         ]
/* FP:suggestions.rs-5494 */     } else {
/* FP:suggestions.rs-5495 */         vec![
/* FP:suggestions.rs-5496 */             (future_output_ty.span.shrink_to_lo(), "impl std::future::Future<Output = ".to_owned()),
/* FP:suggestions.rs-5497 */             (future_output_ty.span.shrink_to_hi(), format!(">{add_bounds}")),
/* FP:suggestions.rs-5498 */             (async_span, String::new()),
/* FP:suggestions.rs-5499 */         ]
/* FP:suggestions.rs-5500 */     };
/* FP:suggestions.rs-5501 */ 
/* FP:suggestions.rs-5502 */     // If there's a body, we also need to wrap it in `async {}`
/* FP:suggestions.rs-5503 */     if let hir::TraitFn::Provided(body) = body {
/* FP:suggestions.rs-5504 */         let body = tcx.hir_body(body);
/* FP:suggestions.rs-5505 */         let body_span = body.value.span;
/* FP:suggestions.rs-5506 */         let body_span_without_braces =
/* FP:suggestions.rs-5507 */             body_span.with_lo(body_span.lo() + BytePos(1)).with_hi(body_span.hi() - BytePos(1));
/* FP:suggestions.rs-5508 */         if body_span_without_braces.is_empty() {
/* FP:suggestions.rs-5509 */             sugg.push((body_span_without_braces, " async {} ".to_owned()));
/* FP:suggestions.rs-5510 */         } else {
/* FP:suggestions.rs-5511 */             sugg.extend([
/* FP:suggestions.rs-5512 */                 (body_span_without_braces.shrink_to_lo(), "async {".to_owned()),
/* FP:suggestions.rs-5513 */                 (body_span_without_braces.shrink_to_hi(), "} ".to_owned()),
/* FP:suggestions.rs-5514 */             ]);
/* FP:suggestions.rs-5515 */         }
/* FP:suggestions.rs-5516 */     }
/* FP:suggestions.rs-5517 */ 
/* FP:suggestions.rs-5518 */     Some(sugg)
/* FP:suggestions.rs-5519 */ }
/* FP:suggestions.rs-5520 */ 
/* FP:suggestions.rs-5521 */ /// On `impl` evaluation cycles, look for `Self::AssocTy` restrictions in `where` clauses, explain
/* FP:suggestions.rs-5522 */ /// they are not allowed and if possible suggest alternatives.
/* FP:suggestions.rs-5523 */ fn point_at_assoc_type_restriction<G: EmissionGuarantee>(
/* FP:suggestions.rs-5524 */     tcx: TyCtxt<'_>,
/* FP:suggestions.rs-5525 */     err: &mut Diag<'_, G>,
/* FP:suggestions.rs-5526 */     self_ty_str: &str,
/* FP:suggestions.rs-5527 */     trait_name: &str,
/* FP:suggestions.rs-5528 */     predicate: ty::Predicate<'_>,
/* FP:suggestions.rs-5529 */     generics: &hir::Generics<'_>,
/* FP:suggestions.rs-5530 */     data: &ImplDerivedCause<'_>,
/* FP:suggestions.rs-5531 */ ) {
/* FP:suggestions.rs-5532 */     let ty::PredicateKind::Clause(clause) = predicate.kind().skip_binder() else {
/* FP:suggestions.rs-5533 */         return;
/* FP:suggestions.rs-5534 */     };
/* FP:suggestions.rs-5535 */     let ty::ClauseKind::Projection(proj) = clause else {
/* FP:suggestions.rs-5536 */         return;
/* FP:suggestions.rs-5537 */     };
/* FP:suggestions.rs-5538 */     let name = tcx.item_name(proj.projection_term.def_id);
/* FP:suggestions.rs-5539 */     let mut predicates = generics.predicates.iter().peekable();
/* FP:suggestions.rs-5540 */     let mut prev: Option<(&hir::WhereBoundPredicate<'_>, Span)> = None;
/* FP:suggestions.rs-5541 */     while let Some(pred) = predicates.next() {
/* FP:suggestions.rs-5542 */         let curr_span = pred.span;
/* FP:suggestions.rs-5543 */         let hir::WherePredicateKind::BoundPredicate(pred) = pred.kind else {
/* FP:suggestions.rs-5544 */             continue;
/* FP:suggestions.rs-5545 */         };
/* FP:suggestions.rs-5546 */         let mut bounds = pred.bounds.iter();
/* FP:suggestions.rs-5547 */         while let Some(bound) = bounds.next() {
/* FP:suggestions.rs-5548 */             let Some(trait_ref) = bound.trait_ref() else {
/* FP:suggestions.rs-5549 */                 continue;
/* FP:suggestions.rs-5550 */             };
/* FP:suggestions.rs-5551 */             if bound.span() != data.span {
/* FP:suggestions.rs-5552 */                 continue;
/* FP:suggestions.rs-5553 */             }
/* FP:suggestions.rs-5554 */             if let hir::TyKind::Path(path) = pred.bounded_ty.kind
/* FP:suggestions.rs-5555 */                 && let hir::QPath::TypeRelative(ty, segment) = path
/* FP:suggestions.rs-5556 */                 && segment.ident.name == name
/* FP:suggestions.rs-5557 */                 && let hir::TyKind::Path(inner_path) = ty.kind
/* FP:suggestions.rs-5558 */                 && let hir::QPath::Resolved(None, inner_path) = inner_path
/* FP:suggestions.rs-5559 */                 && let Res::SelfTyAlias { .. } = inner_path.res
/* FP:suggestions.rs-5560 */             {
/* FP:suggestions.rs-5561 */                 // The following block is to determine the right span to delete for this bound
/* FP:suggestions.rs-5562 */                 // that will leave valid code after the suggestion is applied.
/* FP:suggestions.rs-5563 */                 let span = if pred.origin == hir::PredicateOrigin::WhereClause
/* FP:suggestions.rs-5564 */                     && generics
/* FP:suggestions.rs-5565 */                         .predicates
/* FP:suggestions.rs-5566 */                         .iter()
/* FP:suggestions.rs-5567 */                         .filter(|p| {
/* FP:suggestions.rs-5568 */                             matches!(
/* FP:suggestions.rs-5569 */                                 p.kind,
/* FP:suggestions.rs-5570 */                                 hir::WherePredicateKind::BoundPredicate(p)
/* FP:suggestions.rs-5571 */                                 if hir::PredicateOrigin::WhereClause == p.origin
/* FP:suggestions.rs-5572 */                             )
/* FP:suggestions.rs-5573 */                         })
/* FP:suggestions.rs-5574 */                         .count()
/* FP:suggestions.rs-5575 */                         == 1
/* FP:suggestions.rs-5576 */                 {
/* FP:suggestions.rs-5577 */                     // There's only one `where` bound, that needs to be removed. Remove the whole
/* FP:suggestions.rs-5578 */                     // `where` clause.
/* FP:suggestions.rs-5579 */                     generics.where_clause_span
/* FP:suggestions.rs-5580 */                 } else if let Some(next_pred) = predicates.peek()
/* FP:suggestions.rs-5581 */                     && let hir::WherePredicateKind::BoundPredicate(next) = next_pred.kind
/* FP:suggestions.rs-5582 */                     && pred.origin == next.origin
/* FP:suggestions.rs-5583 */                 {
/* FP:suggestions.rs-5584 */                     // There's another bound, include the comma for the current one.
/* FP:suggestions.rs-5585 */                     curr_span.until(next_pred.span)
/* FP:suggestions.rs-5586 */                 } else if let Some((prev, prev_span)) = prev
/* FP:suggestions.rs-5587 */                     && pred.origin == prev.origin
/* FP:suggestions.rs-5588 */                 {
/* FP:suggestions.rs-5589 */                     // Last bound, try to remove the previous comma.
/* FP:suggestions.rs-5590 */                     prev_span.shrink_to_hi().to(curr_span)
/* FP:suggestions.rs-5591 */                 } else if pred.origin == hir::PredicateOrigin::WhereClause {
/* FP:suggestions.rs-5592 */                     curr_span.with_hi(generics.where_clause_span.hi())
/* FP:suggestions.rs-5593 */                 } else {
/* FP:suggestions.rs-5594 */                     curr_span
/* FP:suggestions.rs-5595 */                 };
/* FP:suggestions.rs-5596 */ 
/* FP:suggestions.rs-5597 */                 err.span_suggestion_verbose(
/* FP:suggestions.rs-5598 */                     span,
/* FP:suggestions.rs-5599 */                     "associated type for the current `impl` cannot be restricted in `where` \
/* FP:suggestions.rs-5600 */                      clauses, remove this bound",
/* FP:suggestions.rs-5601 */                     "",
/* FP:suggestions.rs-5602 */                     Applicability::MaybeIncorrect,
/* FP:suggestions.rs-5603 */                 );
/* FP:suggestions.rs-5604 */             }
/* FP:suggestions.rs-5605 */             if let Some(new) =
/* FP:suggestions.rs-5606 */                 tcx.associated_items(data.impl_or_alias_def_id).find_by_ident_and_kind(
/* FP:suggestions.rs-5607 */                     tcx,
/* FP:suggestions.rs-5608 */                     Ident::with_dummy_span(name),
/* FP:suggestions.rs-5609 */                     ty::AssocTag::Type,
/* FP:suggestions.rs-5610 */                     data.impl_or_alias_def_id,
/* FP:suggestions.rs-5611 */                 )
/* FP:suggestions.rs-5612 */             {
/* FP:suggestions.rs-5613 */                 // The associated type is specified in the `impl` we're
/* FP:suggestions.rs-5614 */                 // looking at. Point at it.
/* FP:suggestions.rs-5615 */                 let span = tcx.def_span(new.def_id);
/* FP:suggestions.rs-5616 */                 err.span_label(
/* FP:suggestions.rs-5617 */                     span,
/* FP:suggestions.rs-5618 */                     format!(
/* FP:suggestions.rs-5619 */                         "associated type `<{self_ty_str} as {trait_name}>::{name}` is specified \
/* FP:suggestions.rs-5620 */                          here",
/* FP:suggestions.rs-5621 */                     ),
/* FP:suggestions.rs-5622 */                 );
/* FP:suggestions.rs-5623 */                 // Search for the associated type `Self::{name}`, get
/* FP:suggestions.rs-5624 */                 // its type and suggest replacing the bound with it.
/* FP:suggestions.rs-5625 */                 let mut visitor = SelfVisitor { name: Some(name), .. };
/* FP:suggestions.rs-5626 */                 visitor.visit_trait_ref(trait_ref);
/* FP:suggestions.rs-5627 */                 for path in visitor.paths {
/* FP:suggestions.rs-5628 */                     err.span_suggestion_verbose(
/* FP:suggestions.rs-5629 */                         path.span,
/* FP:suggestions.rs-5630 */                         "replace the associated type with the type specified in this `impl`",
/* FP:suggestions.rs-5631 */                         tcx.type_of(new.def_id).skip_binder(),
/* FP:suggestions.rs-5632 */                         Applicability::MachineApplicable,
/* FP:suggestions.rs-5633 */                     );
/* FP:suggestions.rs-5634 */                 }
/* FP:suggestions.rs-5635 */             } else {
/* FP:suggestions.rs-5636 */                 let mut visitor = SelfVisitor { name: None, .. };
/* FP:suggestions.rs-5637 */                 visitor.visit_trait_ref(trait_ref);
/* FP:suggestions.rs-5638 */                 let span: MultiSpan =
/* FP:suggestions.rs-5639 */                     visitor.paths.iter().map(|p| p.span).collect::<Vec<Span>>().into();
/* FP:suggestions.rs-5640 */                 err.span_note(
/* FP:suggestions.rs-5641 */                     span,
/* FP:suggestions.rs-5642 */                     "associated types for the current `impl` cannot be restricted in `where` \
/* FP:suggestions.rs-5643 */                      clauses",
/* FP:suggestions.rs-5644 */                 );
/* FP:suggestions.rs-5645 */             }
/* FP:suggestions.rs-5646 */         }
/* FP:suggestions.rs-5647 */         prev = Some((pred, curr_span));
/* FP:suggestions.rs-5648 */     }
/* FP:suggestions.rs-5649 */ }
/* FP:suggestions.rs-5650 */ 
/* FP:suggestions.rs-5651 */ fn get_deref_type_and_refs(mut ty: Ty<'_>) -> (Ty<'_>, Vec<hir::Mutability>) {
/* FP:suggestions.rs-5652 */     let mut refs = vec![];
/* FP:suggestions.rs-5653 */ 
/* FP:suggestions.rs-5654 */     while let ty::Ref(_, new_ty, mutbl) = ty.kind() {
/* FP:suggestions.rs-5655 */         ty = *new_ty;
/* FP:suggestions.rs-5656 */         refs.push(*mutbl);
/* FP:suggestions.rs-5657 */     }
/* FP:suggestions.rs-5658 */ 
/* FP:suggestions.rs-5659 */     (ty, refs)
/* FP:suggestions.rs-5660 */ }
/* FP:suggestions.rs-5661 */ 
/* FP:suggestions.rs-5662 */ /// Look for type `param` in an ADT being used only through a reference to confirm that suggesting
/* FP:suggestions.rs-5663 */ /// `param: ?Sized` would be a valid constraint.
/* FP:suggestions.rs-5664 */ struct FindTypeParam {
/* FP:suggestions.rs-5665 */     param: crate::rustc_span::Symbol,
/* FP:suggestions.rs-5666 */     invalid_spans: Vec<Span> = Vec::new(),
/* FP:suggestions.rs-5667 */     nested: bool = false,
/* FP:suggestions.rs-5668 */ }
/* FP:suggestions.rs-5669 */ 
/* FP:suggestions.rs-5670 */ impl<'v> Visitor<'v> for FindTypeParam {
/* FP:suggestions.rs-5671 */     fn visit_where_predicate(&mut self, _: &'v hir::WherePredicate<'v>) {
/* FP:suggestions.rs-5672 */         // Skip where-clauses, to avoid suggesting indirection for type parameters found there.
/* FP:suggestions.rs-5673 */     }
/* FP:suggestions.rs-5674 */ 
/* FP:suggestions.rs-5675 */     fn visit_ty(&mut self, ty: &hir::Ty<'_, AmbigArg>) {
/* FP:suggestions.rs-5676 */         // We collect the spans of all uses of the "bare" type param, like in `field: T` or
/* FP:suggestions.rs-5677 */         // `field: (T, T)` where we could make `T: ?Sized` while skipping cases that are known to be
/* FP:suggestions.rs-5678 */         // valid like `field: &'a T` or `field: *mut T` and cases that *might* have further `Sized`
/* FP:suggestions.rs-5679 */         // obligations like `Box<T>` and `Vec<T>`, but we perform no extra analysis for those cases
/* FP:suggestions.rs-5680 */         // and suggest `T: ?Sized` regardless of their obligations. This is fine because the errors
/* FP:suggestions.rs-5681 */         // in that case should make what happened clear enough.
/* FP:suggestions.rs-5682 */         match ty.kind {
/* FP:suggestions.rs-5683 */             hir::TyKind::Ptr(_) | hir::TyKind::Ref(..) | hir::TyKind::TraitObject(..) => {}
/* FP:suggestions.rs-5684 */             hir::TyKind::Path(hir::QPath::Resolved(None, path))
/* FP:suggestions.rs-5685 */                 if let [segment] = path.segments
/* FP:suggestions.rs-5686 */                     && segment.ident.name == self.param =>
/* FP:suggestions.rs-5687 */             {
/* FP:suggestions.rs-5688 */                 if !self.nested {
/* FP:suggestions.rs-5689 */                     debug!(?ty, "FindTypeParam::visit_ty");
/* FP:suggestions.rs-5690 */                     self.invalid_spans.push(ty.span);
/* FP:suggestions.rs-5691 */                 }
/* FP:suggestions.rs-5692 */             }
/* FP:suggestions.rs-5693 */             hir::TyKind::Path(_) => {
/* FP:suggestions.rs-5694 */                 let prev = self.nested;
/* FP:suggestions.rs-5695 */                 self.nested = true;
/* FP:suggestions.rs-5696 */                 hir::intravisit::walk_ty(self, ty);
/* FP:suggestions.rs-5697 */                 self.nested = prev;
/* FP:suggestions.rs-5698 */             }
/* FP:suggestions.rs-5699 */             _ => {
/* FP:suggestions.rs-5700 */                 hir::intravisit::walk_ty(self, ty);
/* FP:suggestions.rs-5701 */             }
/* FP:suggestions.rs-5702 */         }
/* FP:suggestions.rs-5703 */     }
/* FP:suggestions.rs-5704 */ }
/* FP:suggestions.rs-5705 */ 
/* FP:suggestions.rs-5706 */ /// Look for type parameters in predicates. We use this to identify whether a bound is suitable in
/* FP:suggestions.rs-5707 */ /// on a given item.
/* FP:suggestions.rs-5708 */ struct ParamFinder {
/* FP:suggestions.rs-5709 */     params: Vec<Symbol> = Vec::new(),
/* FP:suggestions.rs-5710 */ }
/* FP:suggestions.rs-5711 */ 
/* FP:suggestions.rs-5712 */ impl<'tcx> TypeVisitor<TyCtxt<'tcx>> for ParamFinder {
/* FP:suggestions.rs-5713 */     fn visit_ty(&mut self, t: Ty<'tcx>) -> Self::Result {
/* FP:suggestions.rs-5714 */         match t.kind() {
/* FP:suggestions.rs-5715 */             ty::Param(p) => self.params.push(p.name),
/* FP:suggestions.rs-5716 */             _ => {}
/* FP:suggestions.rs-5717 */         }
/* FP:suggestions.rs-5718 */         t.super_visit_with(self)
/* FP:suggestions.rs-5719 */     }
/* FP:suggestions.rs-5720 */ }
/* FP:suggestions.rs-5721 */ 
/* FP:suggestions.rs-5722 */ impl ParamFinder {
/* FP:suggestions.rs-5723 */     /// Whether the `hir::Generics` of the current item can suggest the evaluated bound because its
/* FP:suggestions.rs-5724 */     /// references to type parameters are present in the generics.
/* FP:suggestions.rs-5725 */     fn can_suggest_bound(&self, generics: &hir::Generics<'_>) -> bool {
/* FP:suggestions.rs-5726 */         if self.params.is_empty() {
/* FP:suggestions.rs-5727 */             // There are no references to type parameters at all, so suggesting the bound
/* FP:suggestions.rs-5728 */             // would be reasonable.
/* FP:suggestions.rs-5729 */             return true;
/* FP:suggestions.rs-5730 */         }
/* FP:suggestions.rs-5731 */         generics.params.iter().any(|p| match p.name {
/* FP:suggestions.rs-5732 */             hir::ParamName::Plain(p_name) => {
/* FP:suggestions.rs-5733 */                 // All of the parameters in the bound can be referenced in the current item.
/* FP:suggestions.rs-5734 */                 self.params.iter().any(|p| *p == p_name.name || *p == kw::SelfUpper)
/* FP:suggestions.rs-5735 */             }
/* FP:suggestions.rs-5736 */             _ => true,
/* FP:suggestions.rs-5737 */         })
/* FP:suggestions.rs-5738 */     }
/* FP:suggestions.rs-5739 */ }