// Generated macro for to_time (function)
macro_rules! Depcrate_mysql_types_date_and_time_timeto_time {
() => {
// Module: crate::mysql::types::date_and_time::time
// Provides: {"to_time"}
// Dependencies: {}
fn to_time (dt : MysqlTime) -> Result < NaiveTime , Box < dyn std :: error :: Error > > { for (name , field) in [("year" , dt . year) , ("month" , dt . month) , ("day" , dt . day) , ("offset" , dt . time_zone_displacement . try_into () ?) ,] { if field != 0 { return Err (format ! ("Unable to convert {dt:?} to time: {name} must be 0") . into ()) ; } } let hour : u8 = dt . hour . try_into () ? ; let minute : u8 = dt . minute . try_into () ? ; let second : u8 = dt . second . try_into () ? ; let microsecond : u32 = dt . second_part . try_into () ? ; Ok (NaiveTime :: from_hms_micro (hour , minute , second , microsecond ,) ?) }
};
}
