// Generated macro for impl_213 (impl)
macro_rules! Depcrate_manifestimpl_213 {
() => {
// Module: crate::manifest
// Provides: {"impl_213"}
// Dependencies: {}
impl < T : AsRef < str > > FeatureName < T > { # [doc = " Validated feature name"] pub fn new (name : T) -> Result < Self , NameValidationError > { restricted_names :: validate_feature_name (name . as_ref ()) ? ; Ok (Self (name)) } }
};
}
