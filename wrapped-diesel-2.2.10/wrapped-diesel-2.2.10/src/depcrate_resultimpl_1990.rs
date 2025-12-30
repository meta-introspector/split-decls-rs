// Generated macro for impl_1990 (impl)
macro_rules! Depcrate_resultimpl_1990 {
() => {
// Module: crate::result
// Provides: {"impl_1990"}
// Dependencies: {}
impl StdError for DeserializeFieldError { fn source (& self) -> Option < & (dyn StdError + 'static) > { Some (& * self . error) } }
};
}
