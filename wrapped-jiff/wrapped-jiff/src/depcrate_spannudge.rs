// Generated macro for Nudge (struct)
macro_rules! Depcrate_spanNudge {
() => {
// Module: crate::span
// Provides: {"Nudge"}
// Dependencies: {}
# [doc = " The result of a span rounding strategy. There are three:"] # [doc = ""] # [doc = " * Rounding spans relative to civil datetimes using only invariant"] # [doc = " units (days or less). This is achieved by converting the span to a simple"] # [doc = " integer number of nanoseconds and then rounding that."] # [doc = " * Rounding spans relative to either a civil datetime or a zoned datetime"] # [doc = " where rounding might involve changing non-uniform units. That is, when"] # [doc = " the smallest unit is greater than days for civil datetimes and greater"] # [doc = " than hours for zoned datetimes."] # [doc = " * Rounding spans relative to a zoned datetime whose smallest unit is"] # [doc = " less than days."] # [doc = ""] # [doc = " Each of these might produce a bottom heavy span that needs to be"] # [doc = " re-balanced. This type represents that result via one of three constructors"] # [doc = " corresponding to each of the above strategies, and then provides a routine"] # [doc = " for rebalancing via \"bubbling.\""] # [derive (Debug)] struct Nudge { # [doc = " A possibly bottom heavy rounded span."] span : Span , # [doc = " The nanosecond timestamp corresponding to `relative + span`, where"] # [doc = " `span` is the (possibly bottom heavy) rounded span."] rounded_relative_end : NoUnits128 , # [doc = " Whether rounding may have created a bottom heavy span such that a"] # [doc = " calendar unit might need to be incremented after re-balancing smaller"] # [doc = " units."] grew_big_unit : bool , }
};
}
