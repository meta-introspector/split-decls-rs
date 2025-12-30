// Generated macro for impl_206 (impl)
macro_rules! Depcrate_manifestimpl_206 {
() => {
// Module: crate::manifest
// Provides: {"impl_206"}
// Dependencies: {}
impl < T : AsRef < str > > PackageName < T > { # [doc = " Validated package name"] pub fn new (name : T) -> Result < Self , NameValidationError > { restricted_names :: validate_package_name (name . as_ref ()) ? ; Ok (Self (name)) } }
};
}
