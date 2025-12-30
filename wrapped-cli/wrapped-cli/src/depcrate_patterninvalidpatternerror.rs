// Generated macro for InvalidPatternError (struct)
macro_rules! Depcrate_patternInvalidPatternError {
() => {
// Module: crate::pattern
// Provides: {"InvalidPatternError"}
// Dependencies: {}
# [doc = " An error that occurs when a pattern could not be converted to valid UTF-8."] # [doc = ""] # [doc = " The purpose of this error is to give a more targeted failure mode for"] # [doc = " patterns written by end users that are not valid UTF-8."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct InvalidPatternError { original : String , valid_up_to : usize , }
};
}
