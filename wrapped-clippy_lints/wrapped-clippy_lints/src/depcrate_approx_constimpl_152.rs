// Generated macro for impl_152 (impl)
macro_rules! Depcrate_approx_constimpl_152 {
() => {
// Module: crate::approx_const
// Provides: {"impl_152"}
// Dependencies: {}
impl LateLintPass < '_ > for ApproxConstant { fn check_lit (& mut self , cx : & LateContext < '_ > , _hir_id : HirId , lit : Lit , _negated : bool) { match lit . node { LitKind :: Float (s , LitFloatType :: Suffixed (fty)) => match fty { FloatTy :: F16 => self . check_known_consts (cx , lit . span , s , "f16") , FloatTy :: F32 => self . check_known_consts (cx , lit . span , s , "f32") , FloatTy :: F64 => self . check_known_consts (cx , lit . span , s , "f64") , FloatTy :: F128 => self . check_known_consts (cx , lit . span , s , "f128") , } , LitKind :: Float (s , LitFloatType :: Unsuffixed) => self . check_known_consts (cx , lit . span , s , "f{32, 64}") , _ => () , } } }
};
}
