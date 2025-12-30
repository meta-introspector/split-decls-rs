// Generated macro for validate_dataset (function)
macro_rules! Depcrate_validatorvalidate_dataset {
() => {
// Module: crate::validator
// Provides: {"validate_dataset"}
// Dependencies: {}
pub fn validate_dataset < D : DataAccess > (dataset : & str , data_access : D ,) -> Result < (ValidationResult , f64) , ValidationError > { let validator = DatasetValidator :: new (data_access) ; let entity = EntityIdentifier :: new_dataset (dataset . to_string ()) ; validator . validate (& entity , ValidationLevel :: Dataset) }
};
}
