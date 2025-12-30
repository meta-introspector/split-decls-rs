// Generated macro for impl_190 (impl)
macro_rules! Depcrate_max_sizeimpl_190 {
() => {
// Module: crate::max_size
// Provides: {"impl_190"}
// Dependencies: {}
impl < T : MaxSize , E : MaxSize > MaxSize for Result < T , E > { const POSTCARD_MAX_SIZE : usize = max (T :: POSTCARD_MAX_SIZE , E :: POSTCARD_MAX_SIZE) + 1 ; }
};
}
