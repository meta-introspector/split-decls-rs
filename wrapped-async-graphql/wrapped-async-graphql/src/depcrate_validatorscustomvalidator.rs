// Generated macro for CustomValidator (trait)
macro_rules! Depcrate_validatorsCustomValidator {
() => {
// Module: crate::validators
// Provides: {"CustomValidator"}
// Dependencies: {}
# [doc = " Represents a custom input value validator."] pub trait CustomValidator < T : InputType > { # [doc = " Check the value is valid."] fn check (& self , value : & T) -> Result < () , InputValueError < T > > ; }
};
}
