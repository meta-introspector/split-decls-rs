// Generated macro for version (function)
macro_rules! Depcrateversion {
() => {
// Module: crate
// Provides: {"version"}
// Dependencies: {}
# [doc = " Returns the `rustc` SemVer version."] pub fn version () -> Result < Version > { Ok (version_meta () ? . semver) }
};
}
