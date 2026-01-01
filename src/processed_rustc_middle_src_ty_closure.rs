/* FP:closure.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_closure_UNPARSEABLE_0001
/* FP:closure.rs-0002 */ use std::fmt::Write;
/* FP:closure.rs-0003 */ 
/* FP:closure.rs-0004 */ use crate::rustc_data_structures::fx::FxIndexMap;
/* FP:closure.rs-0005 */ use rustc_hir as hir;
/* FP:closure.rs-0006 */ use crate::rustc_complete::HirId;
/* FP:closure.rs-0007 */ use crate::rustc_complete::def_id::LocalDefId;
/* FP:closure.rs-0008 */ use rustc_macros::{HashStable, TyDecodable, TyEncodable, TypeFoldable, TypeVisitable};
/* FP:closure.rs-0009 */ use crate::rustc_complete::def_id::LocalDefIdMap;
/* FP:closure.rs-0010 */ use crate::rustc_complete::{Ident, Span, Symbol};
/* FP:closure.rs-0011 */ 
/* FP:closure.rs-0012 */ use super::TyCtxt;
/* FP:closure.rs-0013 */ use crate::hir::place::{
/* FP:closure.rs-0014 */     Place as HirPlace, PlaceBase as HirPlaceBase, ProjectionKind as HirProjectionKind,
/* FP:closure.rs-0015 */ };
/* FP:closure.rs-0016 */ use crate::query::Providers;
/* FP:closure.rs-0017 */ use crate::{mir, ty};
/* FP:closure.rs-0018 */ 
/* FP:closure.rs-0019 */ /// Captures are represented using fields inside a structure.
/* FP:closure.rs-0020 */ /// This represents accessing self in the closure structure
/* FP:closure.rs-0021 */ pub const CAPTURE_STRUCT_LOCAL: mir::Local = mir::Local::from_u32(1);
/* FP:closure.rs-0022 */ 
/* FP:closure.rs-0023 */ #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, TyEncodable, TyDecodable, HashStable)]
/* FP:closure.rs-0024 */ #[derive(TypeFoldable, TypeVisitable)]
/* FP:closure.rs-0025 */ pub struct UpvarPath {
/* FP:closure.rs-0026 */     pub hir_id: HirId,
/* FP:closure.rs-0027 */ }
/* FP:closure.rs-0028 */ 
/* FP:closure.rs-0029 */ /// Upvars do not get their own `NodeId`. Instead, we use the pair of
/* FP:closure.rs-0030 */ /// the original var ID (that is, the root variable that is referenced
/* FP:closure.rs-0031 */ /// by the upvar) and the ID of the closure expression.
/* FP:closure.rs-0032 */ #[derive(Clone, Copy, PartialEq, Eq, Hash, TyEncodable, TyDecodable, HashStable)]
/* FP:closure.rs-0033 */ #[derive(TypeFoldable, TypeVisitable)]
/* FP:closure.rs-0034 */ pub struct UpvarId {
/* FP:closure.rs-0035 */     pub var_path: UpvarPath,
/* FP:closure.rs-0036 */     pub closure_expr_id: LocalDefId,
/* FP:closure.rs-0037 */ }
/* FP:closure.rs-0038 */ 
/* FP:closure.rs-0039 */ impl UpvarId {
/* FP:closure.rs-0040 */     pub fn new(var_hir_id: HirId, closure_def_id: LocalDefId) -> UpvarId {
/* FP:closure.rs-0041 */         UpvarId { var_path: UpvarPath { hir_id: var_hir_id }, closure_expr_id: closure_def_id }
/* FP:closure.rs-0042 */     }
/* FP:closure.rs-0043 */ }
/* FP:closure.rs-0044 */ 
/* FP:closure.rs-0045 */ /// Information describing the capture of an upvar. This is computed
/* FP:closure.rs-0046 */ /// during `typeck`, specifically by `regionck`.
/* FP:closure.rs-0047 */ #[derive(Eq, PartialEq, Clone, Debug, Copy, TyEncodable, TyDecodable, HashStable, Hash)]
/* FP:closure.rs-0048 */ #[derive(TypeFoldable, TypeVisitable)]
/* FP:closure.rs-0049 */ pub enum UpvarCapture {
/* FP:closure.rs-0050 */     /// Upvar is captured by value. This is always true when the
/* FP:closure.rs-0051 */     /// closure is labeled `move`, but can also be true in other cases
/* FP:closure.rs-0052 */     /// depending on inference.
/* FP:closure.rs-0053 */     ByValue,
/* FP:closure.rs-0054 */ 
/* FP:closure.rs-0055 */     /// Upvar is captured by use. This is true when the closure is labeled `use`.
/* FP:closure.rs-0056 */     ByUse,
/* FP:closure.rs-0057 */ 
/* FP:closure.rs-0058 */     /// Upvar is captured by reference.
/* FP:closure.rs-0059 */     ByRef(BorrowKind),
/* FP:closure.rs-0060 */ }
/* FP:closure.rs-0061 */ 
/* FP:closure.rs-0062 */ /// Given the closure DefId this map provides a map of root variables to minimum
/* FP:closure.rs-0063 */ /// set of `CapturedPlace`s that need to be tracked to support all captures of that closure.
/* FP:closure.rs-0064 */ pub type MinCaptureInformationMap<'tcx> = LocalDefIdMap<RootVariableMinCaptureList<'tcx>>;
/* FP:closure.rs-0065 */ 
/* FP:closure.rs-0066 */ /// Part of `MinCaptureInformationMap`; Maps a root variable to the list of `CapturedPlace`.
/* FP:closure.rs-0067 */ /// Used to track the minimum set of `Place`s that need to be captured to support all
/* FP:closure.rs-0068 */ /// Places captured by the closure starting at a given root variable.
/* FP:closure.rs-0069 */ ///
/* FP:closure.rs-0070 */ /// This provides a convenient and quick way of checking if a variable being used within
/* FP:closure.rs-0071 */ /// a closure is a capture of a local variable.
/* FP:closure.rs-0072 */ pub type RootVariableMinCaptureList<'tcx> = FxIndexMap<HirId, MinCaptureList<'tcx>>;
/* FP:closure.rs-0073 */ 
/* FP:closure.rs-0074 */ /// Part of `MinCaptureInformationMap`; List of `CapturePlace`s.
/* FP:closure.rs-0075 */ pub type MinCaptureList<'tcx> = Vec<CapturedPlace<'tcx>>;
/* FP:closure.rs-0076 */ 
/* FP:closure.rs-0077 */ /// A composite describing a `Place` that is captured by a closure.
/* FP:closure.rs-0078 */ #[derive(Eq, PartialEq, Clone, Debug, TyEncodable, TyDecodable, HashStable, Hash)]
/* FP:closure.rs-0079 */ #[derive(TypeFoldable, TypeVisitable)]
/* FP:closure.rs-0080 */ pub struct CapturedPlace<'tcx> {
/* FP:closure.rs-0081 */     /// Name and span where the binding happens.
/* FP:closure.rs-0082 */     pub var_ident: Ident,
/* FP:closure.rs-0083 */ 
/* FP:closure.rs-0084 */     /// The `Place` that is captured.
/* FP:closure.rs-0085 */     pub place: HirPlace<'tcx>,
/* FP:closure.rs-0086 */ 
/* FP:closure.rs-0087 */     /// `CaptureKind` and expression(s) that resulted in such capture of `place`.
/* FP:closure.rs-0088 */     pub info: CaptureInfo,
/* FP:closure.rs-0089 */ 
/* FP:closure.rs-0090 */     /// Represents if `place` can be mutated or not.
/* FP:closure.rs-0091 */     pub mutability: hir::Mutability,
/* FP:closure.rs-0092 */ }
/* FP:closure.rs-0093 */ 
/* FP:closure.rs-0094 */ impl<'tcx> CapturedPlace<'tcx> {
/* FP:closure.rs-0095 */     pub fn to_string(&self, tcx: TyCtxt<'tcx>) -> String {
/* FP:closure.rs-0096 */         place_to_string_for_capture(tcx, &self.place)
/* FP:closure.rs-0097 */     }
/* FP:closure.rs-0098 */ 
/* FP:closure.rs-0099 */     /// Returns a symbol of the captured upvar, which looks like `name__field1__field2`.
/* FP:closure.rs-0100 */     pub fn to_symbol(&self) -> Symbol {
/* FP:closure.rs-0101 */         let mut symbol = self.var_ident.to_string();
/* FP:closure.rs-0102 */ 
/* FP:closure.rs-0103 */         let mut ty = self.place.base_ty;
/* FP:closure.rs-0104 */         for proj in self.place.projections.iter() {
/* FP:closure.rs-0105 */             match proj.kind {
/* FP:closure.rs-0106 */                 HirProjectionKind::Field(idx, variant) => match ty.kind() {
/* FP:closure.rs-0107 */                     ty::Tuple(_) => write!(&mut symbol, "__{}", idx.index()).unwrap(),
/* FP:closure.rs-0108 */                     ty::Adt(def, ..) => {
/* FP:closure.rs-0109 */                         write!(
/* FP:closure.rs-0110 */                             &mut symbol,
/* FP:closure.rs-0111 */                             "__{}",
/* FP:closure.rs-0112 */                             def.variant(variant).fields[idx].name.as_str(),
/* FP:closure.rs-0113 */                         )
/* FP:closure.rs-0114 */                         .unwrap();
/* FP:closure.rs-0115 */                     }
/* FP:closure.rs-0116 */                     ty => {
/* FP:closure.rs-0117 */                         bug!("Unexpected type {:?} for `Field` projection", ty)
/* FP:closure.rs-0118 */                     }
/* FP:closure.rs-0119 */                 },
/* FP:closure.rs-0120 */ 
/* FP:closure.rs-0121 */                 HirProjectionKind::UnwrapUnsafeBinder => {
/* FP:closure.rs-0122 */                     write!(&mut symbol, "__unwrap").unwrap();
/* FP:closure.rs-0123 */                 }
/* FP:closure.rs-0124 */ 
/* FP:closure.rs-0125 */                 // Ignore derefs for now, as they are likely caused by
/* FP:closure.rs-0126 */                 // autoderefs that don't appear in the original code.
/* FP:closure.rs-0127 */                 HirProjectionKind::Deref => {}
/* FP:closure.rs-0128 */                 // Just change the type to the hidden type, so we can actually project.
/* FP:closure.rs-0129 */                 HirProjectionKind::OpaqueCast => {}
/* FP:closure.rs-0130 */                 proj => bug!("Unexpected projection {:?} in captured place", proj),
/* FP:closure.rs-0131 */             }
/* FP:closure.rs-0132 */             ty = proj.ty;
/* FP:closure.rs-0133 */         }
/* FP:closure.rs-0134 */ 
/* FP:closure.rs-0135 */         Symbol::intern(&symbol)
/* FP:closure.rs-0136 */     }
/* FP:closure.rs-0137 */ 
/* FP:closure.rs-0138 */     /// Returns the hir-id of the root variable for the captured place.
/* FP:closure.rs-0139 */     /// e.g., if `a.b.c` was captured, would return the hir-id for `a`.
/* FP:closure.rs-0140 */     pub fn get_root_variable(&self) -> HirId {
/* FP:closure.rs-0141 */         match self.place.base {
/* FP:closure.rs-0142 */             HirPlaceBase::Upvar(upvar_id) => upvar_id.var_path.hir_id,
/* FP:closure.rs-0143 */             base => bug!("Expected upvar, found={:?}", base),
/* FP:closure.rs-0144 */         }
/* FP:closure.rs-0145 */     }
/* FP:closure.rs-0146 */ 
/* FP:closure.rs-0147 */     /// Returns the `LocalDefId` of the closure that captured this Place
/* FP:closure.rs-0148 */     pub fn get_closure_local_def_id(&self) -> LocalDefId {
/* FP:closure.rs-0149 */         match self.place.base {
/* FP:closure.rs-0150 */             HirPlaceBase::Upvar(upvar_id) => upvar_id.closure_expr_id,
/* FP:closure.rs-0151 */             base => bug!("expected upvar, found={:?}", base),
/* FP:closure.rs-0152 */         }
/* FP:closure.rs-0153 */     }
/* FP:closure.rs-0154 */ 
/* FP:closure.rs-0155 */     /// Return span pointing to use that resulted in selecting the captured path
/* FP:closure.rs-0156 */     pub fn get_path_span(&self, tcx: TyCtxt<'tcx>) -> Span {
/* FP:closure.rs-0157 */         if let Some(path_expr_id) = self.info.path_expr_id {
/* FP:closure.rs-0158 */             tcx.hir_span(path_expr_id)
/* FP:closure.rs-0159 */         } else if let Some(capture_kind_expr_id) = self.info.capture_kind_expr_id {
/* FP:closure.rs-0160 */             tcx.hir_span(capture_kind_expr_id)
/* FP:closure.rs-0161 */         } else {
/* FP:closure.rs-0162 */             // Fallback on upvars mentioned if neither path or capture expr id is captured
/* FP:closure.rs-0163 */ 
/* FP:closure.rs-0164 */             // Safe to unwrap since we know this place is captured by the closure, therefore the closure must have upvars.
/* FP:closure.rs-0165 */             tcx.upvars_mentioned(self.get_closure_local_def_id()).unwrap()
/* FP:closure.rs-0166 */                 [&self.get_root_variable()]
/* FP:closure.rs-0167 */                 .span
/* FP:closure.rs-0168 */         }
/* FP:closure.rs-0169 */     }
/* FP:closure.rs-0170 */ 
/* FP:closure.rs-0171 */     /// Return span pointing to use that resulted in selecting the current capture kind
/* FP:closure.rs-0172 */     pub fn get_capture_kind_span(&self, tcx: TyCtxt<'tcx>) -> Span {
/* FP:closure.rs-0173 */         if let Some(capture_kind_expr_id) = self.info.capture_kind_expr_id {
/* FP:closure.rs-0174 */             tcx.hir_span(capture_kind_expr_id)
/* FP:closure.rs-0175 */         } else if let Some(path_expr_id) = self.info.path_expr_id {
/* FP:closure.rs-0176 */             tcx.hir_span(path_expr_id)
/* FP:closure.rs-0177 */         } else {
/* FP:closure.rs-0178 */             // Fallback on upvars mentioned if neither path or capture expr id is captured
/* FP:closure.rs-0179 */ 
/* FP:closure.rs-0180 */             // Safe to unwrap since we know this place is captured by the closure, therefore the closure must have upvars.
/* FP:closure.rs-0181 */             tcx.upvars_mentioned(self.get_closure_local_def_id()).unwrap()
/* FP:closure.rs-0182 */                 [&self.get_root_variable()]
/* FP:closure.rs-0183 */                 .span
/* FP:closure.rs-0184 */         }
/* FP:closure.rs-0185 */     }
/* FP:closure.rs-0186 */ 
/* FP:closure.rs-0187 */     pub fn is_by_ref(&self) -> bool {
/* FP:closure.rs-0188 */         match self.info.capture_kind {
/* FP:closure.rs-0189 */             ty::UpvarCapture::ByValue | ty::UpvarCapture::ByUse => false,
/* FP:closure.rs-0190 */             ty::UpvarCapture::ByRef(..) => true,
/* FP:closure.rs-0191 */         }
/* FP:closure.rs-0192 */     }
/* FP:closure.rs-0193 */ }
/* FP:closure.rs-0194 */ 
/* FP:closure.rs-0195 */ #[derive(Copy, Clone, Debug, HashStable)]
/* FP:closure.rs-0196 */ pub struct ClosureTypeInfo<'tcx> {
/* FP:closure.rs-0197 */     user_provided_sig: ty::CanonicalPolyFnSig<'tcx>,
/* FP:closure.rs-0198 */     captures: &'tcx ty::List<&'tcx ty::CapturedPlace<'tcx>>,
/* FP:closure.rs-0199 */     kind_origin: Option<&'tcx (Span, HirPlace<'tcx>)>,
/* FP:closure.rs-0200 */ }
/* FP:closure.rs-0201 */ 
/* FP:closure.rs-0202 */ fn closure_typeinfo<'tcx>(tcx: TyCtxt<'tcx>, def: LocalDefId) -> ClosureTypeInfo<'tcx> {
/* FP:closure.rs-0203 */     debug_assert!(tcx.is_closure_like(def.to_def_id()));
/* FP:closure.rs-0204 */     let typeck_results = tcx.typeck(def);
/* FP:closure.rs-0205 */     let user_provided_sig = typeck_results.user_provided_sigs[&def];
/* FP:closure.rs-0206 */     let captures = typeck_results.closure_min_captures_flattened(def);
/* FP:closure.rs-0207 */     let captures = tcx.mk_captures_from_iter(captures);
/* FP:closure.rs-0208 */     let hir_id = tcx.local_def_id_to_hir_id(def);
/* FP:closure.rs-0209 */     let kind_origin = typeck_results.closure_kind_origins().get(hir_id);
/* FP:closure.rs-0210 */     ClosureTypeInfo { user_provided_sig, captures, kind_origin }
/* FP:closure.rs-0211 */ }
/* FP:closure.rs-0212 */ 
/* FP:closure.rs-0213 */ impl<'tcx> TyCtxt<'tcx> {
/* FP:closure.rs-0214 */     pub fn closure_kind_origin(self, def_id: LocalDefId) -> Option<&'tcx (Span, HirPlace<'tcx>)> {
/* FP:closure.rs-0215 */         self.closure_typeinfo(def_id).kind_origin
/* FP:closure.rs-0216 */     }
/* FP:closure.rs-0217 */ 
/* FP:closure.rs-0218 */     pub fn closure_user_provided_sig(self, def_id: LocalDefId) -> ty::CanonicalPolyFnSig<'tcx> {
/* FP:closure.rs-0219 */         self.closure_typeinfo(def_id).user_provided_sig
/* FP:closure.rs-0220 */     }
/* FP:closure.rs-0221 */ 
/* FP:closure.rs-0222 */     pub fn closure_captures(self, def_id: LocalDefId) -> &'tcx [&'tcx ty::CapturedPlace<'tcx>] {
/* FP:closure.rs-0223 */         if !self.is_closure_like(def_id.to_def_id()) {
/* FP:closure.rs-0224 */             return &[];
/* FP:closure.rs-0225 */         }
/* FP:closure.rs-0226 */         self.closure_typeinfo(def_id).captures
/* FP:closure.rs-0227 */     }
/* FP:closure.rs-0228 */ }
/* FP:closure.rs-0229 */ 
/* FP:closure.rs-0230 */ /// Return true if the `proj_possible_ancestor` represents an ancestor path
/* FP:closure.rs-0231 */ /// to `proj_capture` or `proj_possible_ancestor` is same as `proj_capture`,
/* FP:closure.rs-0232 */ /// assuming they both start off of the same root variable.
/* FP:closure.rs-0233 */ ///
/* FP:closure.rs-0234 */ /// **Note:** It's the caller's responsibility to ensure that both lists of projections
/* FP:closure.rs-0235 */ ///           start off of the same root variable.
/* FP:closure.rs-0236 */ ///
/* FP:closure.rs-0237 */ /// Eg: 1. `foo.x` which is represented using `projections=[Field(x)]` is an ancestor of
/* FP:closure.rs-0238 */ ///        `foo.x.y` which is represented using `projections=[Field(x), Field(y)]`.
/* FP:closure.rs-0239 */ ///        Note both `foo.x` and `foo.x.y` start off of the same root variable `foo`.
/* FP:closure.rs-0240 */ ///     2. Since we only look at the projections here function will return `bar.x` as a valid
/* FP:closure.rs-0241 */ ///        ancestor of `foo.x.y`. It's the caller's responsibility to ensure that both projections
/* FP:closure.rs-0242 */ ///        list are being applied to the same root variable.
/* FP:closure.rs-0243 */ pub fn is_ancestor_or_same_capture(
/* FP:closure.rs-0244 */     proj_possible_ancestor: &[HirProjectionKind],
/* FP:closure.rs-0245 */     proj_capture: &[HirProjectionKind],
/* FP:closure.rs-0246 */ ) -> bool {
/* FP:closure.rs-0247 */     // We want to make sure `is_ancestor_or_same_capture("x.0.0", "x.0")` to return false.
/* FP:closure.rs-0248 */     // Therefore we can't just check if all projections are same in the zipped iterator below.
/* FP:closure.rs-0249 */     if proj_possible_ancestor.len() > proj_capture.len() {
/* FP:closure.rs-0250 */         return false;
/* FP:closure.rs-0251 */     }
/* FP:closure.rs-0252 */ 
/* FP:closure.rs-0253 */     proj_possible_ancestor.iter().zip(proj_capture).all(|(a, b)| a == b)
/* FP:closure.rs-0254 */ }
/* FP:closure.rs-0255 */ 
/* FP:closure.rs-0256 */ /// Part of `MinCaptureInformationMap`; describes the capture kind (&, &mut, move)
/* FP:closure.rs-0257 */ /// for a particular capture as well as identifying the part of the source code
/* FP:closure.rs-0258 */ /// that triggered this capture to occur.
/* FP:closure.rs-0259 */ #[derive(Eq, PartialEq, Clone, Debug, Copy, TyEncodable, TyDecodable, HashStable, Hash)]
/* FP:closure.rs-0260 */ #[derive(TypeFoldable, TypeVisitable)]
/* FP:closure.rs-0261 */ pub struct CaptureInfo {
/* FP:closure.rs-0262 */     /// Expr Id pointing to use that resulted in selecting the current capture kind
/* FP:closure.rs-0263 */     ///
/* FP:closure.rs-0264 */     /// Eg:
/* FP:closure.rs-0265 */     /// ```rust,no_run
/* FP:closure.rs-0266 */     /// let mut t = (0,1);
/* FP:closure.rs-0267 */     ///
/* FP:closure.rs-0268 */     /// let c = || {
/* FP:closure.rs-0269 */     ///     println!("{t:?}"); // L1
/* FP:closure.rs-0270 */     ///     t.1 = 4; // L2
/* FP:closure.rs-0271 */     /// };
/* FP:closure.rs-0272 */     /// ```
/* FP:closure.rs-0273 */     /// `capture_kind_expr_id` will point to the use on L2 and `path_expr_id` will point to the
/* FP:closure.rs-0274 */     /// use on L1.
/* FP:closure.rs-0275 */     ///
/* FP:closure.rs-0276 */     /// If the user doesn't enable feature `capture_disjoint_fields` (RFC 2229) then, it is
/* FP:closure.rs-0277 */     /// possible that we don't see the use of a particular place resulting in capture_kind_expr_id being
/* FP:closure.rs-0278 */     /// None. In such case we fallback on uvpars_mentioned for span.
/* FP:closure.rs-0279 */     ///
/* FP:closure.rs-0280 */     /// Eg:
/* FP:closure.rs-0281 */     /// ```rust,no_run
/* FP:closure.rs-0282 */     /// let x = 5;
/* FP:closure.rs-0283 */     ///
/* FP:closure.rs-0284 */     /// let c = || {
/* FP:closure.rs-0285 */     ///     let _ = x;
/* FP:closure.rs-0286 */     /// };
/* FP:closure.rs-0287 */     /// ```
/* FP:closure.rs-0288 */     ///
/* FP:closure.rs-0289 */     /// In this example, if `capture_disjoint_fields` is **not** set, then x will be captured,
/* FP:closure.rs-0290 */     /// but we won't see it being used during capture analysis, since it's essentially a discard.
/* FP:closure.rs-0291 */     pub capture_kind_expr_id: Option<HirId>,
/* FP:closure.rs-0292 */     /// Expr Id pointing to use that resulted the corresponding place being captured
/* FP:closure.rs-0293 */     ///
/* FP:closure.rs-0294 */     /// See `capture_kind_expr_id` for example.
/* FP:closure.rs-0295 */     ///
/* FP:closure.rs-0296 */     pub path_expr_id: Option<HirId>,
/* FP:closure.rs-0297 */ 
/* FP:closure.rs-0298 */     /// Capture mode that was selected
/* FP:closure.rs-0299 */     pub capture_kind: UpvarCapture,
/* FP:closure.rs-0300 */ }
/* FP:closure.rs-0301 */ 
/* FP:closure.rs-0302 */ pub fn place_to_string_for_capture<'tcx>(tcx: TyCtxt<'tcx>, place: &HirPlace<'tcx>) -> String {
/* FP:closure.rs-0303 */     let mut curr_string: String = match place.base {
/* FP:closure.rs-0304 */         HirPlaceBase::Upvar(upvar_id) => tcx.hir_name(upvar_id.var_path.hir_id).to_string(),
/* FP:closure.rs-0305 */         _ => bug!("Capture_information should only contain upvars"),
/* FP:closure.rs-0306 */     };
/* FP:closure.rs-0307 */ 
/* FP:closure.rs-0308 */     for (i, proj) in place.projections.iter().enumerate() {
/* FP:closure.rs-0309 */         match proj.kind {
/* FP:closure.rs-0310 */             HirProjectionKind::Deref => {
/* FP:closure.rs-0311 */                 curr_string = format!("*{curr_string}");
/* FP:closure.rs-0312 */             }
/* FP:closure.rs-0313 */             HirProjectionKind::Field(idx, variant) => match place.ty_before_projection(i).kind() {
/* FP:closure.rs-0314 */                 ty::Adt(def, ..) => {
/* FP:closure.rs-0315 */                     curr_string = format!(
/* FP:closure.rs-0316 */                         "{}.{}",
/* FP:closure.rs-0317 */                         curr_string,
/* FP:closure.rs-0318 */                         def.variant(variant).fields[idx].name.as_str()
/* FP:closure.rs-0319 */                     );
/* FP:closure.rs-0320 */                 }
/* FP:closure.rs-0321 */                 ty::Tuple(_) => {
/* FP:closure.rs-0322 */                     curr_string = format!("{}.{}", curr_string, idx.index());
/* FP:closure.rs-0323 */                 }
/* FP:closure.rs-0324 */                 _ => {
/* FP:closure.rs-0325 */                     bug!(
/* FP:closure.rs-0326 */                         "Field projection applied to a type other than Adt or Tuple: {:?}.",
/* FP:closure.rs-0327 */                         place.ty_before_projection(i).kind()
/* FP:closure.rs-0328 */                     )
/* FP:closure.rs-0329 */                 }
/* FP:closure.rs-0330 */             },
/* FP:closure.rs-0331 */             proj => bug!("{:?} unexpected because it isn't captured", proj),
/* FP:closure.rs-0332 */         }
/* FP:closure.rs-0333 */     }
/* FP:closure.rs-0334 */ 
/* FP:closure.rs-0335 */     curr_string
/* FP:closure.rs-0336 */ }
/* FP:closure.rs-0337 */ 
/* FP:closure.rs-0338 */ #[derive(Eq, Clone, PartialEq, Debug, TyEncodable, TyDecodable, Copy, HashStable, Hash)]
/* FP:closure.rs-0339 */ #[derive(TypeFoldable, TypeVisitable)]
/* FP:closure.rs-0340 */ pub enum BorrowKind {
/* FP:closure.rs-0341 */     /// Data must be immutable and is aliasable.
/* FP:closure.rs-0342 */     Immutable,
/* FP:closure.rs-0343 */ 
/* FP:closure.rs-0344 */     /// Data must be immutable but not aliasable. This kind of borrow
/* FP:closure.rs-0345 */     /// cannot currently be expressed by the user and is used only in
/* FP:closure.rs-0346 */     /// implicit closure bindings. It is needed when the closure
/* FP:closure.rs-0347 */     /// is borrowing or mutating a mutable referent, e.g.:
/* FP:closure.rs-0348 */     ///
/* FP:closure.rs-0349 */     /// ```
/* FP:closure.rs-0350 */     /// let mut z = 3;
/* FP:closure.rs-0351 */     /// let x: &mut isize = &mut z;
/* FP:closure.rs-0352 */     /// let y = || *x += 5;
/* FP:closure.rs-0353 */     /// ```
/* FP:closure.rs-0354 */     ///
/* FP:closure.rs-0355 */     /// If we were to try to translate this closure into a more explicit
/* FP:closure.rs-0356 */     /// form, we'd encounter an error with the code as written:
/* FP:closure.rs-0357 */     ///
/* FP:closure.rs-0358 */     /// ```compile_fail,E0594
/* FP:closure.rs-0359 */     /// struct Env<'a> { x: &'a &'a mut isize }
/* FP:closure.rs-0360 */     /// let mut z = 3;
/* FP:closure.rs-0361 */     /// let x: &mut isize = &mut z;
/* FP:closure.rs-0362 */     /// let y = (&mut Env { x: &x }, fn_ptr);  // Closure is pair of env and fn
/* FP:closure.rs-0363 */     /// fn fn_ptr(env: &mut Env) { **env.x += 5; }
/* FP:closure.rs-0364 */     /// ```
/* FP:closure.rs-0365 */     ///
/* FP:closure.rs-0366 */     /// This is then illegal because you cannot mutate a `&mut` found
/* FP:closure.rs-0367 */     /// in an aliasable location. To solve, you'd have to translate with
/* FP:closure.rs-0368 */     /// an `&mut` borrow:
/* FP:closure.rs-0369 */     ///
/* FP:closure.rs-0370 */     /// ```compile_fail,E0596
/* FP:closure.rs-0371 */     /// struct Env<'a> { x: &'a mut &'a mut isize }
/* FP:closure.rs-0372 */     /// let mut z = 3;
/* FP:closure.rs-0373 */     /// let x: &mut isize = &mut z;
/* FP:closure.rs-0374 */     /// let y = (&mut Env { x: &mut x }, fn_ptr); // changed from &x to &mut x
/* FP:closure.rs-0375 */     /// fn fn_ptr(env: &mut Env) { **env.x += 5; }
/* FP:closure.rs-0376 */     /// ```
/* FP:closure.rs-0377 */     ///
/* FP:closure.rs-0378 */     /// Now the assignment to `**env.x` is legal, but creating a
/* FP:closure.rs-0379 */     /// mutable pointer to `x` is not because `x` is not mutable. We
/* FP:closure.rs-0380 */     /// could fix this by declaring `x` as `let mut x`. This is ok in
/* FP:closure.rs-0381 */     /// user code, if awkward, but extra weird for closures, since the
/* FP:closure.rs-0382 */     /// borrow is hidden.
/* FP:closure.rs-0383 */     ///
/* FP:closure.rs-0384 */     /// So we introduce a "unique imm" borrow -- the referent is
/* FP:closure.rs-0385 */     /// immutable, but not aliasable. This solves the problem. For
/* FP:closure.rs-0386 */     /// simplicity, we don't give users the way to express this
/* FP:closure.rs-0387 */     /// borrow, it's just used when translating closures.
/* FP:closure.rs-0388 */     ///
/* FP:closure.rs-0389 */     /// FIXME: Rename this to indicate the borrow is actually not immutable.
/* FP:closure.rs-0390 */     UniqueImmutable,
/* FP:closure.rs-0391 */ 
/* FP:closure.rs-0392 */     /// Data is mutable and not aliasable.
/* FP:closure.rs-0393 */     Mutable,
/* FP:closure.rs-0394 */ }
/* FP:closure.rs-0395 */ 
/* FP:closure.rs-0396 */ impl BorrowKind {
/* FP:closure.rs-0397 */     pub fn from_mutbl(m: hir::Mutability) -> BorrowKind {
/* FP:closure.rs-0398 */         match m {
/* FP:closure.rs-0399 */             hir::Mutability::Mut => BorrowKind::Mutable,
/* FP:closure.rs-0400 */             hir::Mutability::Not => BorrowKind::Immutable,
/* FP:closure.rs-0401 */         }
/* FP:closure.rs-0402 */     }
/* FP:closure.rs-0403 */ 
/* FP:closure.rs-0404 */     /// Returns a mutability `m` such that an `&m T` pointer could be used to obtain this borrow
/* FP:closure.rs-0405 */     /// kind. Because borrow kinds are richer than mutabilities, we sometimes have to pick a
/* FP:closure.rs-0406 */     /// mutability that is stronger than necessary so that it at least *would permit* the borrow in
/* FP:closure.rs-0407 */     /// question.
/* FP:closure.rs-0408 */     pub fn to_mutbl_lossy(self) -> hir::Mutability {
/* FP:closure.rs-0409 */         match self {
/* FP:closure.rs-0410 */             BorrowKind::Mutable => hir::Mutability::Mut,
/* FP:closure.rs-0411 */             BorrowKind::Immutable => hir::Mutability::Not,
/* FP:closure.rs-0412 */ 
/* FP:closure.rs-0413 */             // We have no type corresponding to a unique imm borrow, so
/* FP:closure.rs-0414 */             // use `&mut`. It gives all the capabilities of a `&uniq`
/* FP:closure.rs-0415 */             // and hence is a safe "over approximation".
/* FP:closure.rs-0416 */             BorrowKind::UniqueImmutable => hir::Mutability::Mut,
/* FP:closure.rs-0417 */         }
/* FP:closure.rs-0418 */     }
/* FP:closure.rs-0419 */ }
/* FP:closure.rs-0420 */ 
/* FP:closure.rs-0421 */ pub fn analyze_coroutine_closure_captures<'a, 'tcx: 'a, T>(
/* FP:closure.rs-0422 */     parent_captures: impl IntoIterator<Item = &'a CapturedPlace<'tcx>>,
/* FP:closure.rs-0423 */     child_captures: impl IntoIterator<Item = &'a CapturedPlace<'tcx>>,
/* FP:closure.rs-0424 */     mut for_each: impl FnMut((usize, &'a CapturedPlace<'tcx>), (usize, &'a CapturedPlace<'tcx>)) -> T,
/* FP:closure.rs-0425 */ ) -> impl Iterator<Item = T> {
/* FP:closure.rs-0426 */     gen move {
/* FP:closure.rs-0427 */         let mut child_captures = child_captures.into_iter().enumerate().peekable();
/* FP:closure.rs-0428 */ 
/* FP:closure.rs-0429 */         // One parent capture may correspond to several child captures if we end up
/* FP:closure.rs-0430 */         // refining the set of captures via edition-2021 precise captures. We want to
/* FP:closure.rs-0431 */         // match up any number of child captures with one parent capture, so we keep
/* FP:closure.rs-0432 */         // peeking off this `Peekable` until the child doesn't match anymore.
/* FP:closure.rs-0433 */         for (parent_field_idx, parent_capture) in parent_captures.into_iter().enumerate() {
/* FP:closure.rs-0434 */             // Make sure we use every field at least once, b/c why are we capturing something
/* FP:closure.rs-0435 */             // if it's not used in the inner coroutine.
/* FP:closure.rs-0436 */             let mut field_used_at_least_once = false;
/* FP:closure.rs-0437 */ 
/* FP:closure.rs-0438 */             // A parent matches a child if they share the same prefix of projections.
/* FP:closure.rs-0439 */             // The child may have more, if it is capturing sub-fields out of
/* FP:closure.rs-0440 */             // something that is captured by-move in the parent closure.
/* FP:closure.rs-0441 */             while child_captures.peek().is_some_and(|(_, child_capture)| {
/* FP:closure.rs-0442 */                 child_prefix_matches_parent_projections(parent_capture, child_capture)
/* FP:closure.rs-0443 */             }) {
/* FP:closure.rs-0444 */                 let (child_field_idx, child_capture) = child_captures.next().unwrap();
/* FP:closure.rs-0445 */                 // This analysis only makes sense if the parent capture is a
/* FP:closure.rs-0446 */                 // prefix of the child capture.
/* FP:closure.rs-0447 */                 assert!(
/* FP:closure.rs-0448 */                     child_capture.place.projections.len() >= parent_capture.place.projections.len(),
/* FP:closure.rs-0449 */                     "parent capture ({parent_capture:#?}) expected to be prefix of \
/* FP:closure.rs-0450 */                     child capture ({child_capture:#?})"
/* FP:closure.rs-0451 */                 );
/* FP:closure.rs-0452 */ 
/* FP:closure.rs-0453 */                 yield for_each(
/* FP:closure.rs-0454 */                     (parent_field_idx, parent_capture),
/* FP:closure.rs-0455 */                     (child_field_idx, child_capture),
/* FP:closure.rs-0456 */                 );
/* FP:closure.rs-0457 */ 
/* FP:closure.rs-0458 */                 field_used_at_least_once = true;
/* FP:closure.rs-0459 */             }
/* FP:closure.rs-0460 */ 
/* FP:closure.rs-0461 */             // Make sure the field was used at least once.
/* FP:closure.rs-0462 */             assert!(
/* FP:closure.rs-0463 */                 field_used_at_least_once,
/* FP:closure.rs-0464 */                 "we captured {parent_capture:#?} but it was not used in the child coroutine?"
/* FP:closure.rs-0465 */             );
/* FP:closure.rs-0466 */         }
/* FP:closure.rs-0467 */         assert_eq!(child_captures.next(), None, "leftover child captures?");
/* FP:closure.rs-0468 */     }
/* FP:closure.rs-0469 */ }
/* FP:closure.rs-0470 */ 
/* FP:closure.rs-0471 */ fn child_prefix_matches_parent_projections(
/* FP:closure.rs-0472 */     parent_capture: &ty::CapturedPlace<'_>,
/* FP:closure.rs-0473 */     child_capture: &ty::CapturedPlace<'_>,
/* FP:closure.rs-0474 */ ) -> bool {
/* FP:closure.rs-0475 */     let HirPlaceBase::Upvar(parent_base) = parent_capture.place.base else {
/* FP:closure.rs-0476 */         bug!("expected capture to be an upvar");
/* FP:closure.rs-0477 */     };
/* FP:closure.rs-0478 */     let HirPlaceBase::Upvar(child_base) = child_capture.place.base else {
/* FP:closure.rs-0479 */         bug!("expected capture to be an upvar");
/* FP:closure.rs-0480 */     };
/* FP:closure.rs-0481 */ 
/* FP:closure.rs-0482 */     parent_base.var_path.hir_id == child_base.var_path.hir_id
/* FP:closure.rs-0483 */         && std::iter::zip(&child_capture.place.projections, &parent_capture.place.projections)
/* FP:closure.rs-0484 */             .all(|(child, parent)| child.kind == parent.kind)
/* FP:closure.rs-0485 */ }
/* FP:closure.rs-0486 */ 
/* FP:closure.rs-0487 */ pub fn provide(providers: &mut Providers) {
/* FP:closure.rs-0488 */     *providers = Providers { closure_typeinfo, ..*providers }
/* FP:closure.rs-0489 */ }