// Generated macro for format_time (function)
macro_rules! Depcrateformat_time {
() => {
// Module: crate
// Provides: {"format_time"}
// Dependencies: {}
# [doc = " Formats the epoch timestamp as a UTC time like `\"13:02:59Z\"`."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error when `epoch` is out of range."] pub fn format_time (epoch : u64) -> Result < String , String > { let date_time_utc = epoch_to_date_time_utc (epoch) ? ; Ok (date_time_utc . format ("%H:%M:%SZ") . to_string ()) }
};
}
