// Generated macro for impl_210 (impl)
macro_rules! Depcrate_errorimpl_210 {
() => {
// Module: crate::error
// Provides: {"impl_210"}
// Dependencies: {}
impl KeyRejected { # [doc = " The value returned from `<Self as std::error::Error>::description()`"] # [must_use] pub fn description_ (& self) -> & 'static str { self . 0 } pub (crate) fn inconsistent_components () -> Self { KeyRejected ("InconsistentComponents") } # [inline] pub (crate) fn invalid_encoding () -> Self { KeyRejected ("InvalidEncoding") } pub (crate) fn too_small () -> Self { KeyRejected ("TooSmall") } pub (crate) fn too_large () -> Self { KeyRejected ("TooLarge") } pub (crate) fn wrong_algorithm () -> Self { KeyRejected ("WrongAlgorithm") } pub (crate) fn unexpected_error () -> Self { KeyRejected ("UnexpectedError") } pub (crate) fn unspecified () -> Self { KeyRejected ("Unspecified") } }
};
}
