// Generated macro for PeFile64 (type)
macro_rules! Depcrate_read_pe_filePeFile64 {
() => {
// Module: crate::read::pe::file
// Provides: {"PeFile64"}
// Dependencies: {}
# [doc = " A PE32+ (64-bit) image file."] # [doc = ""] # [doc = " This is a file that starts with [`pe::ImageNtHeaders64`], and corresponds"] # [doc = " to [`crate::FileKind::Pe64`]."] pub type PeFile64 < 'data , R = & 'data [u8] > = PeFile < 'data , pe :: ImageNtHeaders64 , R > ;
};
}
