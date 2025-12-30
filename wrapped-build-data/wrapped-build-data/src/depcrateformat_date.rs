// Generated macro for format_date (function)
macro_rules! Depcrateformat_date {
() => {
// Module: crate
// Provides: {"format_date"}
// Dependencies: {}
# [doc = " Formats the epoch timestamp as a UTC date like `\"2021-05-04Z\"`."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error when `epoch` is out of range."] pub fn format_date (epoch : u64) -> Result < String , String > { let date_time_utc = epoch_to_date_time_utc (epoch) ? ; Ok (date_time_utc . format ("%Y-%m-%dZ") . to_string ()) }
};
}
