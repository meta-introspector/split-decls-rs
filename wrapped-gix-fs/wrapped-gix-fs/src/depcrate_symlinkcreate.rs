// Generated macro for create (function)
macro_rules! Depcrate_symlinkcreate {
() => {
// Module: crate::symlink
// Provides: {"create"}
// Dependencies: {}
# [doc = " Create a new symlink at `link` which points to `original`."] # [doc = ""] # [doc = " Note that if a symlink target (the `original`) isn't present on disk, it's assumed to be a"] # [doc = " file, creating a dangling file symlink. This is similar to a dangling symlink on Unix,"] # [doc = " which doesn't have to care about the target type though."] # [cfg (windows)] pub fn create (original : & Path , link : & Path) -> io :: Result < () > { use std :: os :: windows :: fs :: { symlink_dir , symlink_file } ; let orig_abs = link . parent () . expect ("dir for link") . join (original) ; if orig_abs . is_dir () { symlink_dir (original , link) } else { symlink_file (original , link) } }
};
}
