// Generated macro for NewArchiveMember (struct)
macro_rules! Depcrate_archive_writerNewArchiveMember {
() => {
// Module: crate::archive_writer
// Provides: {"NewArchiveMember"}
// Dependencies: {}
pub struct NewArchiveMember < 'a > { pub buf : Box < dyn AsRef < [u8] > + 'a > , pub object_reader : & 'static ObjectReader , pub member_name : String , pub mtime : u64 , pub uid : u32 , pub gid : u32 , pub perms : u32 , }
};
}
