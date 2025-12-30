// Generated macro for time_zone (module)
macro_rules! Depcrate_integrations_jifftime_zone {
() => {
// Module: crate::integrations::jiff
// Provides: {"time_zone"}
// Dependencies: {}
mod time_zone { use super :: TimeZone ; pub (super) fn from_input (s : & str) -> Result < TimeZone , Box < str > > { s . parse () . map_err (| e | format ! ("Invalid `TimeZone`: {e}") . into ()) } }
};
}
