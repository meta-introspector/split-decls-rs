// Generated macro for impl_serializable_state_u64_array (macro)
macro_rules! Depcrate_hazmatimpl_serializable_state_u64_array {
() => {
// Module: crate::hazmat
// Provides: {"impl_serializable_state_u64_array"}
// Dependencies: {}
macro_rules ! impl_serializable_state_u64_array { ($ ($ n : ty) ,*) => { $ (impl_serializable_state_type_array ! (u64 , U8 , $ n) ;) * } ; }
};
}
