// Generated macro for impl_559 (impl)
macro_rules! Depcrate_resimpl_559 {
() => {
// Module: crate::res
// Provides: {"impl_559"}
// Dependencies: {}
impl < 'a > MaybeResPath < 'a > for & QPath < 'a > { # [inline] fn opt_res_path (self) -> OptResPath < 'a > { match * self { QPath :: Resolved (ty , path) => (ty , Some (path)) , QPath :: TypeRelative (..) => (None , None) , } } }
};
}
