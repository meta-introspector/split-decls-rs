// Generated macro for impl_561 (impl)
macro_rules! Depcrate_resimpl_561 {
() => {
// Module: crate::res
// Provides: {"impl_561"}
// Dependencies: {}
impl < 'a > MaybeResPath < 'a > for & PatExpr < 'a > { # [inline] fn opt_res_path (self) -> OptResPath < 'a > { match & self . kind { PatExprKind :: Path (qpath) => qpath . opt_res_path () , _ => (None , None) , } } }
};
}
