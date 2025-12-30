// Generated macro for active_toolchain (function)
macro_rules! Depcrate_utilactive_toolchain {
() => {
// Module: crate::util
// Provides: {"active_toolchain"}
// Dependencies: {}
# [doc = " Queries the active toolchain for the Miri dir."] pub fn active_toolchain () -> Result < String > { let sh = Shell :: new () ? ; sh . change_dir (miri_dir () ?) ; let stdout = cmd ! (sh , "rustup show active-toolchain") . read () ? ; Ok (stdout . split_whitespace () . next () . context ("Could not obtain active Rust toolchain") ? . into ()) }
};
}
