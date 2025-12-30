// Generated macro for tz (module)
macro_rules! Depcrate_integrations_chrono_tztz {
() => {
// Module: crate::integrations::chrono_tz
// Provides: {"tz"}
// Dependencies: {}
mod tz { use super :: TimeZone ; pub (super) fn to_output (v : & TimeZone) -> & 'static str { v . name () } pub (super) fn from_input (s : & str) -> Result < TimeZone , Box < str > > { s . parse :: < TimeZone > () . map_err (| e | format ! ("Failed to parse `TimeZone`: {e}") . into ()) } }
};
}
