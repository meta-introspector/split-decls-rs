// Generated macro for move_directory (function)
macro_rules! Depcrate_utils_iomove_directory {
() => {
// Module: crate::utils::io
// Provides: {"move_directory"}
// Dependencies: {}
# [allow (unused)] pub fn move_directory (src : & Utf8Path , dst : & Utf8Path) -> anyhow :: Result < () > { log :: info ! ("Moving directory {src} to {dst}") ; fs_extra :: dir :: move_dir (src , dst , & CopyOptions :: default () . content_only (true)) ? ; Ok (()) }
};
}
