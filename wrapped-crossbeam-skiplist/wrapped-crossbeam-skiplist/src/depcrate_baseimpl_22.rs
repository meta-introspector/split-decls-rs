// Generated macro for impl_22 (impl)
macro_rules! Depcrate_baseimpl_22 {
() => {
// Module: crate::base
// Provides: {"impl_22"}
// Dependencies: {}
impl < K , V > Deref for Head < K , V > { type Target = Tower < K , V > ; fn deref (& self) -> & Tower < K , V > { unsafe { & * (self as * const _ as * const Tower < K , V >) } } }
};
}
