// Generated macro for MINUTES_PER_CIVIL_DAY (const)
macro_rules! Depcrate_util_tMINUTES_PER_CIVIL_DAY {
() => {
// Module: crate::util::t
// Provides: {"MINUTES_PER_CIVIL_DAY"}
// Dependencies: {}
# [doc = " The number of minutes in a civil day."] pub (crate) const MINUTES_PER_CIVIL_DAY : Constant = Constant (HOURS_PER_CIVIL_DAY . value () * MINUTES_PER_HOUR . value ()) ;
};
}
