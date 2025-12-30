// Generated macro for PeFile32 (type)
macro_rules! Depcrate_read_pe_filePeFile32 {
() => {
// Module: crate::read::pe::file
// Provides: {"PeFile32"}
// Dependencies: {}
# [doc = " A PE32 (32-bit) image file."] # [doc = ""] # [doc = " This is a file that starts with [`pe::ImageNtHeaders32`], and corresponds"] # [doc = " to [`crate::FileKind::Pe32`]."] pub type PeFile32 < 'data , R = & 'data [u8] > = PeFile < 'data , pe :: ImageNtHeaders32 , R > ;
};
}
