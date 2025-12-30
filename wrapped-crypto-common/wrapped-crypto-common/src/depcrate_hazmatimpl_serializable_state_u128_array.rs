// Generated macro for impl_serializable_state_u128_array (macro)
macro_rules! Depcrate_hazmatimpl_serializable_state_u128_array {
() => {
// Module: crate::hazmat
// Provides: {"impl_serializable_state_u128_array"}
// Dependencies: {}
macro_rules ! impl_serializable_state_u128_array { ($ ($ n : ty) ,*) => { $ (impl_serializable_state_type_array ! (u128 , U8 , $ n) ;) * } ; }
};
}
