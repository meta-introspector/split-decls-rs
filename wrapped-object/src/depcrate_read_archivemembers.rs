// Generated macro for Members (enum)
macro_rules! Depcrate_read_archiveMembers {
() => {
// Module: crate::read::archive
// Provides: {"Members"}
// Dependencies: {}
# [doc = " The list of members in the archive."] # [derive (Debug , Clone , Copy)] enum Members < 'data > { Common { offset : u64 , end_offset : u64 , } , AixBig { index : & 'data [archive :: AixMemberOffset] , } , }
};
}
