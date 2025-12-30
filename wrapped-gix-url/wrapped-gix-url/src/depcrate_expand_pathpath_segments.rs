// Generated macro for path_segments (function)
macro_rules! Depcrate_expand_pathpath_segments {
() => {
// Module: crate::expand_path
// Provides: {"path_segments"}
// Dependencies: {}
fn path_segments (path : & BStr) -> Option < impl Iterator < Item = & [u8] > > { if path . starts_with (b"/") { Some (path [1 ..] . split (| c | * c == b'/')) } else { None } }
};
}
