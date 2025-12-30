// Generated macro for ValidationMode (enum)
macro_rules! Depcrate_machineValidationMode {
() => {
// Module: crate::machine
// Provides: {"ValidationMode"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum ValidationMode { # [doc = " Do not perform any kind of validation."] No , # [doc = " Validate the interior of the value, but not things behind references."] Shallow , # [doc = " Fully recursively validate references."] Deep , }
};
}
