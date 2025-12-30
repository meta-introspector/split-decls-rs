// Generated macro for impl_211 (impl)
macro_rules! Depcrate_manifestimpl_211 {
() => {
// Module: crate::manifest
// Provides: {"impl_211"}
// Dependencies: {}
impl < T : AsRef < str > > ProfileName < T > { # [doc = " Validated profile name"] pub fn new (name : T) -> Result < Self , NameValidationError > { restricted_names :: validate_profile_name (name . as_ref ()) ? ; Ok (Self (name)) } }
};
}
