// Generated macro for impl_560 (impl)
macro_rules! Depcrate_resimpl_560 {
() => {
// Module: crate::res
// Provides: {"impl_560"}
// Dependencies: {}
impl < 'a > MaybeResPath < 'a > for & Expr < 'a > { # [inline] fn opt_res_path (self) -> OptResPath < 'a > { match & self . kind { ExprKind :: Path (qpath) => qpath . opt_res_path () , _ => (None , None) , } } }
};
}
