// Generated macro for impl_760 (impl)
macro_rules! Depcrate_ty_type_certainty_certaintyimpl_760 {
() => {
// Module: crate::ty::type_certainty::certainty
// Provides: {"impl_760"}
// Dependencies: {}
impl Meet for Option < DefId > { fn meet (self , other : Self) -> Self { match (self , other) { (None , _) | (_ , None) => None , (Some (lhs) , Some (rhs)) => (lhs == rhs) . then_some (lhs) , } } }
};
}
