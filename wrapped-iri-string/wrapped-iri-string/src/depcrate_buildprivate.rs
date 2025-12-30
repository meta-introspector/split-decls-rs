// Generated macro for private (module)
macro_rules! Depcrate_buildprivate {
() => {
// Module: crate::build
// Provides: {"private"}
// Dependencies: {}
# [doc = " Private module to put the trait to seal."] mod private { use super :: { Builder , Built , Error } ; # [doc = " A trait for types buildable by the [`Builder`]."] pub trait Sealed < 'a > { # [doc = " Validates the content of the builder and returns the validated type if possible."] fn validate_builder (builder : Builder < 'a >) -> Result < Built < 'a , Self > , Error > ; } }
};
}
