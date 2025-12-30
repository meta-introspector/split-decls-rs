// Generated macro for impl_20 (impl)
macro_rules! Depcrate_archive_writerimpl_20 {
() => {
// Module: crate::archive_writer
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a > NewArchiveMember < 'a > { pub fn new < T : AsRef < [u8] > + 'a > (buf : T , object_reader : & 'static ObjectReader , member_name : String ,) -> Self { Self { buf : Box :: new (buf) , object_reader , member_name , mtime : 0 , uid : 0 , gid : 0 , perms : 0o644 , } } }
};
}
