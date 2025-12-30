// Generated macro for clamp_relative_span (function)
macro_rules! Depcrate_spanclamp_relative_span {
() => {
// Module: crate::span
// Provides: {"clamp_relative_span"}
// Dependencies: {}
# [doc = " Returns the nanosecond timestamps of `relative + span` and `relative +"] # [doc = " {amount of unit} + span`."] # [doc = ""] # [doc = " This is useful for determining the actual length, in nanoseconds, of some"] # [doc = " unit amount (usually a single unit). Usually, this is called with a span"] # [doc = " whose units lower than `unit` are zeroed out and with an `amount` that"] # [doc = " is `-1` or `1` or `0`. So for example, if `unit` were `Unit::Day`, then"] # [doc = " you'd get back two nanosecond timestamps relative to the relative datetime"] # [doc = " given that start exactly \"one day\" apart. (Which might be different than 24"] # [doc = " hours, depending on the time zone.)"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This returns an error if adding the units overflows, or if doing the span"] # [doc = " arithmetic on `relative` overflows."] fn clamp_relative_span (relative : & Relative < '_ > , span : Span , unit : Unit , amount : NoUnits ,) -> Result < (NoUnits128 , NoUnits128) , Error > { let amount = span . get_units_ranged (unit) . try_checked_add ("clamp-units" , amount) . with_context (| | { err ! ("failed to add {amount} to {unit} \
                 value {value} on span {span}" , unit = unit . plural () , value = span . get_units_ranged (unit) ,) }) ? ; let span_amount = span . try_units_ranged (unit , amount) . with_context (| | { err ! ("failed to set {unit} unit to {amount} on span {span}" , unit = unit . plural () ,) }) ? ; let relative0 = relative . checked_add (span) ? . to_nanosecond () ; let relative1 = relative . checked_add (span_amount) ? . to_nanosecond () ; Ok ((relative0 , relative1)) }
};
}
