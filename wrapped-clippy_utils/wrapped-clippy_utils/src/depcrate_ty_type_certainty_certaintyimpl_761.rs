// Generated macro for impl_761 (impl)
macro_rules! Depcrate_ty_type_certainty_certaintyimpl_761 {
() => {
// Module: crate::ty::type_certainty::certainty
// Provides: {"impl_761"}
// Dependencies: {}
impl TryJoin for Option < DefId > { fn try_join (self , other : Self) -> Option < Self > { match (self , other) { (Some (lhs) , Some (rhs)) => (lhs == rhs) . then_some (Some (lhs)) , (Some (def_id) , _) | (_ , Some (def_id)) => Some (Some (def_id)) , (None , None) => Some (None) , } } }
};
}
