// Generated macro for ArchiveMember (struct)
macro_rules! Depcrate_read_archiveArchiveMember {
() => {
// Module: crate::read::archive
// Provides: {"ArchiveMember"}
// Dependencies: {}
# [doc = " A partially parsed archive member."] # [derive (Debug)] pub struct ArchiveMember < 'data > { header : MemberHeader < 'data > , name : & 'data [u8] , offset : u64 , size : u64 , }
};
}
