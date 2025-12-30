// Generated macro for TableTransitions (trait)
macro_rules! Depcrate_transitionsTableTransitions {
() => {
// Module: crate::transitions
// Provides: {"TableTransitions"}
// Dependencies: {}
# [doc = " Trait to put the `timespans` method on Tables."] pub trait TableTransitions { # [doc = " Computes a fixed timespan set for the timezone with the given name."] # [doc = " Returns `None` if the table doesn’t contain a time zone with that name."] fn timespans (& self , zone_name : & str) -> Option < FixedTimespanSet > ; }
};
}
