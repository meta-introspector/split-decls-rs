// Generated macro for round_span_invariant (function)
macro_rules! Depcrate_spanround_span_invariant {
() => {
// Module: crate::span
// Provides: {"round_span_invariant"}
// Dependencies: {}
# [doc = " Rounds a span consisting of only invariant units."] # [doc = ""] # [doc = " This only applies when the max of the units in the span being rounded,"] # [doc = " the largest configured unit and the smallest configured unit are all"] # [doc = " invariant. That is, days or lower for spans without a relative datetime or"] # [doc = " a relative civil datetime, and hours or lower for spans with a relative"] # [doc = " zoned datetime."] # [doc = ""] # [doc = " All we do here is convert the span to an integer number of nanoseconds,"] # [doc = " round that and then convert back. There aren't any tricky corner cases to"] # [doc = " consider here."] fn round_span_invariant (span : Span , smallest : Unit , largest : Unit , increment : NoUnits128 , mode : RoundMode ,) -> Result < Span , Error > { assert ! (smallest <= Unit :: Week) ; assert ! (largest <= Unit :: Week) ; let nanos = span . to_invariant_nanoseconds () ; let rounded = mode . round_by_unit_in_nanoseconds (nanos , smallest , increment) ; Span :: from_invariant_nanoseconds (largest , rounded) . with_context (| | { err ! ("failed to convert rounded nanoseconds {rounded} \
             to span for largest unit as {unit}" , unit = largest . plural () ,) }) }
};
}
