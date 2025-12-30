// Generated macro for GeneratorError (enum)
macro_rules! Depcrate_utilsGeneratorError {
() => {
// Module: crate::utils
// Provides: {"GeneratorError"}
// Dependencies: {}
# [derive (Error , Debug)] pub enum GeneratorError { # [error ("{0}")] Syn (# [from] syn :: Error) , # [error ("{0}")] Darling (# [from] darling :: Error) , }
};
}
