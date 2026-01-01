/* FP:need_type_info.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_infer_need_type_info_UNPARSEABLE_0001
/* FP:need_type_info.rs-0002 */ use std::borrow::Cow;
/* FP:need_type_info.rs-0003 */ use std::iter;
/* FP:need_type_info.rs-0004 */ use std::path::PathBuf;
/* FP:need_type_info.rs-0005 */ 
/* FP:need_type_info.rs-0006 */ use crate::rustc_complete::codes::*;
/* FP:need_type_info.rs-0007 */ use crate::rustc_complete::{Diag, IntoDiagArg};
/* FP:need_type_info.rs-0008 */ use rustc_hir as hir;
/* FP:need_type_info.rs-0009 */ use crate::rustc_complete::def::{CtorOf, DefKind, Namespace, Res};
/* FP:need_type_info.rs-0010 */ use crate::rustc_complete::def_id::{DefId, LocalDefId};
/* FP:need_type_info.rs-0011 */ use crate::rustc_complete::intravisit::{self, Visitor};
/* FP:need_type_info.rs-0012 */ use crate::rustc_complete::{Body, Closure, Expr, ExprKind, FnRetTy, HirId, LetStmt, LocalSource};
/* FP:need_type_info.rs-0013 */ use crate::rustc_complete::bug;
/* FP:need_type_info.rs-0014 */ use crate::rustc_complete::hir::nested_filter;
/* FP:need_type_info.rs-0015 */ use crate::rustc_complete::ty::adjustment::{Adjust, Adjustment, AutoBorrow};
/* FP:need_type_info.rs-0016 */ use crate::rustc_complete::ty::print::{FmtPrinter, PrettyPrinter, Print, Printer};
/* FP:need_type_info.rs-0017 */ use crate::rustc_complete::ty::{
/* FP:need_type_info.rs-0018 */     self, GenericArg, GenericArgKind, GenericArgsRef, InferConst, IsSuggestable, Term, TermKind,
/* FP:need_type_info.rs-0019 */     Ty, TyCtxt, TypeFoldable, TypeFolder, TypeSuperFoldable, TypeVisitableExt, TypeckResults,
/* FP:need_type_info.rs-0020 */ };
/* FP:need_type_info.rs-0021 */ use crate::rustc_complete::{BytePos, DUMMY_SP, Ident, Span, sym};
/* FP:need_type_info.rs-0022 */ use tracing::{debug, instrument, warn};
/* FP:need_type_info.rs-0023 */ 
/* FP:need_type_info.rs-0024 */ use super::nice_region_error::placeholder_error::Highlighted;
/* FP:need_type_info.rs-0025 */ use crate::error_reporting::TypeErrCtxt;
/* FP:need_type_info.rs-0026 */ use crate::errors::{
/* FP:need_type_info.rs-0027 */     AmbiguousImpl, AmbiguousReturn, AnnotationRequired, InferenceBadError,
/* FP:need_type_info.rs-0028 */     SourceKindMultiSuggestion, SourceKindSubdiag,
/* FP:need_type_info.rs-0029 */ };
/* FP:need_type_info.rs-0030 */ use crate::infer::InferCtxt;
/* FP:need_type_info.rs-0031 */ 
/* FP:need_type_info.rs-0032 */ pub enum TypeAnnotationNeeded {
/* FP:need_type_info.rs-0033 */     /// ```compile_fail,E0282
/* FP:need_type_info.rs-0034 */     /// let x;
/* FP:need_type_info.rs-0035 */     /// ```
/* FP:need_type_info.rs-0036 */     E0282,
/* FP:need_type_info.rs-0037 */     /// An implementation cannot be chosen unambiguously because of lack of information.
/* FP:need_type_info.rs-0038 */     /// ```compile_fail,E0790
/* FP:need_type_info.rs-0039 */     /// let _ = Default::default();
/* FP:need_type_info.rs-0040 */     /// ```
/* FP:need_type_info.rs-0041 */     E0283,
/* FP:need_type_info.rs-0042 */     /// ```compile_fail,E0284
/* FP:need_type_info.rs-0043 */     /// let mut d: u64 = 2;
/* FP:need_type_info.rs-0044 */     /// d = d % 1u32.into();
/* FP:need_type_info.rs-0045 */     /// ```
/* FP:need_type_info.rs-0046 */     E0284,
/* FP:need_type_info.rs-0047 */ }
/* FP:need_type_info.rs-0048 */ 
/* FP:need_type_info.rs-0049 */ impl From<TypeAnnotationNeeded> for ErrCode {
/* FP:need_type_info.rs-0050 */     fn from(val: TypeAnnotationNeeded) -> Self {
/* FP:need_type_info.rs-0051 */         match val {
/* FP:need_type_info.rs-0052 */             TypeAnnotationNeeded::E0282 => E0282,
/* FP:need_type_info.rs-0053 */             TypeAnnotationNeeded::E0283 => E0283,
/* FP:need_type_info.rs-0054 */             TypeAnnotationNeeded::E0284 => E0284,
/* FP:need_type_info.rs-0055 */         }
/* FP:need_type_info.rs-0056 */     }
/* FP:need_type_info.rs-0057 */ }
/* FP:need_type_info.rs-0058 */ 
/* FP:need_type_info.rs-0059 */ /// Information about a constant or a type containing inference variables.
/* FP:need_type_info.rs-0060 */ pub struct InferenceDiagnosticsData {
/* FP:need_type_info.rs-0061 */     pub name: String,
/* FP:need_type_info.rs-0062 */     pub span: Option<Span>,
/* FP:need_type_info.rs-0063 */     pub kind: UnderspecifiedArgKind,
/* FP:need_type_info.rs-0064 */     pub parent: Option<InferenceDiagnosticsParentData>,
/* FP:need_type_info.rs-0065 */ }
/* FP:need_type_info.rs-0066 */ 
/* FP:need_type_info.rs-0067 */ /// Data on the parent definition where a generic argument was declared.
/* FP:need_type_info.rs-0068 */ pub struct InferenceDiagnosticsParentData {
/* FP:need_type_info.rs-0069 */     prefix: &'static str,
/* FP:need_type_info.rs-0070 */     name: String,
/* FP:need_type_info.rs-0071 */ }
/* FP:need_type_info.rs-0072 */ 
/* FP:need_type_info.rs-0073 */ #[derive(Clone)]
/* FP:need_type_info.rs-0074 */ pub enum UnderspecifiedArgKind {
/* FP:need_type_info.rs-0075 */     Type { prefix: Cow<'static, str> },
/* FP:need_type_info.rs-0076 */     Const { is_parameter: bool },
/* FP:need_type_info.rs-0077 */ }
/* FP:need_type_info.rs-0078 */ 
/* FP:need_type_info.rs-0079 */ impl InferenceDiagnosticsData {
/* FP:need_type_info.rs-0080 */     fn can_add_more_info(&self) -> bool {
/* FP:need_type_info.rs-0081 */         !(self.name == "_" && matches!(self.kind, UnderspecifiedArgKind::Type { .. }))
/* FP:need_type_info.rs-0082 */     }
/* FP:need_type_info.rs-0083 */ 
/* FP:need_type_info.rs-0084 */     fn where_x_is_kind(&self, in_type: Ty<'_>) -> &'static str {
/* FP:need_type_info.rs-0085 */         if in_type.is_ty_or_numeric_infer() {
/* FP:need_type_info.rs-0086 */             ""
/* FP:need_type_info.rs-0087 */         } else if self.name == "_" {
/* FP:need_type_info.rs-0088 */             // FIXME: Consider specializing this message if there is a single `_`
/* FP:need_type_info.rs-0089 */             // in the type.
/* FP:need_type_info.rs-0090 */             "underscore"
/* FP:need_type_info.rs-0091 */         } else {
/* FP:need_type_info.rs-0092 */             "has_name"
/* FP:need_type_info.rs-0093 */         }
/* FP:need_type_info.rs-0094 */     }
/* FP:need_type_info.rs-0095 */ 
/* FP:need_type_info.rs-0096 */     /// Generate a label for a generic argument which can't be inferred. When not
/* FP:need_type_info.rs-0097 */     /// much is known about the argument, `use_diag` may be used to describe the
/* FP:need_type_info.rs-0098 */     /// labeled value.
/* FP:need_type_info.rs-0099 */     fn make_bad_error(&self, span: Span) -> InferenceBadError<'_> {
/* FP:need_type_info.rs-0100 */         let has_parent = self.parent.is_some();
/* FP:need_type_info.rs-0101 */         let bad_kind = if self.can_add_more_info() { "more_info" } else { "other" };
/* FP:need_type_info.rs-0102 */         let (parent_prefix, parent_name) = self
/* FP:need_type_info.rs-0103 */             .parent
/* FP:need_type_info.rs-0104 */             .as_ref()
/* FP:need_type_info.rs-0105 */             .map(|parent| (parent.prefix, parent.name.clone()))
/* FP:need_type_info.rs-0106 */             .unwrap_or_default();
/* FP:need_type_info.rs-0107 */         InferenceBadError {
/* FP:need_type_info.rs-0108 */             span,
/* FP:need_type_info.rs-0109 */             bad_kind,
/* FP:need_type_info.rs-0110 */             prefix_kind: self.kind.clone(),
/* FP:need_type_info.rs-0111 */             prefix: self.kind.try_get_prefix().unwrap_or_default(),
/* FP:need_type_info.rs-0112 */             name: self.name.clone(),
/* FP:need_type_info.rs-0113 */             has_parent,
/* FP:need_type_info.rs-0114 */             parent_prefix,
/* FP:need_type_info.rs-0115 */             parent_name,
/* FP:need_type_info.rs-0116 */         }
/* FP:need_type_info.rs-0117 */     }
/* FP:need_type_info.rs-0118 */ }
/* FP:need_type_info.rs-0119 */ 
/* FP:need_type_info.rs-0120 */ impl InferenceDiagnosticsParentData {
/* FP:need_type_info.rs-0121 */     fn for_parent_def_id(
/* FP:need_type_info.rs-0122 */         tcx: TyCtxt<'_>,
/* FP:need_type_info.rs-0123 */         parent_def_id: DefId,
/* FP:need_type_info.rs-0124 */     ) -> Option<InferenceDiagnosticsParentData> {
/* FP:need_type_info.rs-0125 */         let parent_name =
/* FP:need_type_info.rs-0126 */             tcx.def_key(parent_def_id).disambiguated_data.data.get_opt_name()?.to_string();
/* FP:need_type_info.rs-0127 */ 
/* FP:need_type_info.rs-0128 */         Some(InferenceDiagnosticsParentData {
/* FP:need_type_info.rs-0129 */             prefix: tcx.def_descr(parent_def_id),
/* FP:need_type_info.rs-0130 */             name: parent_name,
/* FP:need_type_info.rs-0131 */         })
/* FP:need_type_info.rs-0132 */     }
/* FP:need_type_info.rs-0133 */ 
/* FP:need_type_info.rs-0134 */     fn for_def_id(tcx: TyCtxt<'_>, def_id: DefId) -> Option<InferenceDiagnosticsParentData> {
/* FP:need_type_info.rs-0135 */         Self::for_parent_def_id(tcx, tcx.parent(def_id))
/* FP:need_type_info.rs-0136 */     }
/* FP:need_type_info.rs-0137 */ }
/* FP:need_type_info.rs-0138 */ 
/* FP:need_type_info.rs-0139 */ impl IntoDiagArg for UnderspecifiedArgKind {
/* FP:need_type_info.rs-0140 */     fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> crate::rustc_errors::DiagArgValue {
/* FP:need_type_info.rs-0141 */         let kind = match self {
/* FP:need_type_info.rs-0142 */             Self::Type { .. } => "type",
/* FP:need_type_info.rs-0143 */             Self::Const { is_parameter: true } => "const_with_param",
/* FP:need_type_info.rs-0144 */             Self::Const { is_parameter: false } => "const",
/* FP:need_type_info.rs-0145 */         };
/* FP:need_type_info.rs-0146 */         crate::rustc_errors::DiagArgValue::Str(kind.into())
/* FP:need_type_info.rs-0147 */     }
/* FP:need_type_info.rs-0148 */ }
/* FP:need_type_info.rs-0149 */ 
/* FP:need_type_info.rs-0150 */ impl UnderspecifiedArgKind {
/* FP:need_type_info.rs-0151 */     fn try_get_prefix(&self) -> Option<&str> {
/* FP:need_type_info.rs-0152 */         match self {
/* FP:need_type_info.rs-0153 */             Self::Type { prefix } => Some(prefix.as_ref()),
/* FP:need_type_info.rs-0154 */             Self::Const { .. } => None,
/* FP:need_type_info.rs-0155 */         }
/* FP:need_type_info.rs-0156 */     }
/* FP:need_type_info.rs-0157 */ }
/* FP:need_type_info.rs-0158 */ 
/* FP:need_type_info.rs-0159 */ struct ClosureEraser<'a, 'tcx> {
/* FP:need_type_info.rs-0160 */     infcx: &'a InferCtxt<'tcx>,
/* FP:need_type_info.rs-0161 */ }
/* FP:need_type_info.rs-0162 */ 
/* FP:need_type_info.rs-0163 */ impl<'a, 'tcx> ClosureEraser<'a, 'tcx> {
/* FP:need_type_info.rs-0164 */     fn new_infer(&mut self) -> Ty<'tcx> {
/* FP:need_type_info.rs-0165 */         self.infcx.next_ty_var(DUMMY_SP)
/* FP:need_type_info.rs-0166 */     }
/* FP:need_type_info.rs-0167 */ }
/* FP:need_type_info.rs-0168 */ 
/* FP:need_type_info.rs-0169 */ impl<'a, 'tcx> TypeFolder<TyCtxt<'tcx>> for ClosureEraser<'a, 'tcx> {
/* FP:need_type_info.rs-0170 */     fn cx(&self) -> TyCtxt<'tcx> {
/* FP:need_type_info.rs-0171 */         self.infcx.tcx
/* FP:need_type_info.rs-0172 */     }
/* FP:need_type_info.rs-0173 */ 
/* FP:need_type_info.rs-0174 */     fn fold_ty(&mut self, ty: Ty<'tcx>) -> Ty<'tcx> {
/* FP:need_type_info.rs-0175 */         match ty.kind() {
/* FP:need_type_info.rs-0176 */             ty::Closure(_, args) => {
/* FP:need_type_info.rs-0177 */                 // For a closure type, we turn it into a function pointer so that it gets rendered
/* FP:need_type_info.rs-0178 */                 // as `fn(args) -> Ret`.
/* FP:need_type_info.rs-0179 */                 let closure_sig = args.as_closure().sig();
/* FP:need_type_info.rs-0180 */                 Ty::new_fn_ptr(
/* FP:need_type_info.rs-0181 */                     self.cx(),
/* FP:need_type_info.rs-0182 */                     self.cx().signature_unclosure(closure_sig, hir::Safety::Safe),
/* FP:need_type_info.rs-0183 */                 )
/* FP:need_type_info.rs-0184 */             }
/* FP:need_type_info.rs-0185 */             ty::Adt(_, args) if !args.iter().any(|a| a.has_infer()) => {
/* FP:need_type_info.rs-0186 */                 // We have a type that doesn't have any inference variables, so we replace
/* FP:need_type_info.rs-0187 */                 // the whole thing with `_`. The type system already knows about this type in
/* FP:need_type_info.rs-0188 */                 // its entirety and it is redundant to specify it for the user. The user only
/* FP:need_type_info.rs-0189 */                 // needs to specify the type parameters that we *couldn't* figure out.
/* FP:need_type_info.rs-0190 */                 self.new_infer()
/* FP:need_type_info.rs-0191 */             }
/* FP:need_type_info.rs-0192 */             ty::Adt(def, args) => {
/* FP:need_type_info.rs-0193 */                 let generics = self.cx().generics_of(def.did());
/* FP:need_type_info.rs-0194 */                 let generics: Vec<bool> = generics
/* FP:need_type_info.rs-0195 */                     .own_params
/* FP:need_type_info.rs-0196 */                     .iter()
/* FP:need_type_info.rs-0197 */                     .map(|param| param.default_value(self.cx()).is_some())
/* FP:need_type_info.rs-0198 */                     .collect();
/* FP:need_type_info.rs-0199 */                 let ty = Ty::new_adt(
/* FP:need_type_info.rs-0200 */                     self.cx(),
/* FP:need_type_info.rs-0201 */                     *def,
/* FP:need_type_info.rs-0202 */                     self.cx().mk_args_from_iter(generics.into_iter().zip(args.iter()).map(
/* FP:need_type_info.rs-0203 */                         |(has_default, arg)| {
/* FP:need_type_info.rs-0204 */                             if arg.has_infer() {
/* FP:need_type_info.rs-0205 */                                 // This param has an unsubstituted type variable, meaning that this
/* FP:need_type_info.rs-0206 */                                 // type has a (potentially deeply nested) type parameter from the
/* FP:need_type_info.rs-0207 */                                 // corresponding type's definition. We have explicitly asked this
/* FP:need_type_info.rs-0208 */                                 // type to not be hidden. In either case, we keep the type and don't
/* FP:need_type_info.rs-0209 */                                 // substitute with `_` just yet.
/* FP:need_type_info.rs-0210 */                                 arg.fold_with(self)
/* FP:need_type_info.rs-0211 */                             } else if has_default {
/* FP:need_type_info.rs-0212 */                                 // We have a type param that has a default type, like the allocator
/* FP:need_type_info.rs-0213 */                                 // in Vec. We decided to show `Vec` itself, because it hasn't yet
/* FP:need_type_info.rs-0214 */                                 // been replaced by an `_` `Infer`, but we want to ensure that the
/* FP:need_type_info.rs-0215 */                                 // type parameter with default types does *not* get replaced with
/* FP:need_type_info.rs-0216 */                                 // `_` because then we'd end up with `Vec<_, _>`, instead of
/* FP:need_type_info.rs-0217 */                                 // `Vec<_>`.
/* FP:need_type_info.rs-0218 */                                 arg
/* FP:need_type_info.rs-0219 */                             } else if let GenericArgKind::Type(_) = arg.kind() {
/* FP:need_type_info.rs-0220 */                                 // We don't replace lifetime or const params, only type params.
/* FP:need_type_info.rs-0221 */                                 self.new_infer().into()
/* FP:need_type_info.rs-0222 */                             } else {
/* FP:need_type_info.rs-0223 */                                 arg.fold_with(self)
/* FP:need_type_info.rs-0224 */                             }
/* FP:need_type_info.rs-0225 */                         },
/* FP:need_type_info.rs-0226 */                     )),
/* FP:need_type_info.rs-0227 */                 );
/* FP:need_type_info.rs-0228 */                 ty
/* FP:need_type_info.rs-0229 */             }
/* FP:need_type_info.rs-0230 */             _ if ty.has_infer() => {
/* FP:need_type_info.rs-0231 */                 // This type has a (potentially nested) type parameter that we couldn't figure out.
/* FP:need_type_info.rs-0232 */                 // We will print this depth of type, so at least the type name and at least one of
/* FP:need_type_info.rs-0233 */                 // its type parameters.
/* FP:need_type_info.rs-0234 */                 ty.super_fold_with(self)
/* FP:need_type_info.rs-0235 */             }
/* FP:need_type_info.rs-0236 */             // We don't have an unknown type parameter anywhere, replace with `_`.
/* FP:need_type_info.rs-0237 */             _ => self.new_infer(),
/* FP:need_type_info.rs-0238 */         }
/* FP:need_type_info.rs-0239 */     }
/* FP:need_type_info.rs-0240 */ 
/* FP:need_type_info.rs-0241 */     fn fold_const(&mut self, c: ty::Const<'tcx>) -> ty::Const<'tcx> {
/* FP:need_type_info.rs-0242 */         // Avoid accidentally erasing the type of the const.
/* FP:need_type_info.rs-0243 */         c
/* FP:need_type_info.rs-0244 */     }
/* FP:need_type_info.rs-0245 */ }
/* FP:need_type_info.rs-0246 */ 
/* FP:need_type_info.rs-0247 */ fn fmt_printer<'a, 'tcx>(infcx: &'a InferCtxt<'tcx>, ns: Namespace) -> FmtPrinter<'a, 'tcx> {
/* FP:need_type_info.rs-0248 */     let mut p = FmtPrinter::new(infcx.tcx, ns);
/* FP:need_type_info.rs-0249 */     let ty_getter = move |ty_vid| {
/* FP:need_type_info.rs-0250 */         if infcx.probe_ty_var(ty_vid).is_ok() {
/* FP:need_type_info.rs-0251 */             warn!("resolved ty var in error message");
/* FP:need_type_info.rs-0252 */         }
/* FP:need_type_info.rs-0253 */ 
/* FP:need_type_info.rs-0254 */         let var_origin = infcx.type_var_origin(ty_vid);
/* FP:need_type_info.rs-0255 */         if let Some(def_id) = var_origin.param_def_id
/* FP:need_type_info.rs-0256 */             // The `Self` param of a trait has the def-id of the trait,
/* FP:need_type_info.rs-0257 */             // since it's a synthetic parameter.
/* FP:need_type_info.rs-0258 */             && infcx.tcx.def_kind(def_id) == DefKind::TyParam
/* FP:need_type_info.rs-0259 */             && let name = infcx.tcx.item_name(def_id)
/* FP:need_type_info.rs-0260 */             && !var_origin.span.from_expansion()
/* FP:need_type_info.rs-0261 */         {
/* FP:need_type_info.rs-0262 */             let generics = infcx.tcx.generics_of(infcx.tcx.parent(def_id));
/* FP:need_type_info.rs-0263 */             let idx = generics.param_def_id_to_index(infcx.tcx, def_id).unwrap();
/* FP:need_type_info.rs-0264 */             let generic_param_def = generics.param_at(idx as usize, infcx.tcx);
/* FP:need_type_info.rs-0265 */             if let ty::GenericParamDefKind::Type { synthetic: true, .. } = generic_param_def.kind {
/* FP:need_type_info.rs-0266 */                 None
/* FP:need_type_info.rs-0267 */             } else {
/* FP:need_type_info.rs-0268 */                 Some(name)
/* FP:need_type_info.rs-0269 */             }
/* FP:need_type_info.rs-0270 */         } else {
/* FP:need_type_info.rs-0271 */             None
/* FP:need_type_info.rs-0272 */         }
/* FP:need_type_info.rs-0273 */     };
/* FP:need_type_info.rs-0274 */     p.ty_infer_name_resolver = Some(Box::new(ty_getter));
/* FP:need_type_info.rs-0275 */     let const_getter =
/* FP:need_type_info.rs-0276 */         move |ct_vid| Some(infcx.tcx.item_name(infcx.const_var_origin(ct_vid)?.param_def_id?));
/* FP:need_type_info.rs-0277 */     p.const_infer_name_resolver = Some(Box::new(const_getter));
/* FP:need_type_info.rs-0278 */     p
/* FP:need_type_info.rs-0279 */ }
/* FP:need_type_info.rs-0280 */ 
/* FP:need_type_info.rs-0281 */ fn ty_to_string<'tcx>(
/* FP:need_type_info.rs-0282 */     infcx: &InferCtxt<'tcx>,
/* FP:need_type_info.rs-0283 */     ty: Ty<'tcx>,
/* FP:need_type_info.rs-0284 */     called_method_def_id: Option<DefId>,
/* FP:need_type_info.rs-0285 */ ) -> String {
/* FP:need_type_info.rs-0286 */     let mut p = fmt_printer(infcx, Namespace::TypeNS);
/* FP:need_type_info.rs-0287 */     let ty = infcx.resolve_vars_if_possible(ty);
/* FP:need_type_info.rs-0288 */     // We use `fn` ptr syntax for closures, but this only works when the closure does not capture
/* FP:need_type_info.rs-0289 */     // anything. We also remove all type parameters that are fully known to the type system.
/* FP:need_type_info.rs-0290 */     let ty = ty.fold_with(&mut ClosureEraser { infcx });
/* FP:need_type_info.rs-0291 */ 
/* FP:need_type_info.rs-0292 */     match (ty.kind(), called_method_def_id) {
/* FP:need_type_info.rs-0293 */         // We don't want the regular output for `fn`s because it includes its path in
/* FP:need_type_info.rs-0294 */         // invalid pseudo-syntax, we want the `fn`-pointer output instead.
/* FP:need_type_info.rs-0295 */         (ty::FnDef(..), _) => {
/* FP:need_type_info.rs-0296 */             ty.fn_sig(infcx.tcx).print(&mut p).unwrap();
/* FP:need_type_info.rs-0297 */             p.into_buffer()
/* FP:need_type_info.rs-0298 */         }
/* FP:need_type_info.rs-0299 */         (_, Some(def_id))
/* FP:need_type_info.rs-0300 */             if ty.is_ty_or_numeric_infer()
/* FP:need_type_info.rs-0301 */                 && infcx.tcx.get_diagnostic_item(sym::iterator_collect_fn) == Some(def_id) =>
/* FP:need_type_info.rs-0302 */         {
/* FP:need_type_info.rs-0303 */             "Vec<_>".to_string()
/* FP:need_type_info.rs-0304 */         }
/* FP:need_type_info.rs-0305 */         _ if ty.is_ty_or_numeric_infer() => "/* Type */".to_string(),
/* FP:need_type_info.rs-0306 */         _ => {
/* FP:need_type_info.rs-0307 */             ty.print(&mut p).unwrap();
/* FP:need_type_info.rs-0308 */             p.into_buffer()
/* FP:need_type_info.rs-0309 */         }
/* FP:need_type_info.rs-0310 */     }
/* FP:need_type_info.rs-0311 */ }
/* FP:need_type_info.rs-0312 */ 
/* FP:need_type_info.rs-0313 */ /// We don't want to directly use `ty_to_string` for closures as their type isn't really
/* FP:need_type_info.rs-0314 */ /// something users are familiar with. Directly printing the `fn_sig` of closures also
/* FP:need_type_info.rs-0315 */ /// doesn't work as they actually use the "rust-call" API.
/* FP:need_type_info.rs-0316 */ fn closure_as_fn_str<'tcx>(infcx: &InferCtxt<'tcx>, ty: Ty<'tcx>) -> String {
/* FP:need_type_info.rs-0317 */     let ty::Closure(_, args) = ty.kind() else {
/* FP:need_type_info.rs-0318 */         bug!("cannot convert non-closure to fn str in `closure_as_fn_str`")
/* FP:need_type_info.rs-0319 */     };
/* FP:need_type_info.rs-0320 */     let fn_sig = args.as_closure().sig();
/* FP:need_type_info.rs-0321 */     let args = fn_sig
/* FP:need_type_info.rs-0322 */         .inputs()
/* FP:need_type_info.rs-0323 */         .skip_binder()
/* FP:need_type_info.rs-0324 */         .iter()
/* FP:need_type_info.rs-0325 */         .next()
/* FP:need_type_info.rs-0326 */         .map(|args| {
/* FP:need_type_info.rs-0327 */             args.tuple_fields()
/* FP:need_type_info.rs-0328 */                 .iter()
/* FP:need_type_info.rs-0329 */                 .map(|arg| ty_to_string(infcx, arg, None))
/* FP:need_type_info.rs-0330 */                 .collect::<Vec<_>>()
/* FP:need_type_info.rs-0331 */                 .join(", ")
/* FP:need_type_info.rs-0332 */         })
/* FP:need_type_info.rs-0333 */         .unwrap_or_default();
/* FP:need_type_info.rs-0334 */     let ret = if fn_sig.output().skip_binder().is_unit() {
/* FP:need_type_info.rs-0335 */         String::new()
/* FP:need_type_info.rs-0336 */     } else {
/* FP:need_type_info.rs-0337 */         format!(" -> {}", ty_to_string(infcx, fn_sig.output().skip_binder(), None))
/* FP:need_type_info.rs-0338 */     };
/* FP:need_type_info.rs-0339 */     format!("fn({args}){ret}")
/* FP:need_type_info.rs-0340 */ }
/* FP:need_type_info.rs-0341 */ 
/* FP:need_type_info.rs-0342 */ impl<'a, 'tcx> TypeErrCtxt<'a, 'tcx> {
/* FP:need_type_info.rs-0343 */     /// Extracts data used by diagnostic for either types or constants
/* FP:need_type_info.rs-0344 */     /// which were stuck during inference.
/* FP:need_type_info.rs-0345 */     pub fn extract_inference_diagnostics_data(
/* FP:need_type_info.rs-0346 */         &self,
/* FP:need_type_info.rs-0347 */         term: Term<'tcx>,
/* FP:need_type_info.rs-0348 */         highlight: ty::print::RegionHighlightMode<'tcx>,
/* FP:need_type_info.rs-0349 */     ) -> InferenceDiagnosticsData {
/* FP:need_type_info.rs-0350 */         let tcx = self.tcx;
/* FP:need_type_info.rs-0351 */         match term.kind() {
/* FP:need_type_info.rs-0352 */             TermKind::Ty(ty) => {
/* FP:need_type_info.rs-0353 */                 if let ty::Infer(ty::TyVar(ty_vid)) = *ty.kind() {
/* FP:need_type_info.rs-0354 */                     let var_origin = self.infcx.type_var_origin(ty_vid);
/* FP:need_type_info.rs-0355 */                     if let Some(def_id) = var_origin.param_def_id
/* FP:need_type_info.rs-0356 */                         // The `Self` param of a trait has the def-id of the trait,
/* FP:need_type_info.rs-0357 */                         // since it's a synthetic parameter.
/* FP:need_type_info.rs-0358 */                         && self.tcx.def_kind(def_id) == DefKind::TyParam
/* FP:need_type_info.rs-0359 */                         && !var_origin.span.from_expansion()
/* FP:need_type_info.rs-0360 */                     {
/* FP:need_type_info.rs-0361 */                         return InferenceDiagnosticsData {
/* FP:need_type_info.rs-0362 */                             name: self.tcx.item_name(def_id).to_string(),
/* FP:need_type_info.rs-0363 */                             span: Some(var_origin.span),
/* FP:need_type_info.rs-0364 */                             kind: UnderspecifiedArgKind::Type { prefix: "type parameter".into() },
/* FP:need_type_info.rs-0365 */                             parent: InferenceDiagnosticsParentData::for_def_id(self.tcx, def_id),
/* FP:need_type_info.rs-0366 */                         };
/* FP:need_type_info.rs-0367 */                     }
/* FP:need_type_info.rs-0368 */                 }
/* FP:need_type_info.rs-0369 */ 
/* FP:need_type_info.rs-0370 */                 InferenceDiagnosticsData {
/* FP:need_type_info.rs-0371 */                     name: Highlighted { highlight, ns: Namespace::TypeNS, tcx, value: ty }
/* FP:need_type_info.rs-0372 */                         .to_string(),
/* FP:need_type_info.rs-0373 */                     span: None,
/* FP:need_type_info.rs-0374 */                     kind: UnderspecifiedArgKind::Type { prefix: ty.prefix_string(self.tcx) },
/* FP:need_type_info.rs-0375 */                     parent: None,
/* FP:need_type_info.rs-0376 */                 }
/* FP:need_type_info.rs-0377 */             }
/* FP:need_type_info.rs-0378 */             TermKind::Const(ct) => {
/* FP:need_type_info.rs-0379 */                 if let ty::ConstKind::Infer(InferConst::Var(vid)) = ct.kind() {
/* FP:need_type_info.rs-0380 */                     let origin = self.const_var_origin(vid).expect("expected unresolved const var");
/* FP:need_type_info.rs-0381 */                     if let Some(def_id) = origin.param_def_id {
/* FP:need_type_info.rs-0382 */                         return InferenceDiagnosticsData {
/* FP:need_type_info.rs-0383 */                             name: self.tcx.item_name(def_id).to_string(),
/* FP:need_type_info.rs-0384 */                             span: Some(origin.span),
/* FP:need_type_info.rs-0385 */                             kind: UnderspecifiedArgKind::Const { is_parameter: true },
/* FP:need_type_info.rs-0386 */                             parent: InferenceDiagnosticsParentData::for_def_id(self.tcx, def_id),
/* FP:need_type_info.rs-0387 */                         };
/* FP:need_type_info.rs-0388 */                     }
/* FP:need_type_info.rs-0389 */ 
/* FP:need_type_info.rs-0390 */                     debug_assert!(!origin.span.is_dummy());
/* FP:need_type_info.rs-0391 */                     InferenceDiagnosticsData {
/* FP:need_type_info.rs-0392 */                         name: Highlighted { highlight, ns: Namespace::ValueNS, tcx, value: ct }
/* FP:need_type_info.rs-0393 */                             .to_string(),
/* FP:need_type_info.rs-0394 */                         span: Some(origin.span),
/* FP:need_type_info.rs-0395 */                         kind: UnderspecifiedArgKind::Const { is_parameter: false },
/* FP:need_type_info.rs-0396 */                         parent: None,
/* FP:need_type_info.rs-0397 */                     }
/* FP:need_type_info.rs-0398 */                 } else {
/* FP:need_type_info.rs-0399 */                     // If we end up here the `FindInferSourceVisitor`
/* FP:need_type_info.rs-0400 */                     // won't work, as its expected argument isn't an inference variable.
/* FP:need_type_info.rs-0401 */                     //
/* FP:need_type_info.rs-0402 */                     // FIXME: Ideally we should look into the generic constant
/* FP:need_type_info.rs-0403 */                     // to figure out which inference var is actually unresolved so that
/* FP:need_type_info.rs-0404 */                     // this path is unreachable.
/* FP:need_type_info.rs-0405 */                     InferenceDiagnosticsData {
/* FP:need_type_info.rs-0406 */                         name: Highlighted { highlight, ns: Namespace::ValueNS, tcx, value: ct }
/* FP:need_type_info.rs-0407 */                             .to_string(),
/* FP:need_type_info.rs-0408 */                         span: None,
/* FP:need_type_info.rs-0409 */                         kind: UnderspecifiedArgKind::Const { is_parameter: false },
/* FP:need_type_info.rs-0410 */                         parent: None,
/* FP:need_type_info.rs-0411 */                     }
/* FP:need_type_info.rs-0412 */                 }
/* FP:need_type_info.rs-0413 */             }
/* FP:need_type_info.rs-0414 */         }
/* FP:need_type_info.rs-0415 */     }
/* FP:need_type_info.rs-0416 */ 
/* FP:need_type_info.rs-0417 */     /// Used as a fallback in [TypeErrCtxt::emit_inference_failure_err]
/* FP:need_type_info.rs-0418 */     /// in case we weren't able to get a better error.
/* FP:need_type_info.rs-0419 */     fn bad_inference_failure_err(
/* FP:need_type_info.rs-0420 */         &self,
/* FP:need_type_info.rs-0421 */         span: Span,
/* FP:need_type_info.rs-0422 */         arg_data: InferenceDiagnosticsData,
/* FP:need_type_info.rs-0423 */         error_code: TypeAnnotationNeeded,
/* FP:need_type_info.rs-0424 */     ) -> Diag<'a> {
/* FP:need_type_info.rs-0425 */         let source_kind = "other";
/* FP:need_type_info.rs-0426 */         let source_name = "";
/* FP:need_type_info.rs-0427 */         let failure_span = None;
/* FP:need_type_info.rs-0428 */         let infer_subdiags = Vec::new();
/* FP:need_type_info.rs-0429 */         let multi_suggestions = Vec::new();
/* FP:need_type_info.rs-0430 */         let bad_label = Some(arg_data.make_bad_error(span));
/* FP:need_type_info.rs-0431 */         match error_code {
/* FP:need_type_info.rs-0432 */             TypeAnnotationNeeded::E0282 => self.dcx().create_err(AnnotationRequired {
/* FP:need_type_info.rs-0433 */                 span,
/* FP:need_type_info.rs-0434 */                 source_kind,
/* FP:need_type_info.rs-0435 */                 source_name,
/* FP:need_type_info.rs-0436 */                 failure_span,
/* FP:need_type_info.rs-0437 */                 infer_subdiags,
/* FP:need_type_info.rs-0438 */                 multi_suggestions,
/* FP:need_type_info.rs-0439 */                 bad_label,
/* FP:need_type_info.rs-0440 */             }),
/* FP:need_type_info.rs-0441 */             TypeAnnotationNeeded::E0283 => self.dcx().create_err(AmbiguousImpl {
/* FP:need_type_info.rs-0442 */                 span,
/* FP:need_type_info.rs-0443 */                 source_kind,
/* FP:need_type_info.rs-0444 */                 source_name,
/* FP:need_type_info.rs-0445 */                 failure_span,
/* FP:need_type_info.rs-0446 */                 infer_subdiags,
/* FP:need_type_info.rs-0447 */                 multi_suggestions,
/* FP:need_type_info.rs-0448 */                 bad_label,
/* FP:need_type_info.rs-0449 */             }),
/* FP:need_type_info.rs-0450 */             TypeAnnotationNeeded::E0284 => self.dcx().create_err(AmbiguousReturn {
/* FP:need_type_info.rs-0451 */                 span,
/* FP:need_type_info.rs-0452 */                 source_kind,
/* FP:need_type_info.rs-0453 */                 source_name,
/* FP:need_type_info.rs-0454 */                 failure_span,
/* FP:need_type_info.rs-0455 */                 infer_subdiags,
/* FP:need_type_info.rs-0456 */                 multi_suggestions,
/* FP:need_type_info.rs-0457 */                 bad_label,
/* FP:need_type_info.rs-0458 */             }),
/* FP:need_type_info.rs-0459 */         }
/* FP:need_type_info.rs-0460 */     }
/* FP:need_type_info.rs-0461 */ 
/* FP:need_type_info.rs-0462 */     #[instrument(level = "debug", skip(self, error_code))]
/* FP:need_type_info.rs-0463 */     pub fn emit_inference_failure_err(
/* FP:need_type_info.rs-0464 */         &self,
/* FP:need_type_info.rs-0465 */         body_def_id: LocalDefId,
/* FP:need_type_info.rs-0466 */         failure_span: Span,
/* FP:need_type_info.rs-0467 */         term: Term<'tcx>,
/* FP:need_type_info.rs-0468 */         error_code: TypeAnnotationNeeded,
/* FP:need_type_info.rs-0469 */         should_label_span: bool,
/* FP:need_type_info.rs-0470 */     ) -> Diag<'a> {
/* FP:need_type_info.rs-0471 */         let term = self.resolve_vars_if_possible(term);
/* FP:need_type_info.rs-0472 */         let arg_data = self
/* FP:need_type_info.rs-0473 */             .extract_inference_diagnostics_data(term, ty::print::RegionHighlightMode::default());
/* FP:need_type_info.rs-0474 */ 
/* FP:need_type_info.rs-0475 */         let Some(typeck_results) = &self.typeck_results else {
/* FP:need_type_info.rs-0476 */             // If we don't have any typeck results we're outside
/* FP:need_type_info.rs-0477 */             // of a body, so we won't be able to get better info
/* FP:need_type_info.rs-0478 */             // here.
/* FP:need_type_info.rs-0479 */             return self.bad_inference_failure_err(failure_span, arg_data, error_code);
/* FP:need_type_info.rs-0480 */         };
/* FP:need_type_info.rs-0481 */ 
/* FP:need_type_info.rs-0482 */         let mut local_visitor = FindInferSourceVisitor::new(self, typeck_results, term);
/* FP:need_type_info.rs-0483 */         if let Some(body) = self.tcx.hir_maybe_body_owned_by(
/* FP:need_type_info.rs-0484 */             self.tcx.typeck_root_def_id(body_def_id.to_def_id()).expect_local(),
/* FP:need_type_info.rs-0485 */         ) {
/* FP:need_type_info.rs-0486 */             let expr = body.value;
/* FP:need_type_info.rs-0487 */             local_visitor.visit_expr(expr);
/* FP:need_type_info.rs-0488 */         }
/* FP:need_type_info.rs-0489 */ 
/* FP:need_type_info.rs-0490 */         let Some(InferSource { span, kind }) = local_visitor.infer_source else {
/* FP:need_type_info.rs-0491 */             return self.bad_inference_failure_err(failure_span, arg_data, error_code);
/* FP:need_type_info.rs-0492 */         };
/* FP:need_type_info.rs-0493 */ 
/* FP:need_type_info.rs-0494 */         let (source_kind, name, long_ty_path) = kind.ty_localized_msg(self);
/* FP:need_type_info.rs-0495 */         let failure_span = if should_label_span && !failure_span.overlaps(span) {
/* FP:need_type_info.rs-0496 */             Some(failure_span)
/* FP:need_type_info.rs-0497 */         } else {
/* FP:need_type_info.rs-0498 */             None
/* FP:need_type_info.rs-0499 */         };
/* FP:need_type_info.rs-0500 */ 
/* FP:need_type_info.rs-0501 */         let mut infer_subdiags = Vec::new();
/* FP:need_type_info.rs-0502 */         let mut multi_suggestions = Vec::new();
/* FP:need_type_info.rs-0503 */         match kind {
/* FP:need_type_info.rs-0504 */             InferSourceKind::LetBinding { insert_span, pattern_name, ty, def_id } => {
/* FP:need_type_info.rs-0505 */                 infer_subdiags.push(SourceKindSubdiag::LetLike {
/* FP:need_type_info.rs-0506 */                     span: insert_span,
/* FP:need_type_info.rs-0507 */                     name: pattern_name.map(|name| name.to_string()).unwrap_or_else(String::new),
/* FP:need_type_info.rs-0508 */                     x_kind: arg_data.where_x_is_kind(ty),
/* FP:need_type_info.rs-0509 */                     prefix_kind: arg_data.kind.clone(),
/* FP:need_type_info.rs-0510 */                     prefix: arg_data.kind.try_get_prefix().unwrap_or_default(),
/* FP:need_type_info.rs-0511 */                     arg_name: arg_data.name,
/* FP:need_type_info.rs-0512 */                     kind: if pattern_name.is_some() { "with_pattern" } else { "other" },
/* FP:need_type_info.rs-0513 */                     type_name: ty_to_string(self, ty, def_id),
/* FP:need_type_info.rs-0514 */                 });
/* FP:need_type_info.rs-0515 */             }
/* FP:need_type_info.rs-0516 */             InferSourceKind::ClosureArg { insert_span, ty } => {
/* FP:need_type_info.rs-0517 */                 infer_subdiags.push(SourceKindSubdiag::LetLike {
/* FP:need_type_info.rs-0518 */                     span: insert_span,
/* FP:need_type_info.rs-0519 */                     name: String::new(),
/* FP:need_type_info.rs-0520 */                     x_kind: arg_data.where_x_is_kind(ty),
/* FP:need_type_info.rs-0521 */                     prefix_kind: arg_data.kind.clone(),
/* FP:need_type_info.rs-0522 */                     prefix: arg_data.kind.try_get_prefix().unwrap_or_default(),
/* FP:need_type_info.rs-0523 */                     arg_name: arg_data.name,
/* FP:need_type_info.rs-0524 */                     kind: "closure",
/* FP:need_type_info.rs-0525 */                     type_name: ty_to_string(self, ty, None),
/* FP:need_type_info.rs-0526 */                 });
/* FP:need_type_info.rs-0527 */             }
/* FP:need_type_info.rs-0528 */             InferSourceKind::GenericArg {
/* FP:need_type_info.rs-0529 */                 insert_span,
/* FP:need_type_info.rs-0530 */                 argument_index,
/* FP:need_type_info.rs-0531 */                 generics_def_id,
/* FP:need_type_info.rs-0532 */                 def_id: _,
/* FP:need_type_info.rs-0533 */                 generic_args,
/* FP:need_type_info.rs-0534 */                 have_turbofish,
/* FP:need_type_info.rs-0535 */             } => {
/* FP:need_type_info.rs-0536 */                 let generics = self.tcx.generics_of(generics_def_id);
/* FP:need_type_info.rs-0537 */                 let is_type = term.as_type().is_some();
/* FP:need_type_info.rs-0538 */ 
/* FP:need_type_info.rs-0539 */                 let (parent_exists, parent_prefix, parent_name) =
/* FP:need_type_info.rs-0540 */                     InferenceDiagnosticsParentData::for_parent_def_id(self.tcx, generics_def_id)
/* FP:need_type_info.rs-0541 */                         .map_or((false, String::new(), String::new()), |parent| {
/* FP:need_type_info.rs-0542 */                             (true, parent.prefix.to_string(), parent.name)
/* FP:need_type_info.rs-0543 */                         });
/* FP:need_type_info.rs-0544 */ 
/* FP:need_type_info.rs-0545 */                 infer_subdiags.push(SourceKindSubdiag::GenericLabel {
/* FP:need_type_info.rs-0546 */                     span,
/* FP:need_type_info.rs-0547 */                     is_type,
/* FP:need_type_info.rs-0548 */                     param_name: generics.own_params[argument_index].name.to_string(),
/* FP:need_type_info.rs-0549 */                     parent_exists,
/* FP:need_type_info.rs-0550 */                     parent_prefix,
/* FP:need_type_info.rs-0551 */                     parent_name,
/* FP:need_type_info.rs-0552 */                 });
/* FP:need_type_info.rs-0553 */ 
/* FP:need_type_info.rs-0554 */                 let args = if self.tcx.get_diagnostic_item(sym::iterator_collect_fn)
/* FP:need_type_info.rs-0555 */                     == Some(generics_def_id)
/* FP:need_type_info.rs-0556 */                 {
/* FP:need_type_info.rs-0557 */                     "Vec<_>".to_string()
/* FP:need_type_info.rs-0558 */                 } else {
/* FP:need_type_info.rs-0559 */                     let mut p = fmt_printer(self, Namespace::TypeNS);
/* FP:need_type_info.rs-0560 */                     p.comma_sep(generic_args.iter().copied().map(|arg| {
/* FP:need_type_info.rs-0561 */                         if arg.is_suggestable(self.tcx, true) {
/* FP:need_type_info.rs-0562 */                             return arg;
/* FP:need_type_info.rs-0563 */                         }
/* FP:need_type_info.rs-0564 */ 
/* FP:need_type_info.rs-0565 */                         match arg.kind() {
/* FP:need_type_info.rs-0566 */                             GenericArgKind::Lifetime(_) => bug!("unexpected lifetime"),
/* FP:need_type_info.rs-0567 */                             GenericArgKind::Type(_) => self.next_ty_var(DUMMY_SP).into(),
/* FP:need_type_info.rs-0568 */                             GenericArgKind::Const(_) => self.next_const_var(DUMMY_SP).into(),
/* FP:need_type_info.rs-0569 */                         }
/* FP:need_type_info.rs-0570 */                     }))
/* FP:need_type_info.rs-0571 */                     .unwrap();
/* FP:need_type_info.rs-0572 */                     p.into_buffer()
/* FP:need_type_info.rs-0573 */                 };
/* FP:need_type_info.rs-0574 */ 
/* FP:need_type_info.rs-0575 */                 if !have_turbofish {
/* FP:need_type_info.rs-0576 */                     infer_subdiags.push(SourceKindSubdiag::GenericSuggestion {
/* FP:need_type_info.rs-0577 */                         span: insert_span,
/* FP:need_type_info.rs-0578 */                         arg_count: generic_args.len(),
/* FP:need_type_info.rs-0579 */                         args,
/* FP:need_type_info.rs-0580 */                     });
/* FP:need_type_info.rs-0581 */                 }
/* FP:need_type_info.rs-0582 */             }
/* FP:need_type_info.rs-0583 */             InferSourceKind::FullyQualifiedMethodCall { receiver, successor, args, def_id } => {
/* FP:need_type_info.rs-0584 */                 let placeholder = Some(self.next_ty_var(DUMMY_SP));
/* FP:need_type_info.rs-0585 */                 if let Some(args) = args.make_suggestable(self.infcx.tcx, true, placeholder) {
/* FP:need_type_info.rs-0586 */                     let mut p = fmt_printer(self, Namespace::ValueNS);
/* FP:need_type_info.rs-0587 */                     p.print_def_path(def_id, args).unwrap();
/* FP:need_type_info.rs-0588 */                     let def_path = p.into_buffer();
/* FP:need_type_info.rs-0589 */ 
/* FP:need_type_info.rs-0590 */                     // We only care about whether we have to add `&` or `&mut ` for now.
/* FP:need_type_info.rs-0591 */                     // This is the case if the last adjustment is a borrow and the
/* FP:need_type_info.rs-0592 */                     // first adjustment was not a builtin deref.
/* FP:need_type_info.rs-0593 */                     let adjustment = match typeck_results.expr_adjustments(receiver) {
/* FP:need_type_info.rs-0594 */                         [
/* FP:need_type_info.rs-0595 */                             Adjustment { kind: Adjust::Deref(None), target: _ },
/* FP:need_type_info.rs-0596 */                             ..,
/* FP:need_type_info.rs-0597 */                             Adjustment { kind: Adjust::Borrow(AutoBorrow::Ref(..)), target: _ },
/* FP:need_type_info.rs-0598 */                         ] => "",
/* FP:need_type_info.rs-0599 */                         [
/* FP:need_type_info.rs-0600 */                             ..,
/* FP:need_type_info.rs-0601 */                             Adjustment { kind: Adjust::Borrow(AutoBorrow::Ref(mut_)), target: _ },
/* FP:need_type_info.rs-0602 */                         ] => hir::Mutability::from(*mut_).ref_prefix_str(),
/* FP:need_type_info.rs-0603 */                         _ => "",
/* FP:need_type_info.rs-0604 */                     };
/* FP:need_type_info.rs-0605 */ 
/* FP:need_type_info.rs-0606 */                     multi_suggestions.push(SourceKindMultiSuggestion::new_fully_qualified(
/* FP:need_type_info.rs-0607 */                         receiver.span,
/* FP:need_type_info.rs-0608 */                         def_path,
/* FP:need_type_info.rs-0609 */                         adjustment,
/* FP:need_type_info.rs-0610 */                         successor,
/* FP:need_type_info.rs-0611 */                     ));
/* FP:need_type_info.rs-0612 */                 }
/* FP:need_type_info.rs-0613 */             }
/* FP:need_type_info.rs-0614 */             InferSourceKind::ClosureReturn { ty, data, should_wrap_expr } => {
/* FP:need_type_info.rs-0615 */                 let placeholder = Some(self.next_ty_var(DUMMY_SP));
/* FP:need_type_info.rs-0616 */                 if let Some(ty) = ty.make_suggestable(self.infcx.tcx, true, placeholder) {
/* FP:need_type_info.rs-0617 */                     let ty_info = ty_to_string(self, ty, None);
/* FP:need_type_info.rs-0618 */                     multi_suggestions.push(SourceKindMultiSuggestion::new_closure_return(
/* FP:need_type_info.rs-0619 */                         ty_info,
/* FP:need_type_info.rs-0620 */                         data,
/* FP:need_type_info.rs-0621 */                         should_wrap_expr,
/* FP:need_type_info.rs-0622 */                     ));
/* FP:need_type_info.rs-0623 */                 }
/* FP:need_type_info.rs-0624 */             }
/* FP:need_type_info.rs-0625 */         }
/* FP:need_type_info.rs-0626 */         let mut err = match error_code {
/* FP:need_type_info.rs-0627 */             TypeAnnotationNeeded::E0282 => self.dcx().create_err(AnnotationRequired {
/* FP:need_type_info.rs-0628 */                 span,
/* FP:need_type_info.rs-0629 */                 source_kind,
/* FP:need_type_info.rs-0630 */                 source_name: &name,
/* FP:need_type_info.rs-0631 */                 failure_span,
/* FP:need_type_info.rs-0632 */                 infer_subdiags,
/* FP:need_type_info.rs-0633 */                 multi_suggestions,
/* FP:need_type_info.rs-0634 */                 bad_label: None,
/* FP:need_type_info.rs-0635 */             }),
/* FP:need_type_info.rs-0636 */             TypeAnnotationNeeded::E0283 => self.dcx().create_err(AmbiguousImpl {
/* FP:need_type_info.rs-0637 */                 span,
/* FP:need_type_info.rs-0638 */                 source_kind,
/* FP:need_type_info.rs-0639 */                 source_name: &name,
/* FP:need_type_info.rs-0640 */                 failure_span,
/* FP:need_type_info.rs-0641 */                 infer_subdiags,
/* FP:need_type_info.rs-0642 */                 multi_suggestions,
/* FP:need_type_info.rs-0643 */                 bad_label: None,
/* FP:need_type_info.rs-0644 */             }),
/* FP:need_type_info.rs-0645 */             TypeAnnotationNeeded::E0284 => self.dcx().create_err(AmbiguousReturn {
/* FP:need_type_info.rs-0646 */                 span,
/* FP:need_type_info.rs-0647 */                 source_kind,
/* FP:need_type_info.rs-0648 */                 source_name: &name,
/* FP:need_type_info.rs-0649 */                 failure_span,
/* FP:need_type_info.rs-0650 */                 infer_subdiags,
/* FP:need_type_info.rs-0651 */                 multi_suggestions,
/* FP:need_type_info.rs-0652 */                 bad_label: None,
/* FP:need_type_info.rs-0653 */             }),
/* FP:need_type_info.rs-0654 */         };
/* FP:need_type_info.rs-0655 */         *err.long_ty_path() = long_ty_path;
/* FP:need_type_info.rs-0656 */         err
/* FP:need_type_info.rs-0657 */     }
/* FP:need_type_info.rs-0658 */ }
/* FP:need_type_info.rs-0659 */ 
/* FP:need_type_info.rs-0660 */ #[derive(Debug)]
/* FP:need_type_info.rs-0661 */ struct InferSource<'tcx> {
/* FP:need_type_info.rs-0662 */     span: Span,
/* FP:need_type_info.rs-0663 */     kind: InferSourceKind<'tcx>,
/* FP:need_type_info.rs-0664 */ }
/* FP:need_type_info.rs-0665 */ 
/* FP:need_type_info.rs-0666 */ #[derive(Debug)]
/* FP:need_type_info.rs-0667 */ enum InferSourceKind<'tcx> {
/* FP:need_type_info.rs-0668 */     LetBinding {
/* FP:need_type_info.rs-0669 */         insert_span: Span,
/* FP:need_type_info.rs-0670 */         pattern_name: Option<Ident>,
/* FP:need_type_info.rs-0671 */         ty: Ty<'tcx>,
/* FP:need_type_info.rs-0672 */         def_id: Option<DefId>,
/* FP:need_type_info.rs-0673 */     },
/* FP:need_type_info.rs-0674 */     ClosureArg {
/* FP:need_type_info.rs-0675 */         insert_span: Span,
/* FP:need_type_info.rs-0676 */         ty: Ty<'tcx>,
/* FP:need_type_info.rs-0677 */     },
/* FP:need_type_info.rs-0678 */     GenericArg {
/* FP:need_type_info.rs-0679 */         insert_span: Span,
/* FP:need_type_info.rs-0680 */         argument_index: usize,
/* FP:need_type_info.rs-0681 */         generics_def_id: DefId,
/* FP:need_type_info.rs-0682 */         def_id: DefId,
/* FP:need_type_info.rs-0683 */         generic_args: &'tcx [GenericArg<'tcx>],
/* FP:need_type_info.rs-0684 */         have_turbofish: bool,
/* FP:need_type_info.rs-0685 */     },
/* FP:need_type_info.rs-0686 */     FullyQualifiedMethodCall {
/* FP:need_type_info.rs-0687 */         receiver: &'tcx Expr<'tcx>,
/* FP:need_type_info.rs-0688 */         /// If the method has other arguments, this is ", " and the start of the first argument,
/* FP:need_type_info.rs-0689 */         /// while for methods without arguments this is ")" and the end of the method call.
/* FP:need_type_info.rs-0690 */         successor: (&'static str, BytePos),
/* FP:need_type_info.rs-0691 */         args: GenericArgsRef<'tcx>,
/* FP:need_type_info.rs-0692 */         def_id: DefId,
/* FP:need_type_info.rs-0693 */     },
/* FP:need_type_info.rs-0694 */     ClosureReturn {
/* FP:need_type_info.rs-0695 */         ty: Ty<'tcx>,
/* FP:need_type_info.rs-0696 */         data: &'tcx FnRetTy<'tcx>,
/* FP:need_type_info.rs-0697 */         should_wrap_expr: Option<Span>,
/* FP:need_type_info.rs-0698 */     },
/* FP:need_type_info.rs-0699 */ }
/* FP:need_type_info.rs-0700 */ 
/* FP:need_type_info.rs-0701 */ impl<'tcx> InferSource<'tcx> {
/* FP:need_type_info.rs-0702 */     fn from_expansion(&self) -> bool {
/* FP:need_type_info.rs-0703 */         let source_from_expansion = match self.kind {
/* FP:need_type_info.rs-0704 */             InferSourceKind::LetBinding { insert_span, .. }
/* FP:need_type_info.rs-0705 */             | InferSourceKind::ClosureArg { insert_span, .. }
/* FP:need_type_info.rs-0706 */             | InferSourceKind::GenericArg { insert_span, .. } => insert_span.from_expansion(),
/* FP:need_type_info.rs-0707 */             InferSourceKind::FullyQualifiedMethodCall { receiver, .. } => {
/* FP:need_type_info.rs-0708 */                 receiver.span.from_expansion()
/* FP:need_type_info.rs-0709 */             }
/* FP:need_type_info.rs-0710 */             InferSourceKind::ClosureReturn { data, should_wrap_expr, .. } => {
/* FP:need_type_info.rs-0711 */                 data.span().from_expansion() || should_wrap_expr.is_some_and(Span::from_expansion)
/* FP:need_type_info.rs-0712 */             }
/* FP:need_type_info.rs-0713 */         };
/* FP:need_type_info.rs-0714 */         source_from_expansion || self.span.from_expansion()
/* FP:need_type_info.rs-0715 */     }
/* FP:need_type_info.rs-0716 */ }
/* FP:need_type_info.rs-0717 */ 
/* FP:need_type_info.rs-0718 */ impl<'tcx> InferSourceKind<'tcx> {
/* FP:need_type_info.rs-0719 */     fn ty_localized_msg(&self, infcx: &InferCtxt<'tcx>) -> (&'static str, String, Option<PathBuf>) {
/* FP:need_type_info.rs-0720 */         let mut long_ty_path = None;
/* FP:need_type_info.rs-0721 */         match *self {
/* FP:need_type_info.rs-0722 */             InferSourceKind::LetBinding { ty, .. }
/* FP:need_type_info.rs-0723 */             | InferSourceKind::ClosureArg { ty, .. }
/* FP:need_type_info.rs-0724 */             | InferSourceKind::ClosureReturn { ty, .. } => {
/* FP:need_type_info.rs-0725 */                 if ty.is_closure() {
/* FP:need_type_info.rs-0726 */                     ("closure", closure_as_fn_str(infcx, ty), long_ty_path)
/* FP:need_type_info.rs-0727 */                 } else if !ty.is_ty_or_numeric_infer() {
/* FP:need_type_info.rs-0728 */                     ("normal", infcx.tcx.short_string(ty, &mut long_ty_path), long_ty_path)
/* FP:need_type_info.rs-0729 */                 } else {
/* FP:need_type_info.rs-0730 */                     ("other", String::new(), long_ty_path)
/* FP:need_type_info.rs-0731 */                 }
/* FP:need_type_info.rs-0732 */             }
/* FP:need_type_info.rs-0733 */             // FIXME: We should be able to add some additional info here.
/* FP:need_type_info.rs-0734 */             InferSourceKind::GenericArg { .. }
/* FP:need_type_info.rs-0735 */             | InferSourceKind::FullyQualifiedMethodCall { .. } => {
/* FP:need_type_info.rs-0736 */                 ("other", String::new(), long_ty_path)
/* FP:need_type_info.rs-0737 */             }
/* FP:need_type_info.rs-0738 */         }
/* FP:need_type_info.rs-0739 */     }
/* FP:need_type_info.rs-0740 */ }
/* FP:need_type_info.rs-0741 */ 
/* FP:need_type_info.rs-0742 */ #[derive(Debug)]
/* FP:need_type_info.rs-0743 */ struct InsertableGenericArgs<'tcx> {
/* FP:need_type_info.rs-0744 */     insert_span: Span,
/* FP:need_type_info.rs-0745 */     args: GenericArgsRef<'tcx>,
/* FP:need_type_info.rs-0746 */     generics_def_id: DefId,
/* FP:need_type_info.rs-0747 */     def_id: DefId,
/* FP:need_type_info.rs-0748 */     have_turbofish: bool,
/* FP:need_type_info.rs-0749 */ }
/* FP:need_type_info.rs-0750 */ 
/* FP:need_type_info.rs-0751 */ /// A visitor which searches for the "best" spot to use in the inference error.
/* FP:need_type_info.rs-0752 */ ///
/* FP:need_type_info.rs-0753 */ /// For this it walks over the hir body and tries to check all places where
/* FP:need_type_info.rs-0754 */ /// inference variables could be bound.
/* FP:need_type_info.rs-0755 */ ///
/* FP:need_type_info.rs-0756 */ /// While doing so, the currently best spot is stored in `infer_source`.
/* FP:need_type_info.rs-0757 */ /// For details on how we rank spots, see [Self::source_cost]
/* FP:need_type_info.rs-0758 */ struct FindInferSourceVisitor<'a, 'tcx> {
/* FP:need_type_info.rs-0759 */     tecx: &'a TypeErrCtxt<'a, 'tcx>,
/* FP:need_type_info.rs-0760 */     typeck_results: &'a TypeckResults<'tcx>,
/* FP:need_type_info.rs-0761 */ 
/* FP:need_type_info.rs-0762 */     target: Term<'tcx>,
/* FP:need_type_info.rs-0763 */ 
/* FP:need_type_info.rs-0764 */     attempt: usize,
/* FP:need_type_info.rs-0765 */     infer_source_cost: usize,
/* FP:need_type_info.rs-0766 */     infer_source: Option<InferSource<'tcx>>,
/* FP:need_type_info.rs-0767 */ }
/* FP:need_type_info.rs-0768 */ 
/* FP:need_type_info.rs-0769 */ impl<'a, 'tcx> FindInferSourceVisitor<'a, 'tcx> {
/* FP:need_type_info.rs-0770 */     fn new(
/* FP:need_type_info.rs-0771 */         tecx: &'a TypeErrCtxt<'a, 'tcx>,
/* FP:need_type_info.rs-0772 */         typeck_results: &'a TypeckResults<'tcx>,
/* FP:need_type_info.rs-0773 */         target: Term<'tcx>,
/* FP:need_type_info.rs-0774 */     ) -> Self {
/* FP:need_type_info.rs-0775 */         FindInferSourceVisitor {
/* FP:need_type_info.rs-0776 */             tecx,
/* FP:need_type_info.rs-0777 */             typeck_results,
/* FP:need_type_info.rs-0778 */ 
/* FP:need_type_info.rs-0779 */             target,
/* FP:need_type_info.rs-0780 */ 
/* FP:need_type_info.rs-0781 */             attempt: 0,
/* FP:need_type_info.rs-0782 */             infer_source_cost: usize::MAX,
/* FP:need_type_info.rs-0783 */             infer_source: None,
/* FP:need_type_info.rs-0784 */         }
/* FP:need_type_info.rs-0785 */     }
/* FP:need_type_info.rs-0786 */ 
/* FP:need_type_info.rs-0787 */     /// Computes cost for the given source.
/* FP:need_type_info.rs-0788 */     ///
/* FP:need_type_info.rs-0789 */     /// Sources with a small cost are prefer and should result
/* FP:need_type_info.rs-0790 */     /// in a clearer and idiomatic suggestion.
/* FP:need_type_info.rs-0791 */     fn source_cost(&self, source: &InferSource<'tcx>) -> usize {
/* FP:need_type_info.rs-0792 */         #[derive(Clone, Copy)]
/* FP:need_type_info.rs-0793 */         struct CostCtxt<'tcx> {
/* FP:need_type_info.rs-0794 */             tcx: TyCtxt<'tcx>,
/* FP:need_type_info.rs-0795 */         }
/* FP:need_type_info.rs-0796 */         impl<'tcx> CostCtxt<'tcx> {
/* FP:need_type_info.rs-0797 */             fn arg_cost(self, arg: GenericArg<'tcx>) -> usize {
/* FP:need_type_info.rs-0798 */                 match arg.kind() {
/* FP:need_type_info.rs-0799 */                     GenericArgKind::Lifetime(_) => 0, // erased
/* FP:need_type_info.rs-0800 */                     GenericArgKind::Type(ty) => self.ty_cost(ty),
/* FP:need_type_info.rs-0801 */                     GenericArgKind::Const(_) => 3, // some non-zero value
/* FP:need_type_info.rs-0802 */                 }
/* FP:need_type_info.rs-0803 */             }
/* FP:need_type_info.rs-0804 */             fn ty_cost(self, ty: Ty<'tcx>) -> usize {
/* FP:need_type_info.rs-0805 */                 match *ty.kind() {
/* FP:need_type_info.rs-0806 */                     ty::Closure(..) => 1000,
/* FP:need_type_info.rs-0807 */                     ty::FnDef(..) => 150,
/* FP:need_type_info.rs-0808 */                     ty::FnPtr(..) => 30,
/* FP:need_type_info.rs-0809 */                     ty::Adt(def, args) => {
/* FP:need_type_info.rs-0810 */                         5 + self
/* FP:need_type_info.rs-0811 */                             .tcx
/* FP:need_type_info.rs-0812 */                             .generics_of(def.did())
/* FP:need_type_info.rs-0813 */                             .own_args_no_defaults(self.tcx, args)
/* FP:need_type_info.rs-0814 */                             .iter()
/* FP:need_type_info.rs-0815 */                             .map(|&arg| self.arg_cost(arg))
/* FP:need_type_info.rs-0816 */                             .sum::<usize>()
/* FP:need_type_info.rs-0817 */                     }
/* FP:need_type_info.rs-0818 */                     ty::Tuple(args) => 5 + args.iter().map(|arg| self.ty_cost(arg)).sum::<usize>(),
/* FP:need_type_info.rs-0819 */                     ty::Ref(_, ty, _) => 2 + self.ty_cost(ty),
/* FP:need_type_info.rs-0820 */                     ty::Infer(..) => 0,
/* FP:need_type_info.rs-0821 */                     _ => 1,
/* FP:need_type_info.rs-0822 */                 }
/* FP:need_type_info.rs-0823 */             }
/* FP:need_type_info.rs-0824 */         }
/* FP:need_type_info.rs-0825 */ 
/* FP:need_type_info.rs-0826 */         // The sources are listed in order of preference here.
/* FP:need_type_info.rs-0827 */         let tcx = self.tecx.tcx;
/* FP:need_type_info.rs-0828 */         let ctx = CostCtxt { tcx };
/* FP:need_type_info.rs-0829 */         match source.kind {
/* FP:need_type_info.rs-0830 */             InferSourceKind::LetBinding { ty, .. } => ctx.ty_cost(ty),
/* FP:need_type_info.rs-0831 */             InferSourceKind::ClosureArg { ty, .. } => ctx.ty_cost(ty),
/* FP:need_type_info.rs-0832 */             InferSourceKind::GenericArg { def_id, generic_args, .. } => {
/* FP:need_type_info.rs-0833 */                 let variant_cost = match tcx.def_kind(def_id) {
/* FP:need_type_info.rs-0834 */                     // `None::<u32>` and friends are ugly.
/* FP:need_type_info.rs-0835 */                     DefKind::Variant | DefKind::Ctor(CtorOf::Variant, _) => 15,
/* FP:need_type_info.rs-0836 */                     _ => 10,
/* FP:need_type_info.rs-0837 */                 };
/* FP:need_type_info.rs-0838 */                 variant_cost + generic_args.iter().map(|&arg| ctx.arg_cost(arg)).sum::<usize>()
/* FP:need_type_info.rs-0839 */             }
/* FP:need_type_info.rs-0840 */             InferSourceKind::FullyQualifiedMethodCall { args, .. } => {
/* FP:need_type_info.rs-0841 */                 20 + args.iter().map(|arg| ctx.arg_cost(arg)).sum::<usize>()
/* FP:need_type_info.rs-0842 */             }
/* FP:need_type_info.rs-0843 */             InferSourceKind::ClosureReturn { ty, should_wrap_expr, .. } => {
/* FP:need_type_info.rs-0844 */                 30 + ctx.ty_cost(ty) + if should_wrap_expr.is_some() { 10 } else { 0 }
/* FP:need_type_info.rs-0845 */             }
/* FP:need_type_info.rs-0846 */         }
/* FP:need_type_info.rs-0847 */     }
/* FP:need_type_info.rs-0848 */ 
/* FP:need_type_info.rs-0849 */     /// Uses `fn source_cost` to determine whether this inference source is preferable to
/* FP:need_type_info.rs-0850 */     /// previous sources. We generally prefer earlier sources.
/* FP:need_type_info.rs-0851 */     #[instrument(level = "debug", skip(self))]
/* FP:need_type_info.rs-0852 */     fn update_infer_source(&mut self, mut new_source: InferSource<'tcx>) {
/* FP:need_type_info.rs-0853 */         if new_source.from_expansion() {
/* FP:need_type_info.rs-0854 */             return;
/* FP:need_type_info.rs-0855 */         }
/* FP:need_type_info.rs-0856 */ 
/* FP:need_type_info.rs-0857 */         let cost = self.source_cost(&new_source) + self.attempt;
/* FP:need_type_info.rs-0858 */         debug!(?cost);
/* FP:need_type_info.rs-0859 */         self.attempt += 1;
/* FP:need_type_info.rs-0860 */         if let Some(InferSource { kind: InferSourceKind::GenericArg { def_id: did, .. }, .. }) =
/* FP:need_type_info.rs-0861 */             self.infer_source
/* FP:need_type_info.rs-0862 */             && let InferSourceKind::LetBinding { ref ty, ref mut def_id, .. } = new_source.kind
/* FP:need_type_info.rs-0863 */             && ty.is_ty_or_numeric_infer()
/* FP:need_type_info.rs-0864 */         {
/* FP:need_type_info.rs-0865 */             // Customize the output so we talk about `let x: Vec<_> = iter.collect();` instead of
/* FP:need_type_info.rs-0866 */             // `let x: _ = iter.collect();`, as this is a very common case.
/* FP:need_type_info.rs-0867 */             *def_id = Some(did);
/* FP:need_type_info.rs-0868 */         }
/* FP:need_type_info.rs-0869 */ 
/* FP:need_type_info.rs-0870 */         if cost < self.infer_source_cost {
/* FP:need_type_info.rs-0871 */             self.infer_source_cost = cost;
/* FP:need_type_info.rs-0872 */             self.infer_source = Some(new_source);
/* FP:need_type_info.rs-0873 */         }
/* FP:need_type_info.rs-0874 */     }
/* FP:need_type_info.rs-0875 */ 
/* FP:need_type_info.rs-0876 */     fn node_args_opt(&self, hir_id: HirId) -> Option<GenericArgsRef<'tcx>> {
/* FP:need_type_info.rs-0877 */         let args = self.typeck_results.node_args_opt(hir_id);
/* FP:need_type_info.rs-0878 */         self.tecx.resolve_vars_if_possible(args)
/* FP:need_type_info.rs-0879 */     }
/* FP:need_type_info.rs-0880 */ 
/* FP:need_type_info.rs-0881 */     fn opt_node_type(&self, hir_id: HirId) -> Option<Ty<'tcx>> {
/* FP:need_type_info.rs-0882 */         let ty = self.typeck_results.node_type_opt(hir_id);
/* FP:need_type_info.rs-0883 */         self.tecx.resolve_vars_if_possible(ty)
/* FP:need_type_info.rs-0884 */     }
/* FP:need_type_info.rs-0885 */ 
/* FP:need_type_info.rs-0886 */     // Check whether this generic argument is the inference variable we
/* FP:need_type_info.rs-0887 */     // are looking for.
/* FP:need_type_info.rs-0888 */     fn generic_arg_is_target(&self, arg: GenericArg<'tcx>) -> bool {
/* FP:need_type_info.rs-0889 */         if arg == self.target.into() {
/* FP:need_type_info.rs-0890 */             return true;
/* FP:need_type_info.rs-0891 */         }
/* FP:need_type_info.rs-0892 */ 
/* FP:need_type_info.rs-0893 */         match (arg.kind(), self.target.kind()) {
/* FP:need_type_info.rs-0894 */             (GenericArgKind::Type(inner_ty), TermKind::Ty(target_ty)) => {
/* FP:need_type_info.rs-0895 */                 use ty::{Infer, TyVar};
/* FP:need_type_info.rs-0896 */                 match (inner_ty.kind(), target_ty.kind()) {
/* FP:need_type_info.rs-0897 */                     (&Infer(TyVar(a_vid)), &Infer(TyVar(b_vid))) => {
/* FP:need_type_info.rs-0898 */                         self.tecx.sub_unification_table_root_var(a_vid)
/* FP:need_type_info.rs-0899 */                             == self.tecx.sub_unification_table_root_var(b_vid)
/* FP:need_type_info.rs-0900 */                     }
/* FP:need_type_info.rs-0901 */                     _ => false,
/* FP:need_type_info.rs-0902 */                 }
/* FP:need_type_info.rs-0903 */             }
/* FP:need_type_info.rs-0904 */             (GenericArgKind::Const(inner_ct), TermKind::Const(target_ct)) => {
/* FP:need_type_info.rs-0905 */                 match (inner_ct.kind(), target_ct.kind()) {
/* FP:need_type_info.rs-0906 */                     (
/* FP:need_type_info.rs-0907 */                         ty::ConstKind::Infer(ty::InferConst::Var(a_vid)),
/* FP:need_type_info.rs-0908 */                         ty::ConstKind::Infer(ty::InferConst::Var(b_vid)),
/* FP:need_type_info.rs-0909 */                     ) => self.tecx.root_const_var(a_vid) == self.tecx.root_const_var(b_vid),
/* FP:need_type_info.rs-0910 */                     _ => false,
/* FP:need_type_info.rs-0911 */                 }
/* FP:need_type_info.rs-0912 */             }
/* FP:need_type_info.rs-0913 */             _ => false,
/* FP:need_type_info.rs-0914 */         }
/* FP:need_type_info.rs-0915 */     }
/* FP:need_type_info.rs-0916 */ 
/* FP:need_type_info.rs-0917 */     /// Does this generic argument contain our target inference variable
/* FP:need_type_info.rs-0918 */     /// in a way which can be written by the user.
/* FP:need_type_info.rs-0919 */     fn generic_arg_contains_target(&self, arg: GenericArg<'tcx>) -> bool {
/* FP:need_type_info.rs-0920 */         let mut walker = arg.walk();
/* FP:need_type_info.rs-0921 */         while let Some(inner) = walker.next() {
/* FP:need_type_info.rs-0922 */             if self.generic_arg_is_target(inner) {
/* FP:need_type_info.rs-0923 */                 return true;
/* FP:need_type_info.rs-0924 */             }
/* FP:need_type_info.rs-0925 */             match inner.kind() {
/* FP:need_type_info.rs-0926 */                 GenericArgKind::Lifetime(_) => {}
/* FP:need_type_info.rs-0927 */                 GenericArgKind::Type(ty) => {
/* FP:need_type_info.rs-0928 */                     if matches!(
/* FP:need_type_info.rs-0929 */                         ty.kind(),
/* FP:need_type_info.rs-0930 */                         ty::Alias(ty::Opaque, ..)
/* FP:need_type_info.rs-0931 */                             | ty::Closure(..)
/* FP:need_type_info.rs-0932 */                             | ty::CoroutineClosure(..)
/* FP:need_type_info.rs-0933 */                             | ty::Coroutine(..)
/* FP:need_type_info.rs-0934 */                     ) {
/* FP:need_type_info.rs-0935 */                         // Opaque types can't be named by the user right now.
/* FP:need_type_info.rs-0936 */                         //
/* FP:need_type_info.rs-0937 */                         // Both the generic arguments of closures and coroutines can
/* FP:need_type_info.rs-0938 */                         // also not be named. We may want to only look into the closure
/* FP:need_type_info.rs-0939 */                         // signature in case it has no captures, as that can be represented
/* FP:need_type_info.rs-0940 */                         // using `fn(T) -> R`.
/* FP:need_type_info.rs-0941 */ 
/* FP:need_type_info.rs-0942 */                         // FIXME(type_alias_impl_trait): These opaque types
/* FP:need_type_info.rs-0943 */                         // can actually be named, so it would make sense to
/* FP:need_type_info.rs-0944 */                         // adjust this case and add a test for it.
/* FP:need_type_info.rs-0945 */                         walker.skip_current_subtree();
/* FP:need_type_info.rs-0946 */                     }
/* FP:need_type_info.rs-0947 */                 }
/* FP:need_type_info.rs-0948 */                 GenericArgKind::Const(ct) => {
/* FP:need_type_info.rs-0949 */                     if matches!(ct.kind(), ty::ConstKind::Unevaluated(..)) {
/* FP:need_type_info.rs-0950 */                         // You can't write the generic arguments for
/* FP:need_type_info.rs-0951 */                         // unevaluated constants.
/* FP:need_type_info.rs-0952 */                         walker.skip_current_subtree();
/* FP:need_type_info.rs-0953 */                     }
/* FP:need_type_info.rs-0954 */                 }
/* FP:need_type_info.rs-0955 */             }
/* FP:need_type_info.rs-0956 */         }
/* FP:need_type_info.rs-0957 */         false
/* FP:need_type_info.rs-0958 */     }
/* FP:need_type_info.rs-0959 */ 
/* FP:need_type_info.rs-0960 */     fn expr_inferred_arg_iter(
/* FP:need_type_info.rs-0961 */         &self,
/* FP:need_type_info.rs-0962 */         expr: &'tcx hir::Expr<'tcx>,
/* FP:need_type_info.rs-0963 */     ) -> Box<dyn Iterator<Item = InsertableGenericArgs<'tcx>> + 'a> {
/* FP:need_type_info.rs-0964 */         let tcx = self.tecx.tcx;
/* FP:need_type_info.rs-0965 */         match expr.kind {
/* FP:need_type_info.rs-0966 */             hir::ExprKind::Path(ref path) => {
/* FP:need_type_info.rs-0967 */                 if let Some(args) = self.node_args_opt(expr.hir_id) {
/* FP:need_type_info.rs-0968 */                     return self.path_inferred_arg_iter(expr.hir_id, args, path);
/* FP:need_type_info.rs-0969 */                 }
/* FP:need_type_info.rs-0970 */             }
/* FP:need_type_info.rs-0971 */             // FIXME(#98711): Ideally we would also deal with type relative
/* FP:need_type_info.rs-0972 */             // paths here, even if that is quite rare.
/* FP:need_type_info.rs-0973 */             //
/* FP:need_type_info.rs-0974 */             // See the `need_type_info/expr-struct-type-relative-gat.rs` test
/* FP:need_type_info.rs-0975 */             // for an example where that would be needed.
/* FP:need_type_info.rs-0976 */             //
/* FP:need_type_info.rs-0977 */             // However, the `type_dependent_def_id` for `Self::Output` in an
/* FP:need_type_info.rs-0978 */             // impl is currently the `DefId` of `Output` in the trait definition
/* FP:need_type_info.rs-0979 */             // which makes this somewhat difficult and prevents us from just
/* FP:need_type_info.rs-0980 */             // using `self.path_inferred_arg_iter` here.
/* FP:need_type_info.rs-0981 */             hir::ExprKind::Struct(&hir::QPath::Resolved(_self_ty, path), _, _)
/* FP:need_type_info.rs-0982 */             // FIXME(TaKO8Ki): Ideally we should support other kinds,
/* FP:need_type_info.rs-0983 */             // such as `TyAlias` or `AssocTy`. For that we have to map
/* FP:need_type_info.rs-0984 */             // back from the self type to the type alias though. That's difficult.
/* FP:need_type_info.rs-0985 */             //
/* FP:need_type_info.rs-0986 */             // See the `need_type_info/issue-103053.rs` test for
/* FP:need_type_info.rs-0987 */             // a example.
/* FP:need_type_info.rs-0988 */             if matches!(path.res, Res::Def(DefKind::Struct | DefKind::Enum | DefKind::Union, _)) => {
/* FP:need_type_info.rs-0989 */                 if let Some(ty) = self.opt_node_type(expr.hir_id)
/* FP:need_type_info.rs-0990 */                     && let ty::Adt(_, args) = ty.kind()
/* FP:need_type_info.rs-0991 */                 {
/* FP:need_type_info.rs-0992 */                     return Box::new(self.resolved_path_inferred_arg_iter(path, args));
/* FP:need_type_info.rs-0993 */                 }
/* FP:need_type_info.rs-0994 */             }
/* FP:need_type_info.rs-0995 */             hir::ExprKind::MethodCall(segment, ..) => {
/* FP:need_type_info.rs-0996 */                 if let Some(def_id) = self.typeck_results.type_dependent_def_id(expr.hir_id) {
/* FP:need_type_info.rs-0997 */                     let generics = tcx.generics_of(def_id);
/* FP:need_type_info.rs-0998 */                     let insertable: Option<_> = try {
/* FP:need_type_info.rs-0999 */                         if generics.has_impl_trait() {
/* FP:need_type_info.rs-1000 */                             None?
/* FP:need_type_info.rs-1001 */                         }
/* FP:need_type_info.rs-1002 */                         let args = self.node_args_opt(expr.hir_id)?;
/* FP:need_type_info.rs-1003 */                         let span = tcx.hir_span(segment.hir_id);
/* FP:need_type_info.rs-1004 */                         let insert_span = segment.ident.span.shrink_to_hi().with_hi(span.hi());
/* FP:need_type_info.rs-1005 */                         InsertableGenericArgs {
/* FP:need_type_info.rs-1006 */                             insert_span,
/* FP:need_type_info.rs-1007 */                             args,
/* FP:need_type_info.rs-1008 */                             generics_def_id: def_id,
/* FP:need_type_info.rs-1009 */                             def_id,
/* FP:need_type_info.rs-1010 */                             have_turbofish: false,
/* FP:need_type_info.rs-1011 */                         }
/* FP:need_type_info.rs-1012 */                     };
/* FP:need_type_info.rs-1013 */                     return Box::new(insertable.into_iter());
/* FP:need_type_info.rs-1014 */                 }
/* FP:need_type_info.rs-1015 */             }
/* FP:need_type_info.rs-1016 */             _ => {}
/* FP:need_type_info.rs-1017 */         }
/* FP:need_type_info.rs-1018 */ 
/* FP:need_type_info.rs-1019 */         Box::new(iter::empty())
/* FP:need_type_info.rs-1020 */     }
/* FP:need_type_info.rs-1021 */ 
/* FP:need_type_info.rs-1022 */     fn resolved_path_inferred_arg_iter(
/* FP:need_type_info.rs-1023 */         &self,
/* FP:need_type_info.rs-1024 */         path: &'tcx hir::Path<'tcx>,
/* FP:need_type_info.rs-1025 */         args: GenericArgsRef<'tcx>,
/* FP:need_type_info.rs-1026 */     ) -> impl Iterator<Item = InsertableGenericArgs<'tcx>> + 'tcx {
/* FP:need_type_info.rs-1027 */         let tcx = self.tecx.tcx;
/* FP:need_type_info.rs-1028 */         let have_turbofish = path.segments.iter().any(|segment| {
/* FP:need_type_info.rs-1029 */             segment.args.is_some_and(|args| args.args.iter().any(|arg| arg.is_ty_or_const()))
/* FP:need_type_info.rs-1030 */         });
/* FP:need_type_info.rs-1031 */         // The last segment of a path often has `Res::Err` and the
/* FP:need_type_info.rs-1032 */         // correct `Res` is the one of the whole path.
/* FP:need_type_info.rs-1033 */         //
/* FP:need_type_info.rs-1034 */         // FIXME: We deal with that one separately for now,
/* FP:need_type_info.rs-1035 */         // would be good to remove this special case.
/* FP:need_type_info.rs-1036 */         let last_segment_using_path_data: Option<_> = try {
/* FP:need_type_info.rs-1037 */             let generics_def_id = tcx.res_generics_def_id(path.res)?;
/* FP:need_type_info.rs-1038 */             let generics = tcx.generics_of(generics_def_id);
/* FP:need_type_info.rs-1039 */             if generics.has_impl_trait() {
/* FP:need_type_info.rs-1040 */                 do yeet ();
/* FP:need_type_info.rs-1041 */             }
/* FP:need_type_info.rs-1042 */             let insert_span =
/* FP:need_type_info.rs-1043 */                 path.segments.last().unwrap().ident.span.shrink_to_hi().with_hi(path.span.hi());
/* FP:need_type_info.rs-1044 */             InsertableGenericArgs {
/* FP:need_type_info.rs-1045 */                 insert_span,
/* FP:need_type_info.rs-1046 */                 args,
/* FP:need_type_info.rs-1047 */                 generics_def_id,
/* FP:need_type_info.rs-1048 */                 def_id: path.res.def_id(),
/* FP:need_type_info.rs-1049 */                 have_turbofish,
/* FP:need_type_info.rs-1050 */             }
/* FP:need_type_info.rs-1051 */         };
/* FP:need_type_info.rs-1052 */ 
/* FP:need_type_info.rs-1053 */         path.segments
/* FP:need_type_info.rs-1054 */             .iter()
/* FP:need_type_info.rs-1055 */             .filter_map(move |segment| {
/* FP:need_type_info.rs-1056 */                 let res = segment.res;
/* FP:need_type_info.rs-1057 */                 let generics_def_id = tcx.res_generics_def_id(res)?;
/* FP:need_type_info.rs-1058 */                 let generics = tcx.generics_of(generics_def_id);
/* FP:need_type_info.rs-1059 */                 if generics.has_impl_trait() {
/* FP:need_type_info.rs-1060 */                     return None;
/* FP:need_type_info.rs-1061 */                 }
/* FP:need_type_info.rs-1062 */                 let span = tcx.hir_span(segment.hir_id);
/* FP:need_type_info.rs-1063 */                 let insert_span = segment.ident.span.shrink_to_hi().with_hi(span.hi());
/* FP:need_type_info.rs-1064 */                 Some(InsertableGenericArgs {
/* FP:need_type_info.rs-1065 */                     insert_span,
/* FP:need_type_info.rs-1066 */                     args,
/* FP:need_type_info.rs-1067 */                     generics_def_id,
/* FP:need_type_info.rs-1068 */                     def_id: res.def_id(),
/* FP:need_type_info.rs-1069 */                     have_turbofish,
/* FP:need_type_info.rs-1070 */                 })
/* FP:need_type_info.rs-1071 */             })
/* FP:need_type_info.rs-1072 */             .chain(last_segment_using_path_data)
/* FP:need_type_info.rs-1073 */     }
/* FP:need_type_info.rs-1074 */ 
/* FP:need_type_info.rs-1075 */     fn path_inferred_arg_iter(
/* FP:need_type_info.rs-1076 */         &self,
/* FP:need_type_info.rs-1077 */         hir_id: HirId,
/* FP:need_type_info.rs-1078 */         args: GenericArgsRef<'tcx>,
/* FP:need_type_info.rs-1079 */         qpath: &'tcx hir::QPath<'tcx>,
/* FP:need_type_info.rs-1080 */     ) -> Box<dyn Iterator<Item = InsertableGenericArgs<'tcx>> + 'a> {
/* FP:need_type_info.rs-1081 */         let tcx = self.tecx.tcx;
/* FP:need_type_info.rs-1082 */         match qpath {
/* FP:need_type_info.rs-1083 */             hir::QPath::Resolved(_self_ty, path) => {
/* FP:need_type_info.rs-1084 */                 Box::new(self.resolved_path_inferred_arg_iter(path, args))
/* FP:need_type_info.rs-1085 */             }
/* FP:need_type_info.rs-1086 */             hir::QPath::TypeRelative(ty, segment) => {
/* FP:need_type_info.rs-1087 */                 let Some(def_id) = self.typeck_results.type_dependent_def_id(hir_id) else {
/* FP:need_type_info.rs-1088 */                     return Box::new(iter::empty());
/* FP:need_type_info.rs-1089 */                 };
/* FP:need_type_info.rs-1090 */ 
/* FP:need_type_info.rs-1091 */                 let generics = tcx.generics_of(def_id);
/* FP:need_type_info.rs-1092 */                 let segment: Option<_> = try {
/* FP:need_type_info.rs-1093 */                     if !segment.infer_args || generics.has_impl_trait() {
/* FP:need_type_info.rs-1094 */                         do yeet ();
/* FP:need_type_info.rs-1095 */                     }
/* FP:need_type_info.rs-1096 */                     let span = tcx.hir_span(segment.hir_id);
/* FP:need_type_info.rs-1097 */                     let insert_span = segment.ident.span.shrink_to_hi().with_hi(span.hi());
/* FP:need_type_info.rs-1098 */                     InsertableGenericArgs {
/* FP:need_type_info.rs-1099 */                         insert_span,
/* FP:need_type_info.rs-1100 */                         args,
/* FP:need_type_info.rs-1101 */                         generics_def_id: def_id,
/* FP:need_type_info.rs-1102 */                         def_id,
/* FP:need_type_info.rs-1103 */                         have_turbofish: false,
/* FP:need_type_info.rs-1104 */                     }
/* FP:need_type_info.rs-1105 */                 };
/* FP:need_type_info.rs-1106 */ 
/* FP:need_type_info.rs-1107 */                 let parent_def_id = generics.parent.unwrap();
/* FP:need_type_info.rs-1108 */                 if let DefKind::Impl { .. } = tcx.def_kind(parent_def_id) {
/* FP:need_type_info.rs-1109 */                     let parent_ty = tcx.type_of(parent_def_id).instantiate(tcx, args);
/* FP:need_type_info.rs-1110 */                     match (parent_ty.kind(), &ty.kind) {
/* FP:need_type_info.rs-1111 */                         (
/* FP:need_type_info.rs-1112 */                             ty::Adt(def, args),
/* FP:need_type_info.rs-1113 */                             hir::TyKind::Path(hir::QPath::Resolved(_self_ty, path)),
/* FP:need_type_info.rs-1114 */                         ) => {
/* FP:need_type_info.rs-1115 */                             if tcx.res_generics_def_id(path.res) != Some(def.did()) {
/* FP:need_type_info.rs-1116 */                                 match path.res {
/* FP:need_type_info.rs-1117 */                                     Res::Def(DefKind::TyAlias, _) => {
/* FP:need_type_info.rs-1118 */                                         // FIXME: Ideally we should support this. For that
/* FP:need_type_info.rs-1119 */                                         // we have to map back from the self type to the
/* FP:need_type_info.rs-1120 */                                         // type alias though. That's difficult.
/* FP:need_type_info.rs-1121 */                                         //
/* FP:need_type_info.rs-1122 */                                         // See the `need_type_info/type-alias.rs` test for
/* FP:need_type_info.rs-1123 */                                         // some examples.
/* FP:need_type_info.rs-1124 */                                     }
/* FP:need_type_info.rs-1125 */                                     // There cannot be inference variables in the self type,
/* FP:need_type_info.rs-1126 */                                     // so there's nothing for us to do here.
/* FP:need_type_info.rs-1127 */                                     Res::SelfTyParam { .. } | Res::SelfTyAlias { .. } => {}
/* FP:need_type_info.rs-1128 */                                     _ => warn!(
/* FP:need_type_info.rs-1129 */                                         "unexpected path: def={:?} args={:?} path={:?}",
/* FP:need_type_info.rs-1130 */                                         def, args, path,
/* FP:need_type_info.rs-1131 */                                     ),
/* FP:need_type_info.rs-1132 */                                 }
/* FP:need_type_info.rs-1133 */                             } else {
/* FP:need_type_info.rs-1134 */                                 return Box::new(
/* FP:need_type_info.rs-1135 */                                     self.resolved_path_inferred_arg_iter(path, args).chain(segment),
/* FP:need_type_info.rs-1136 */                                 );
/* FP:need_type_info.rs-1137 */                             }
/* FP:need_type_info.rs-1138 */                         }
/* FP:need_type_info.rs-1139 */                         _ => (),
/* FP:need_type_info.rs-1140 */                     }
/* FP:need_type_info.rs-1141 */                 }
/* FP:need_type_info.rs-1142 */ 
/* FP:need_type_info.rs-1143 */                 Box::new(segment.into_iter())
/* FP:need_type_info.rs-1144 */             }
/* FP:need_type_info.rs-1145 */             hir::QPath::LangItem(_, _) => Box::new(iter::empty()),
/* FP:need_type_info.rs-1146 */         }
/* FP:need_type_info.rs-1147 */     }
/* FP:need_type_info.rs-1148 */ }
/* FP:need_type_info.rs-1149 */ 
/* FP:need_type_info.rs-1150 */ impl<'a, 'tcx> Visitor<'tcx> for FindInferSourceVisitor<'a, 'tcx> {
/* FP:need_type_info.rs-1151 */     type NestedFilter = nested_filter::OnlyBodies;
/* FP:need_type_info.rs-1152 */ 
/* FP:need_type_info.rs-1153 */     fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
/* FP:need_type_info.rs-1154 */         self.tecx.tcx
/* FP:need_type_info.rs-1155 */     }
/* FP:need_type_info.rs-1156 */ 
/* FP:need_type_info.rs-1157 */     fn visit_local(&mut self, local: &'tcx LetStmt<'tcx>) {
/* FP:need_type_info.rs-1158 */         intravisit::walk_local(self, local);
/* FP:need_type_info.rs-1159 */ 
/* FP:need_type_info.rs-1160 */         if let Some(ty) = self.opt_node_type(local.hir_id) {
/* FP:need_type_info.rs-1161 */             if self.generic_arg_contains_target(ty.into()) {
/* FP:need_type_info.rs-1162 */                 match local.source {
/* FP:need_type_info.rs-1163 */                     LocalSource::Normal if local.ty.is_none() => {
/* FP:need_type_info.rs-1164 */                         self.update_infer_source(InferSource {
/* FP:need_type_info.rs-1165 */                             span: local.pat.span,
/* FP:need_type_info.rs-1166 */                             kind: InferSourceKind::LetBinding {
/* FP:need_type_info.rs-1167 */                                 insert_span: local.pat.span.shrink_to_hi(),
/* FP:need_type_info.rs-1168 */                                 pattern_name: local.pat.simple_ident(),
/* FP:need_type_info.rs-1169 */                                 ty,
/* FP:need_type_info.rs-1170 */                                 def_id: None,
/* FP:need_type_info.rs-1171 */                             },
/* FP:need_type_info.rs-1172 */                         })
/* FP:need_type_info.rs-1173 */                     }
/* FP:need_type_info.rs-1174 */                     _ => {}
/* FP:need_type_info.rs-1175 */                 }
/* FP:need_type_info.rs-1176 */             }
/* FP:need_type_info.rs-1177 */         }
/* FP:need_type_info.rs-1178 */     }
/* FP:need_type_info.rs-1179 */ 
/* FP:need_type_info.rs-1180 */     /// For closures, we first visit the parameters and then the content,
/* FP:need_type_info.rs-1181 */     /// as we prefer those.
/* FP:need_type_info.rs-1182 */     fn visit_body(&mut self, body: &Body<'tcx>) {
/* FP:need_type_info.rs-1183 */         for param in body.params {
/* FP:need_type_info.rs-1184 */             debug!(
/* FP:need_type_info.rs-1185 */                 "param: span {:?}, ty_span {:?}, pat.span {:?}",
/* FP:need_type_info.rs-1186 */                 param.span, param.ty_span, param.pat.span
/* FP:need_type_info.rs-1187 */             );
/* FP:need_type_info.rs-1188 */             if param.ty_span != param.pat.span {
/* FP:need_type_info.rs-1189 */                 debug!("skipping param: has explicit type");
/* FP:need_type_info.rs-1190 */                 continue;
/* FP:need_type_info.rs-1191 */             }
/* FP:need_type_info.rs-1192 */ 
/* FP:need_type_info.rs-1193 */             let Some(param_ty) = self.opt_node_type(param.hir_id) else { continue };
/* FP:need_type_info.rs-1194 */ 
/* FP:need_type_info.rs-1195 */             if self.generic_arg_contains_target(param_ty.into()) {
/* FP:need_type_info.rs-1196 */                 self.update_infer_source(InferSource {
/* FP:need_type_info.rs-1197 */                     span: param.pat.span,
/* FP:need_type_info.rs-1198 */                     kind: InferSourceKind::ClosureArg {
/* FP:need_type_info.rs-1199 */                         insert_span: param.pat.span.shrink_to_hi(),
/* FP:need_type_info.rs-1200 */                         ty: param_ty,
/* FP:need_type_info.rs-1201 */                     },
/* FP:need_type_info.rs-1202 */                 })
/* FP:need_type_info.rs-1203 */             }
/* FP:need_type_info.rs-1204 */         }
/* FP:need_type_info.rs-1205 */         intravisit::walk_body(self, body);
/* FP:need_type_info.rs-1206 */     }
/* FP:need_type_info.rs-1207 */ 
/* FP:need_type_info.rs-1208 */     #[instrument(level = "debug", skip(self))]
/* FP:need_type_info.rs-1209 */     fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
/* FP:need_type_info.rs-1210 */         let tcx = self.tecx.tcx;
/* FP:need_type_info.rs-1211 */         match expr.kind {
/* FP:need_type_info.rs-1212 */             // When encountering `func(arg)` first look into `arg` and then `func`,
/* FP:need_type_info.rs-1213 */             // as `arg` is "more specific".
/* FP:need_type_info.rs-1214 */             ExprKind::Call(func, args) => {
/* FP:need_type_info.rs-1215 */                 for arg in args {
/* FP:need_type_info.rs-1216 */                     self.visit_expr(arg);
/* FP:need_type_info.rs-1217 */                 }
/* FP:need_type_info.rs-1218 */                 self.visit_expr(func);
/* FP:need_type_info.rs-1219 */             }
/* FP:need_type_info.rs-1220 */             _ => intravisit::walk_expr(self, expr),
/* FP:need_type_info.rs-1221 */         }
/* FP:need_type_info.rs-1222 */ 
/* FP:need_type_info.rs-1223 */         for args in self.expr_inferred_arg_iter(expr) {
/* FP:need_type_info.rs-1224 */             debug!(?args);
/* FP:need_type_info.rs-1225 */             let InsertableGenericArgs {
/* FP:need_type_info.rs-1226 */                 insert_span,
/* FP:need_type_info.rs-1227 */                 args,
/* FP:need_type_info.rs-1228 */                 generics_def_id,
/* FP:need_type_info.rs-1229 */                 def_id,
/* FP:need_type_info.rs-1230 */                 have_turbofish,
/* FP:need_type_info.rs-1231 */             } = args;
/* FP:need_type_info.rs-1232 */             let generics = tcx.generics_of(generics_def_id);
/* FP:need_type_info.rs-1233 */             if let Some(mut argument_index) = generics
/* FP:need_type_info.rs-1234 */                 .own_args(args)
/* FP:need_type_info.rs-1235 */                 .iter()
/* FP:need_type_info.rs-1236 */                 .position(|&arg| self.generic_arg_contains_target(arg))
/* FP:need_type_info.rs-1237 */             {
/* FP:need_type_info.rs-1238 */                 if generics.parent.is_none() && generics.has_self {
/* FP:need_type_info.rs-1239 */                     argument_index += 1;
/* FP:need_type_info.rs-1240 */                 }
/* FP:need_type_info.rs-1241 */                 let args = self.tecx.resolve_vars_if_possible(args);
/* FP:need_type_info.rs-1242 */                 let generic_args =
/* FP:need_type_info.rs-1243 */                     &generics.own_args_no_defaults(tcx, args)[generics.own_counts().lifetimes..];
/* FP:need_type_info.rs-1244 */                 let span = match expr.kind {
/* FP:need_type_info.rs-1245 */                     ExprKind::MethodCall(path, ..) => path.ident.span,
/* FP:need_type_info.rs-1246 */                     _ => expr.span,
/* FP:need_type_info.rs-1247 */                 };
/* FP:need_type_info.rs-1248 */ 
/* FP:need_type_info.rs-1249 */                 self.update_infer_source(InferSource {
/* FP:need_type_info.rs-1250 */                     span,
/* FP:need_type_info.rs-1251 */                     kind: InferSourceKind::GenericArg {
/* FP:need_type_info.rs-1252 */                         insert_span,
/* FP:need_type_info.rs-1253 */                         argument_index,
/* FP:need_type_info.rs-1254 */                         generics_def_id,
/* FP:need_type_info.rs-1255 */                         def_id,
/* FP:need_type_info.rs-1256 */                         generic_args,
/* FP:need_type_info.rs-1257 */                         have_turbofish,
/* FP:need_type_info.rs-1258 */                     },
/* FP:need_type_info.rs-1259 */                 });
/* FP:need_type_info.rs-1260 */             }
/* FP:need_type_info.rs-1261 */         }
/* FP:need_type_info.rs-1262 */ 
/* FP:need_type_info.rs-1263 */         if let Some(node_ty) = self.opt_node_type(expr.hir_id) {
/* FP:need_type_info.rs-1264 */             if let (
/* FP:need_type_info.rs-1265 */                 &ExprKind::Closure(&Closure { fn_decl, body, fn_decl_span, .. }),
/* FP:need_type_info.rs-1266 */                 ty::Closure(_, args),
/* FP:need_type_info.rs-1267 */             ) = (&expr.kind, node_ty.kind())
/* FP:need_type_info.rs-1268 */             {
/* FP:need_type_info.rs-1269 */                 let output = args.as_closure().sig().output().skip_binder();
/* FP:need_type_info.rs-1270 */                 if self.generic_arg_contains_target(output.into()) {
/* FP:need_type_info.rs-1271 */                     let body = self.tecx.tcx.hir_body(body);
/* FP:need_type_info.rs-1272 */                     let should_wrap_expr = if matches!(body.value.kind, ExprKind::Block(..)) {
/* FP:need_type_info.rs-1273 */                         None
/* FP:need_type_info.rs-1274 */                     } else {
/* FP:need_type_info.rs-1275 */                         Some(body.value.span.shrink_to_hi())
/* FP:need_type_info.rs-1276 */                     };
/* FP:need_type_info.rs-1277 */                     self.update_infer_source(InferSource {
/* FP:need_type_info.rs-1278 */                         span: fn_decl_span,
/* FP:need_type_info.rs-1279 */                         kind: InferSourceKind::ClosureReturn {
/* FP:need_type_info.rs-1280 */                             ty: output,
/* FP:need_type_info.rs-1281 */                             data: &fn_decl.output,
/* FP:need_type_info.rs-1282 */                             should_wrap_expr,
/* FP:need_type_info.rs-1283 */                         },
/* FP:need_type_info.rs-1284 */                     })
/* FP:need_type_info.rs-1285 */                 }
/* FP:need_type_info.rs-1286 */             }
/* FP:need_type_info.rs-1287 */         }
/* FP:need_type_info.rs-1288 */ 
/* FP:need_type_info.rs-1289 */         let has_impl_trait = |def_id| {
/* FP:need_type_info.rs-1290 */             iter::successors(Some(tcx.generics_of(def_id)), |generics| {
/* FP:need_type_info.rs-1291 */                 generics.parent.map(|def_id| tcx.generics_of(def_id))
/* FP:need_type_info.rs-1292 */             })
/* FP:need_type_info.rs-1293 */             .any(|generics| generics.has_impl_trait())
/* FP:need_type_info.rs-1294 */         };
/* FP:need_type_info.rs-1295 */         if let ExprKind::MethodCall(path, receiver, method_args, span) = expr.kind
/* FP:need_type_info.rs-1296 */             && let Some(args) = self.node_args_opt(expr.hir_id)
/* FP:need_type_info.rs-1297 */             && args.iter().any(|arg| self.generic_arg_contains_target(arg))
/* FP:need_type_info.rs-1298 */             && let Some(def_id) = self.typeck_results.type_dependent_def_id(expr.hir_id)
/* FP:need_type_info.rs-1299 */             && self.tecx.tcx.trait_of_assoc(def_id).is_some()
/* FP:need_type_info.rs-1300 */             && !has_impl_trait(def_id)
/* FP:need_type_info.rs-1301 */             // FIXME(fn_delegation): In delegation item argument spans are equal to last path
/* FP:need_type_info.rs-1302 */             // segment. This leads to ICE's when emitting `multipart_suggestion`.
/* FP:need_type_info.rs-1303 */             && tcx.hir_opt_delegation_sig_id(expr.hir_id.owner.def_id).is_none()
/* FP:need_type_info.rs-1304 */         {
/* FP:need_type_info.rs-1305 */             let successor =
/* FP:need_type_info.rs-1306 */                 method_args.get(0).map_or_else(|| (")", span.hi()), |arg| (", ", arg.span.lo()));
/* FP:need_type_info.rs-1307 */             let args = self.tecx.resolve_vars_if_possible(args);
/* FP:need_type_info.rs-1308 */             self.update_infer_source(InferSource {
/* FP:need_type_info.rs-1309 */                 span: path.ident.span,
/* FP:need_type_info.rs-1310 */                 kind: InferSourceKind::FullyQualifiedMethodCall {
/* FP:need_type_info.rs-1311 */                     receiver,
/* FP:need_type_info.rs-1312 */                     successor,
/* FP:need_type_info.rs-1313 */                     args,
/* FP:need_type_info.rs-1314 */                     def_id,
/* FP:need_type_info.rs-1315 */                 },
/* FP:need_type_info.rs-1316 */             })
/* FP:need_type_info.rs-1317 */         }
/* FP:need_type_info.rs-1318 */     }
/* FP:need_type_info.rs-1319 */ }