// Generated macro for impl_76 (impl)
macro_rules! Depcrate_referencesimpl_76 {
() => {
// Module: crate::references
// Provides: {"impl_76"}
// Dependencies: {}
impl ReferenceStage { # [track_caller] pub fn parse (mut arg : & str) -> Self { if arg == "windows" { arg = "windows,skip-root,Windows" } let arg : Vec < _ > = arg . split (',') . collect () ; if arg . len () != 3 { invalid_reference () ; } Self { name : arg [0] . to_string () , style : ReferenceStyle :: parse (arg [1]) , path : arg [2] . to_string () , } } }
};
}
