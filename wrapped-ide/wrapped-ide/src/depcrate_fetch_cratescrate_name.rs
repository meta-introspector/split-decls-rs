// Generated macro for crate_name (function)
macro_rules! Depcrate_fetch_cratescrate_name {
() => {
// Module: crate::fetch_crates
// Provides: {"crate_name"}
// Dependencies: {}
fn crate_name (data : & ide_db :: base_db :: ExtraCrateData) -> Option < String > { data . display_name . as_ref () . map (| it | it . canonical_name () . as_str () . to_owned ()) }
};
}
