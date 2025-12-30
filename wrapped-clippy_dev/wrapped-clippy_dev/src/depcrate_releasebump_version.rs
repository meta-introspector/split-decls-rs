// Generated macro for bump_version (function)
macro_rules! Depcrate_releasebump_version {
() => {
// Module: crate::release
// Provides: {"bump_version"}
// Dependencies: {}
pub fn bump_version (mut version : Version) { version . minor += 1 ; let mut updater = FileUpdater :: default () ; for file in CARGO_TOML_FILES { updater . update_file (file , & mut | _ , src , dst | { let package = parse_cargo_package (src) ; if package . version_range . is_empty () { dst . push_str (src) ; UpdateStatus :: Unchanged } else { dst . push_str (& src [.. package . version_range . start]) ; write ! (dst , "\"{}\"" , version . toml_display ()) . unwrap () ; dst . push_str (& src [package . version_range . end ..]) ; UpdateStatus :: from_changed (src . get (package . version_range) != dst . get (package . version_range)) } }) ; } }
};
}
