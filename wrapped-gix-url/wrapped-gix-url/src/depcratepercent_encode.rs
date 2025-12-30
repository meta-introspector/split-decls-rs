// Generated macro for percent_encode (function)
macro_rules! Depcratepercent_encode {
() => {
// Module: crate
// Provides: {"percent_encode"}
// Dependencies: {}
fn percent_encode (s : & str) -> Cow < '_ , str > { percent_encoding :: utf8_percent_encode (s , percent_encoding :: NON_ALPHANUMERIC) . into () }
};
}
