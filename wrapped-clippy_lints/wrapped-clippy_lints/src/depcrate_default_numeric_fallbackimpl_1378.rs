// Generated macro for impl_1378 (impl)
macro_rules! Depcrate_default_numeric_fallbackimpl_1378 {
() => {
// Module: crate::default_numeric_fallback
// Provides: {"impl_1378"}
// Dependencies: {}
impl < 'a , 'tcx > NumericFallbackVisitor < 'a , 'tcx > { fn new (cx : & 'a LateContext < 'tcx > , is_parent_const : bool) -> Self { Self { ty_bounds : vec ! [if is_parent_const { ExplicitTyBound (true) } else { ExplicitTyBound (false) }] , cx , } } # [doc = " Check whether a passed literal has potential to cause fallback or not."] fn check_lit (& self , lit : Lit , lit_ty : Ty < 'tcx > , emit_hir_id : HirId) { if ! lit . span . in_external_macro (self . cx . sess () . source_map ()) && matches ! (self . ty_bounds . last () , Some (ExplicitTyBound (false))) && matches ! (lit . node , LitKind :: Int (_ , LitIntType :: Unsuffixed) | LitKind :: Float (_ , LitFloatType :: Unsuffixed)) { let (suffix , is_float) = match lit_ty . kind () { ty :: Int (IntTy :: I32) => ("i32" , false) , ty :: Float (FloatTy :: F64) => ("f64" , true) , _ => return , } ; span_lint_hir_and_then (self . cx , DEFAULT_NUMERIC_FALLBACK , emit_hir_id , lit . span , "default numeric fallback might occur" , | diag | { let src = if let Some (src) = snippet_opt (self . cx , lit . span) { src } else { match lit . node { LitKind :: Int (src , _) => format ! ("{src}") , LitKind :: Float (src , _) => format ! ("{src}") , _ => unreachable ! ("Default numeric fallback never results in other types") , } } ; let sugg = numeric_literal :: format (& src , Some (suffix) , is_float) ; diag . span_suggestion (lit . span , "consider adding suffix" , sugg , Applicability :: MaybeIncorrect) ; } ,) ; } } }
};
}
