// Generated macro for impl_11 (impl)
macro_rules! Depcrate_common_defaultimpl_11 {
() => {
// Module: crate::common::default
// Provides: {"impl_11"}
// Dependencies: {}
impl From < Option < syn :: Expr > > for Value { fn from (opt : Option < syn :: Expr >) -> Self { match opt { Some (expr) => Self :: Expr (Box :: new (expr)) , None => Self :: Default , } } }
};
}
