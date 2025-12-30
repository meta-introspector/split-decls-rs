// Generated macro for impl_209 (impl)
macro_rules! Depcrate_manifestimpl_209 {
() => {
// Module: crate::manifest
// Provides: {"impl_209"}
// Dependencies: {}
impl < T : AsRef < str > > RegistryName < T > { # [doc = " Validated registry name"] pub fn new (name : T) -> Result < Self , NameValidationError > { restricted_names :: validate_registry_name (name . as_ref ()) ? ; Ok (Self (name)) } }
};
}
