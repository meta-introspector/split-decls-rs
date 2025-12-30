// Generated macro for ArchiveMemberIterator (struct)
macro_rules! Depcrate_read_archiveArchiveMemberIterator {
() => {
// Module: crate::read::archive
// Provides: {"ArchiveMemberIterator"}
// Dependencies: {}
# [doc = " An iterator over the members of an archive."] # [derive (Debug)] pub struct ArchiveMemberIterator < 'data , R : ReadRef < 'data > = & 'data [u8] > { data : R , members : Members < 'data > , names : & 'data [u8] , thin : bool , }
};
}
