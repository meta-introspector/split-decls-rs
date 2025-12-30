// Generated macro for epoch_to_date_time_utc (function)
macro_rules! Depcrateepoch_to_date_time_utc {
() => {
// Module: crate
// Provides: {"epoch_to_date_time_utc"}
// Dependencies: {}
# [doc = " Converts the epoch timestamp to a UTC time."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error when `epoch` is out of range."] fn epoch_to_date_time_utc (epoch : u64) -> Result < chrono :: DateTime < Utc > , String > { let epoch_i64 = i64 :: try_from (epoch) . map_err (| _ | format ! ("epoch is out of range: {epoch}")) ? ; chrono :: TimeZone :: timestamp_opt (& Utc , epoch_i64 , 0) . earliest () . ok_or_else (| | format ! ("failed to convert epoch to UTC timestamp: {epoch}")) }
};
}
