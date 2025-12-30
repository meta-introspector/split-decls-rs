// Generated macro for impl_14 (impl)
macro_rules! Depcrate_validatorimpl_14 {
() => {
// Module: crate::validator
// Provides: {"impl_14"}
// Dependencies: {}
impl CachedResponse { pub fn new (status : u16 , content : ValidationResult , progress : f64) -> Self { Self { http_status : status , content , progress , } } pub fn is_success (& self) -> bool { self . http_status == 200 } }
};
}
