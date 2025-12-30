// Generated macro for duration (module)
macro_rules! Depcrate_integrations_jiffduration {
() => {
// Module: crate::integrations::jiff
// Provides: {"duration"}
// Dependencies: {}
mod duration { use super :: Duration ; pub (super) fn from_input (s : & str) -> Result < Duration , Box < str > > { s . parse () . map_err (| e | format ! ("Invalid `Duration`: {e}") . into ()) } }
};
}
