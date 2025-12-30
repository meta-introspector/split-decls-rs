// Generated macro for parse_ex_data (function)
macro_rules! Depcrate_recordreplay_qlogparse_ex_data {
() => {
// Module: crate::recordreplay::qlog
// Provides: {"parse_ex_data"}
// Dependencies: {}
fn parse_ex_data (ex_data : & ExData) -> bool { ex_data . get ("fin_stream") . unwrap_or (& serde_json :: Value :: Null) . as_bool () . unwrap_or_default () }
};
}
