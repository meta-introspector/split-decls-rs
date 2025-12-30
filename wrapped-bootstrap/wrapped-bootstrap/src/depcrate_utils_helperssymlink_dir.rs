// Generated macro for symlink_dir (function)
macro_rules! Depcrate_utils_helperssymlink_dir {
() => {
// Module: crate::utils::helpers
// Provides: {"symlink_dir"}
// Dependencies: {}
# [doc = " Symlinks two directories, using junctions on Windows and normal symlinks on"] # [doc = " Unix."] pub fn symlink_dir (config : & Config , original : & Path , link : & Path) -> io :: Result < () > { if config . dry_run () { return Ok (()) ; } let _ = fs :: remove_dir_all (link) ; return symlink_dir_inner (original , link) ; # [cfg (not (windows))] fn symlink_dir_inner (original : & Path , link : & Path) -> io :: Result < () > { use std :: os :: unix :: fs ; fs :: symlink (original , link) } # [cfg (windows)] fn symlink_dir_inner (target : & Path , junction : & Path) -> io :: Result < () > { junction :: create (target , junction) } }
};
}
