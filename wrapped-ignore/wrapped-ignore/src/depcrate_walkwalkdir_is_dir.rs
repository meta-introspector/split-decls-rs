// Generated macro for walkdir_is_dir (function)
macro_rules! Depcrate_walkwalkdir_is_dir {
() => {
// Module: crate::walk
// Provides: {"walkdir_is_dir"}
// Dependencies: {}
# [doc = " Returns true if the given walkdir entry corresponds to a directory."] # [doc = ""] # [doc = " This is normally just `dent.file_type().is_dir()`, but when we aren't"] # [doc = " following symlinks, the root directory entry may be a symlink to a"] # [doc = " directory that we *do* follow---by virtue of it being specified by the user"] # [doc = " explicitly. In that case, we need to follow the symlink and query whether"] # [doc = " it's a directory or not. But we only do this for root entries to avoid an"] # [doc = " additional stat check in most cases."] fn walkdir_is_dir (dent : & walkdir :: DirEntry) -> bool { if dent . file_type () . is_dir () { return true ; } if ! dent . file_type () . is_symlink () || dent . depth () > 0 { return false ; } dent . path () . metadata () . ok () . map_or (false , | md | md . file_type () . is_dir ()) }
};
}
