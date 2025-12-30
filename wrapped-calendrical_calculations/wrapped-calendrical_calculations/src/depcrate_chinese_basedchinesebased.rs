// Generated macro for ChineseBased (trait)
macro_rules! Depcrate_chinese_basedChineseBased {
() => {
// Module: crate::chinese_based
// Provides: {"ChineseBased"}
// Dependencies: {}
# [doc = " The trait ChineseBased is used by Chinese-based calendars to perform computations shared by such calendar."] # [doc = " To do so, calendars should:"] # [doc = ""] # [doc = " - Implement `fn location` by providing a location at which observations of the moon are recorded, which"] # [doc = "   may change over time (the zone is important, long, lat, and elevation are not relevant for these calculations)"] # [doc = " - Define `const EPOCH` as a `RataDie` marking the start date of the era of the Calendar for internal use,"] # [doc = "   which may not accurately reflect how years or eras are marked traditionally or seen by end-users"] pub trait ChineseBased { # [doc = " Given a fixed date, return the UTC offset used for observations of the new moon in order to"] # [doc = " calculate the beginning of months. For multiple Chinese-based lunar calendars, this has"] # [doc = " changed over the years, and can cause differences in calendar date."] fn utc_offset (fixed : RataDie) -> f64 ; # [doc = " The RataDie of the beginning of the epoch used for internal computation; this may not"] # [doc = " reflect traditional methods of year-tracking or eras, since Chinese-based calendars"] # [doc = " may not track years ordinally in the same way many western calendars do."] const EPOCH : RataDie ; # [doc = " The name of the calendar for debugging."] const DEBUG_NAME : & 'static str ; }
};
}
