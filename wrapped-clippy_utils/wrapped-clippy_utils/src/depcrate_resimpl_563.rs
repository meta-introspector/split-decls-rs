// Generated macro for impl_563 (impl)
macro_rules! Depcrate_resimpl_563 {
() => {
// Module: crate::res
// Provides: {"impl_563"}
// Dependencies: {}
impl < 'a > MaybeResPath < 'a > for & Pat < 'a > { # [inline] fn opt_res_path (self) -> OptResPath < 'a > { match self . kind { PatKind :: Expr (e) => e . opt_res_path () , _ => (None , None) , } } }
};
}
