// Generated macro for ZonedDaySeconds (type)
macro_rules! Depcrate_util_tZonedDaySeconds {
() => {
// Module: crate::util::t
// Provides: {"ZonedDaySeconds"}
// Dependencies: {}
# [doc = " The number of seconds permitted in a single day."] # [doc = ""] # [doc = " This is mostly just a \"sensible\" cap on what is possible. We allow one day"] # [doc = " to span up to 7 civil days."] # [doc = ""] # [doc = " It must also be at least 1 second long."] pub (crate) type ZonedDaySeconds = ri64 < 1 , { 7 * SECONDS_PER_CIVIL_DAY . bound () } > ;
};
}
