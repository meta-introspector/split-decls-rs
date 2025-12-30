// Generated macro for db (function)
macro_rules! Depcrate_tz_dbdb {
() => {
// Module: crate::tz::db
// Provides: {"db"}
// Dependencies: {}
# [doc = " Returns a copy of the global [`TimeZoneDatabase`]."] # [doc = ""] # [doc = " This is the same database used for convenience routines like"] # [doc = " [`Timestamp::in_tz`](crate::Timestamp::in_tz) and parsing routines"] # [doc = " for [`Zoned`](crate::Zoned) that need to do IANA time zone identifier"] # [doc = " lookups. Basically, whenever an implicit time zone database is needed,"] # [doc = " it is *this* copy of the time zone database that is used."] # [doc = ""] # [doc = " In feature configurations where a time zone database cannot interact with"] # [doc = " the file system (like when `std` is not enabled), this returns a database"] # [doc = " where every lookup will fail."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::tz;"] # [doc = ""] # [doc = " assert!(tz::db().get(\"Antarctica/Troll\").is_ok());"] # [doc = " assert!(tz::db().get(\"does-not-exist\").is_err());"] # [doc = " ```"] pub fn db () -> & 'static TimeZoneDatabase { # [cfg (any (not (feature = "std") , miri))] { static NONE : TimeZoneDatabase = TimeZoneDatabase :: none () ; & NONE } # [cfg (all (feature = "std" , not (miri)))] { use std :: sync :: OnceLock ; static DB : OnceLock < TimeZoneDatabase > = OnceLock :: new () ; DB . get_or_init (| | { let db = TimeZoneDatabase :: from_env () ; debug ! ("initialized global time zone database: {db:?}") ; db }) } }
};
}
