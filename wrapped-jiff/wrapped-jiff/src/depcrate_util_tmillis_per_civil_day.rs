// Generated macro for MILLIS_PER_CIVIL_DAY (const)
macro_rules! Depcrate_util_tMILLIS_PER_CIVIL_DAY {
() => {
// Module: crate::util::t
// Provides: {"MILLIS_PER_CIVIL_DAY"}
// Dependencies: {}
# [doc = " The number of microseconds in a civil day."] pub (crate) const MILLIS_PER_CIVIL_DAY : Constant = Constant (SECONDS_PER_CIVIL_DAY . value () * MILLIS_PER_SECOND . value ()) ;
};
}
