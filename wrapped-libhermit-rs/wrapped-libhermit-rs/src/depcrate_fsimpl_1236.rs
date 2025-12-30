// Generated macro for impl_1236 (impl)
macro_rules! Depcrate_fsimpl_1236 {
() => {
// Module: crate::fs
// Provides: {"impl_1236"}
// Dependencies: {}
impl Metadata { # [doc = " Returns the size of the file, in bytes"] pub fn len (& self) -> usize { self . 0 . st_size . try_into () . unwrap () } # [doc = " Returns true if this metadata is for a file."] pub fn is_file (& self) -> bool { self . 0 . st_mode . contains (AccessPermission :: S_IFREG) } # [doc = " Returns true if this metadata is for a directory."] pub fn is_dir (& self) -> bool { self . 0 . st_mode . contains (AccessPermission :: S_IFDIR) } # [doc = " Returns the last modification time listed in this metadata."] pub fn modified (& self) -> io :: Result < SystemTime > { Ok (SystemTime :: from (self . 0 . st_mtim)) } # [doc = " Returns the last modification time listed in this metadata."] pub fn accessed (& self) -> io :: Result < SystemTime > { Ok (SystemTime :: from (self . 0 . st_atim)) } }
};
}
