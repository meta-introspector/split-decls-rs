// Generated macro for Year (enum)
macro_rules! Depcrate_lineYear {
() => {
// Module: crate::line
// Provides: {"Year"}
// Dependencies: {}
# [doc = " A **year** definition field."] # [doc = ""] # [doc = " A year has one of the following representations in a file:"] # [doc = ""] # [doc = " - `min` or `minimum`, the minimum year possible, for when a rule needs to"] # [doc = "   apply up until the first rule with a specific year;"] # [doc = " - `max` or `maximum`, the maximum year possible, for when a rule needs to"] # [doc = "   apply after the last rule with a specific year;"] # [doc = " - a year number, referring to a specific year."] # [derive (PartialEq , Debug , Copy , Clone)] pub enum Year { # [doc = " The minimum year possible: `min` or `minimum`."] Minimum , # [doc = " The maximum year possible: `max` or `maximum`."] Maximum , # [doc = " A specific year number."] Number (i64) , }
};
}
