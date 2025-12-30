// Generated macro for macro_17 (macro)
macro_rules! Depcrate_errorsmacro_17 {
() => {
// Module: crate::errors
// Provides: {"macro_17"}
// Dependencies: {}
simple ! { # [doc = " Error returned when one or two `u16`s are not valid UTF-16."] # [doc = ""] # [doc = " They are returned in sinking precedence;"] # [doc = " The condition that causes the first variant to be returned is checked"] # [doc = " for before the condition the next variant is returned for."] Utf16TupleError { # [doc = " The first unit is a trailing / low surrogate, which is never valid."] FirstIsTrailingSurrogate => "the first unit is a trailing surrogate" , # [doc = " The provided second unit is not necessary."] SuperfluousSecond => "the second unit is superfluous" , # [doc = " The first and only unit requires a second unit."] MissingSecond => "the first unit requires a second unit" , # [doc = " The second unit is needed and was provided, but is not a trailing surrogate."] SecondIsNotTrailingSurrogate => "the required second unit is not a trailing surrogate" , } }
};
}
