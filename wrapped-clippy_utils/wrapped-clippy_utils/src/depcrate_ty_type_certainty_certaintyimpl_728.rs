// Generated macro for impl_728 (impl)
macro_rules! Depcrate_ty_type_certainty_certaintyimpl_728 {
() => {
// Module: crate::ty::type_certainty::certainty
// Provides: {"impl_728"}
// Dependencies: {}
impl Meet for Option < DefId > { fn meet (self , other : Self) -> Self { match (self , other) { (None , _) | (_ , None) => None , (Some (lhs) , Some (rhs)) => (lhs == rhs) . then_some (lhs) , } } }
};
}
