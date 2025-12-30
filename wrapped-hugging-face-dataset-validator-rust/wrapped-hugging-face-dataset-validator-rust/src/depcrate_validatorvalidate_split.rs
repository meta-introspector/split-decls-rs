// Generated macro for validate_split (function)
macro_rules! Depcrate_validatorvalidate_split {
() => {
// Module: crate::validator
// Provides: {"validate_split"}
// Dependencies: {}
pub fn validate_split < D : DataAccess > (dataset : & str , config : & str , split : & str , data_access : D ,) -> Result < (ValidationResult , f64) , ValidationError > { let validator = DatasetValidator :: new (data_access) ; let entity = EntityIdentifier :: new_split (dataset . to_string () , config . to_string () , split . to_string ()) ; validator . validate (& entity , ValidationLevel :: Split) }
};
}
