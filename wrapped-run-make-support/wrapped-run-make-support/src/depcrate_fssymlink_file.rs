// Generated macro for symlink_file (function)
macro_rules! Depcrate_fssymlink_file {
() => {
// Module: crate::fs
// Provides: {"symlink_file"}
// Dependencies: {}
# [doc = " Create a new symbolic link to a file."] # [doc = ""] # [doc = " # Removing the symlink"] # [doc = ""] # [doc = " On both Windows and Unix, a symlink-to-file needs to be removed with a corresponding"] # [doc = " [`fs::remove_file`](crate::fs::remove_file) and not [`fs::remove_dir`](crate::fs::remove_dir)."] pub fn symlink_file < P : AsRef < Path > , Q : AsRef < Path > > (original : P , link : Q) { # [cfg (unix)] { if let Err (e) = std :: os :: unix :: fs :: symlink (original . as_ref () , link . as_ref ()) { panic ! ("failed to create symlink: original=`{}`, link=`{}`: {e}" , original . as_ref () . display () , link . as_ref () . display ()) ; } } # [cfg (windows)] { if let Err (e) = std :: os :: windows :: fs :: symlink_file (original . as_ref () , link . as_ref ()) { panic ! ("failed to create symlink-to-file: original=`{}`, link=`{}`: {e}" , original . as_ref () . display () , link . as_ref () . display ()) ; } } # [cfg (not (any (windows , unix)))] { unimplemented ! ("target family not currently supported") } }
};
}
