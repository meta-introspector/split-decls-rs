// Generated macro for impl_119 (impl)
macro_rules! Depcrate_walkimpl_119 {
() => {
// Module: crate::walk
// Provides: {"impl_119"}
// Dependencies: {}
impl Work { # [doc = " Returns true if and only if this work item is a directory."] fn is_dir (& self) -> bool { self . dent . is_dir () } # [doc = " Returns true if and only if this work item is a symlink."] fn is_symlink (& self) -> bool { self . dent . file_type () . map_or (false , | ft | ft . is_symlink ()) } # [doc = " Adds ignore rules for parent directories."] # [doc = ""] # [doc = " Note that this only applies to entries at depth 0. On all other"] # [doc = " entries, this is a no-op."] fn add_parents (& mut self) -> Option < Error > { if self . dent . depth () > 0 { return None ; } let (ig , err) = self . ignore . add_parents (self . dent . path ()) ; self . ignore = ig ; err } # [doc = " Reads the directory contents of this work item and adds ignore"] # [doc = " rules for this directory."] # [doc = ""] # [doc = " If there was a problem with reading the directory contents, then"] # [doc = " an error is returned. If there was a problem reading the ignore"] # [doc = " rules for this directory, then the error is attached to this"] # [doc = " work item's directory entry."] fn read_dir (& mut self) -> Result < fs :: ReadDir , Error > { let readdir = match fs :: read_dir (self . dent . path ()) { Ok (readdir) => readdir , Err (err) => { let err = Error :: from (err) . with_path (self . dent . path ()) . with_depth (self . dent . depth ()) ; return Err (err) ; } } ; let (ig , err) = self . ignore . add_child (self . dent . path ()) ; self . ignore = ig ; self . dent . err = err ; Ok (readdir) } }
};
}
