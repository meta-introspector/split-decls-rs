// Generated macro for validate_config (function)
macro_rules! Depcrate_validatorvalidate_config {
() => {
// Module: crate::validator
// Provides: {"validate_config"}
// Dependencies: {}
pub fn validate_config < D : DataAccess > (dataset : & str , config : & str , data_access : D ,) -> Result < (ValidationResult , f64) , ValidationError > { let validator = DatasetValidator :: new (data_access) ; let entity = EntityIdentifier :: new_config (dataset . to_string () , config . to_string ()) ; validator . validate (& entity , ValidationLevel :: Config) }
};
}
