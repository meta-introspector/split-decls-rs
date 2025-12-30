// Generated macro for zoned_date_time (module)
macro_rules! Depcrate_integrations_jiffzoned_date_time {
() => {
// Module: crate::integrations::jiff
// Provides: {"zoned_date_time"}
// Dependencies: {}
mod zoned_date_time { use super :: ZonedDateTime ; pub (super) fn from_input (s : & str) -> Result < ZonedDateTime , Box < str > > { s . parse () . map_err (| e | format ! ("Invalid `ZonedDateTime`: {e}") . into ()) } }
};
}
