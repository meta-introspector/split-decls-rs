// Generated macro for SECONDS_PER_CIVIL_WEEK (const)
macro_rules! Depcrate_util_tSECONDS_PER_CIVIL_WEEK {
() => {
// Module: crate::util::t
// Provides: {"SECONDS_PER_CIVIL_WEEK"}
// Dependencies: {}
# [doc = " The number of seconds in a civil week."] # [doc = ""] # [doc = " Some weeks will have more or less seconds because of DST transitions. But"] # [doc = " such things are ignored when dealing with civil time, and so this constant"] # [doc = " is still useful."] pub (crate) const SECONDS_PER_CIVIL_WEEK : Constant = Constant (DAYS_PER_CIVIL_WEEK . value () * HOURS_PER_CIVIL_DAY . value () * SECONDS_PER_HOUR . value () ,) ;
};
}
