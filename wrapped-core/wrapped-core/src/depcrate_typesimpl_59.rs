// Generated macro for impl_59 (impl)
macro_rules! Depcrate_typesimpl_59 {
() => {
// Module: crate::types
// Provides: {"impl_59"}
// Dependencies: {}
impl std :: ops :: BitOrAssign for TypeInfo { fn bitor_assign (& mut self , rhs : Self) { self . borrowed |= rhs . borrowed ; self . owned |= rhs . owned ; self . error |= rhs . error ; self . has_list |= rhs . has_list ; self . has_tuple |= rhs . has_tuple ; self . has_resource |= rhs . has_resource ; self . has_borrow_handle |= rhs . has_borrow_handle ; self . has_own_handle |= rhs . has_own_handle ; } }
};
}
