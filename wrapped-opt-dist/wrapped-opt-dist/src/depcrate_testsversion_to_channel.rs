// Generated macro for version_to_channel (function)
macro_rules! Depcrate_testsversion_to_channel {
() => {
// Module: crate::tests
// Provides: {"version_to_channel"}
// Dependencies: {}
# [doc = " Roughly convert a version string (`nightly`, `beta`, or `1.XY.Z`) to channel string (`nightly`,"] # [doc = " `beta` or `stable`)."] fn version_to_channel (version_str : & str) -> & 'static str { match version_str { "nightly" => "nightly" , "beta" => "beta" , _ => "stable" , } }
};
}
