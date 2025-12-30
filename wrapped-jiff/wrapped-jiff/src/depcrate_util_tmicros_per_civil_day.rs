// Generated macro for MICROS_PER_CIVIL_DAY (const)
macro_rules! Depcrate_util_tMICROS_PER_CIVIL_DAY {
() => {
// Module: crate::util::t
// Provides: {"MICROS_PER_CIVIL_DAY"}
// Dependencies: {}
# [doc = " The number of microseconds in a civil day."] pub (crate) const MICROS_PER_CIVIL_DAY : Constant = Constant (SECONDS_PER_CIVIL_DAY . value () * MICROS_PER_SECOND . value ()) ;
};
}
