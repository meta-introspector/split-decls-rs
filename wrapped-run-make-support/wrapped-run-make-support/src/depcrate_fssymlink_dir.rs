// Generated macro for symlink_dir (function)
macro_rules! Depcrate_fssymlink_dir {
() => {
// Module: crate::fs
// Provides: {"symlink_dir"}
// Dependencies: {}
# [doc = " Create a new symbolic link to a directory."] # [doc = ""] # [doc = " # Removing the symlink"] # [doc = ""] # [doc = " - On Windows, a symlink-to-directory needs to be removed with a corresponding [`fs::remove_dir`]"] # [doc = "   and not [`fs::remove_file`]."] # [doc = " - On Unix, remove the symlink with [`fs::remove_file`]."] # [doc = ""] # [doc = " [`fs::remove_dir`]: crate::fs::remove_dir"] # [doc = " [`fs::remove_file`]: crate::fs::remove_file"] pub fn symlink_dir < P : AsRef < Path > , Q : AsRef < Path > > (original : P , link : Q) { # [cfg (unix)] { if let Err (e) = std :: os :: unix :: fs :: symlink (original . as_ref () , link . as_ref ()) { panic ! ("failed to create symlink: original=`{}`, link=`{}`: {e}" , original . as_ref () . display () , link . as_ref () . display ()) ; } } # [cfg (windows)] { if let Err (e) = std :: os :: windows :: fs :: symlink_dir (original . as_ref () , link . as_ref ()) { panic ! ("failed to create symlink-to-directory: original=`{}`, link=`{}`: {e}" , original . as_ref () . display () , link . as_ref () . display ()) ; } } # [cfg (not (any (windows , unix)))] { unimplemented ! ("target family not currently supported") } }
};
}
