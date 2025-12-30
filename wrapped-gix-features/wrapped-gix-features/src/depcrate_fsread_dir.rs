// Generated macro for read_dir (module)
macro_rules! Depcrate_fsread_dir {
() => {
// Module: crate::fs
// Provides: {"read_dir"}
// Dependencies: {}
# [doc = ""] # [cfg (feature = "fs-read-dir")] pub mod read_dir { use std :: { borrow :: Cow , ffi :: OsStr , fs :: FileType , path :: Path } ; # [doc = " A directory entry adding precompose-unicode support to [`std::fs::DirEntry`]."] pub type DirEntry = super :: walkdir_precompose :: DirEntry < std :: fs :: DirEntry > ; impl super :: walkdir_precompose :: DirEntryApi for std :: fs :: DirEntry { fn path (& self) -> Cow < '_ , Path > { self . path () . into () } fn file_name (& self) -> Cow < '_ , OsStr > { self . file_name () . into () } fn file_type (& self) -> std :: io :: Result < FileType > { self . file_type () } } }
};
}
