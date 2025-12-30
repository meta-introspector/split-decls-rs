// Generated macro for macro_18 (macro)
macro_rules! Depcrate_errorsmacro_18 {
() => {
// Module: crate::errors
// Provides: {"macro_18"}
// Dependencies: {}
simple ! { # [doc = " Error returned when a slice of `u16`s doesn't start with valid UTF-16."] Utf16SliceError { # [doc = " The slice is empty."] EmptySlice => "the slice is empty" , # [doc = " The first unit is a trailing surrogate."] FirstIsTrailingSurrogate => "the first unit is a trailing surrogate" , # [doc = " The first and only unit requires a second unit."] MissingSecond => "the first and only unit requires a second one" , # [doc = " The first unit requires a second one, but it's not a trailing surrogate."] SecondIsNotTrailingSurrogate => "the required second unit is not a trailing surrogate" , } }
};
}
