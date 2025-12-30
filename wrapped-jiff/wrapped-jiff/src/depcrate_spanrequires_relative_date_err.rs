// Generated macro for requires_relative_date_err (function)
macro_rules! Depcrate_spanrequires_relative_date_err {
() => {
// Module: crate::span
// Provides: {"requires_relative_date_err"}
// Dependencies: {}
fn requires_relative_date_err (unit : Unit) -> Result < () , Error > { if unit . is_variable () { return Err (if matches ! (unit , Unit :: Week | Unit :: Day) { err ! ("using unit '{unit}' in a span or configuration \
                 requires that either a relative reference time be given \
                 or `SpanRelativeTo::days_are_24_hours()` is used to \
                 indicate invariant 24-hour days, \
                 but neither were provided" , unit = unit . singular () ,) } else { err ! ("using unit '{unit}' in a span or configuration \
                 requires that a relative reference time be given, \
                 but none was provided" , unit = unit . singular () ,) }) ; } Ok (()) }
};
}
