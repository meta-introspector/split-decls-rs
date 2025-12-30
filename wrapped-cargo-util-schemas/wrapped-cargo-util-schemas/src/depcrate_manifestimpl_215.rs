// Generated macro for impl_215 (impl)
macro_rules! Depcrate_manifestimpl_215 {
() => {
// Module: crate::manifest
// Provides: {"impl_215"}
// Dependencies: {}
impl < T : AsRef < str > > PathBaseName < T > { # [doc = " Validated path base name"] pub fn new (name : T) -> Result < Self , NameValidationError > { restricted_names :: validate_path_base_name (name . as_ref ()) ? ; Ok (Self (name)) } }
};
}
