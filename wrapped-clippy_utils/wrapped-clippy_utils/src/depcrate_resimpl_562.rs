// Generated macro for impl_562 (impl)
macro_rules! Depcrate_resimpl_562 {
() => {
// Module: crate::res
// Provides: {"impl_562"}
// Dependencies: {}
impl < 'a , AmbigArg > MaybeResPath < 'a > for & hir :: Ty < 'a , AmbigArg > { # [inline] fn opt_res_path (self) -> OptResPath < 'a > { match & self . kind { TyKind :: Path (qpath) => qpath . opt_res_path () , _ => (None , None) , } } }
};
}
