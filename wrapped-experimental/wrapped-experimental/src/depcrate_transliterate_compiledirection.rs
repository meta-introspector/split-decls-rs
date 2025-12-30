// Generated macro for Direction (enum)
macro_rules! Depcrate_transliterate_compileDirection {
() => {
// Module: crate::transliterate::compile
// Provides: {"Direction"}
// Dependencies: {}
# [doc = " The direction of a rule-based transliterator in respect to its source."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub (crate) enum Direction { # [doc = " Forwards, i.e., left-to-right in the source."] Forward , # [doc = " Reverse, i.e., right-to-left in the source."] Reverse , # [doc = " Both forwards and reverse."] Both , }
};
}
