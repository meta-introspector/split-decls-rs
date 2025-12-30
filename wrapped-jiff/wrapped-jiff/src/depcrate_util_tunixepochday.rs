// Generated macro for UnixEpochDay (type)
macro_rules! Depcrate_util_tUnixEpochDay {
() => {
// Module: crate::util::t
// Provides: {"UnixEpochDay"}
// Dependencies: {}
# [doc = " The number of days from the Unix epoch for the Gregorian calendar."] # [doc = ""] # [doc = " The range supported is based on the range of Unix timestamps that we"] # [doc = " support."] # [doc = ""] # [doc = " While I had originally used the \"rate die\" concept from Calendrical"] # [doc = " Calculations, I found [Howard Hinnant's formulation][date-algorithms]"] # [doc = " much more straight-forward. And while I didn't benchmark them, it also"] # [doc = " appears faster."] # [doc = ""] # [doc = " [date-algorithms]: http://howardhinnant.github.io/date_algorithms.html"] pub (crate) type UnixEpochDay = ri32 < { (UnixSeconds :: MIN + SpanZoneOffset :: MIN) . div_euclid (SECONDS_PER_CIVIL_DAY . bound ()) } , { (UnixSeconds :: MAX + SpanZoneOffset :: MAX) . div_euclid (SECONDS_PER_CIVIL_DAY . bound ()) } , > ;
};
}
