// Generated macro for FixedTimespanSet (struct)
macro_rules! Depcrate_transitionsFixedTimespanSet {
() => {
// Module: crate::transitions
// Provides: {"FixedTimespanSet"}
// Dependencies: {}
# [doc = " A set of timespans, separated by the instances at which the timespans"] # [doc = " change over. There will always be one more timespan than transitions."] # [doc = ""] # [doc = " This mimics the `FixedTimespanSet` struct in `datetime::cal::zone`,"] # [doc = " except it uses owned `Vec`s instead of slices."] # [derive (PartialEq , Debug , Clone)] pub struct FixedTimespanSet { # [doc = " The first timespan, which is assumed to have been in effect up until"] # [doc = " the initial transition instant (if any). Each set has to have at"] # [doc = " least one timespan."] pub first : FixedTimespan , # [doc = " The rest of the timespans, as a vector of tuples, each containing:"] # [doc = ""] # [doc = " 1. A transition instant at which the previous timespan ends and the"] # [doc = "    next one begins, stored as a Unix timestamp;"] # [doc = " 2. The actual timespan to transition into."] pub rest : Vec < (i64 , FixedTimespan) > , }
};
}
