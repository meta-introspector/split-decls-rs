// Generated macro for impl_184 (impl)
macro_rules! Depcrate_max_sizeimpl_184 {
() => {
// Module: crate::max_size
// Provides: {"impl_184"}
// Dependencies: {}
impl < T : MaxSize , E : MaxSize > MaxSize for Result < T , E > { const POSTCARD_MAX_SIZE : usize = max (T :: POSTCARD_MAX_SIZE , E :: POSTCARD_MAX_SIZE) + 1 ; }
};
}
