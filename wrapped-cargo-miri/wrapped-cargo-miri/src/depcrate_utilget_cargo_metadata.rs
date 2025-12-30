// Generated macro for get_cargo_metadata (function)
macro_rules! Depcrate_utilget_cargo_metadata {
() => {
// Module: crate::util
// Provides: {"get_cargo_metadata"}
// Dependencies: {}
pub fn get_cargo_metadata () -> Metadata { MetadataCommand :: new () . no_deps () . other_options (cargo_extra_flags ()) . exec () . unwrap () }
};
}
