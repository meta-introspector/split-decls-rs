// Generated macro for format_timestamp (function)
macro_rules! Depcrateformat_timestamp {
() => {
// Module: crate
// Provides: {"format_timestamp"}
// Dependencies: {}
# [doc = " Formats the epoch timestamp as a UTC timestamp like `\"20201-05-04T13:02:59Z\"`."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error when `epoch` is out of range."] pub fn format_timestamp (epoch : u64) -> Result < String , String > { let date_time_utc = epoch_to_date_time_utc (epoch) ? ; Ok (date_time_utc . to_rfc3339_opts (chrono :: SecondsFormat :: Secs , true)) }
};
}
