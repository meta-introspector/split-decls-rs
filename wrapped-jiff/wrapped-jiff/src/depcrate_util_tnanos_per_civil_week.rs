// Generated macro for NANOS_PER_CIVIL_WEEK (const)
macro_rules! Depcrate_util_tNANOS_PER_CIVIL_WEEK {
() => {
// Module: crate::util::t
// Provides: {"NANOS_PER_CIVIL_WEEK"}
// Dependencies: {}
# [doc = " The number of nanoseconds in a civil week."] # [doc = ""] # [doc = " Some weeks will have more or less seconds because of DST transitions. But"] # [doc = " such things are ignored when dealing with civil time, and so this constant"] # [doc = " is still useful."] pub (crate) const NANOS_PER_CIVIL_WEEK : Constant = Constant (SECONDS_PER_CIVIL_WEEK . value () * NANOS_PER_SECOND . value ()) ;
};
}
