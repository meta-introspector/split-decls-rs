// Generated macro for ZonedDayNanoseconds (type)
macro_rules! Depcrate_util_tZonedDayNanoseconds {
() => {
// Module: crate::util::t
// Provides: {"ZonedDayNanoseconds"}
// Dependencies: {}
# [doc = " The number of nanoseconds permitted in a single day."] # [doc = ""] # [doc = " This is mostly just a \"sensible\" cap on what is possible. We allow one day"] # [doc = " to span up to 7 civil days."] # [doc = ""] # [doc = " It must also be at least 1 second long."] pub (crate) type ZonedDayNanoseconds = ri64 < { ZonedDaySeconds :: MIN * NANOS_PER_SECOND . bound () } , { ZonedDaySeconds :: MAX * NANOS_PER_SECOND . bound () } , > ;
};
}
