// Generated macro for to_datetime (function)
macro_rules! Depcrate_mysql_types_date_and_time_timeto_datetime {
() => {
// Module: crate::mysql::types::date_and_time::time
// Provides: {"to_datetime"}
// Dependencies: {}
fn to_datetime (dt : MysqlTime) -> Result < OffsetDateTime , Box < dyn std :: error :: Error > > { let year : i32 = dt . year . try_into () ? ; let month : u8 = dt . month . try_into () ? ; let month : Month = month . try_into () ? ; let day : u8 = dt . day . try_into () ? ; let hour : u8 = dt . hour . try_into () ? ; let minute : u8 = dt . minute . try_into () ? ; let second : u8 = dt . second . try_into () ? ; let microsecond : u32 = dt . second_part . try_into () ? ; let offset = UtcOffset :: from_whole_seconds (dt . time_zone_displacement) ? ; Ok (PrimitiveDateTime :: new (NaiveDate :: from_calendar_date (year , month , day) ? , NaiveTime :: from_hms_micro (hour , minute , second , microsecond) ? ,) . assume_offset (offset)) }
};
}
