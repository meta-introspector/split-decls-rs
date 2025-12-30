// Generated macro for PeComdat (struct)
macro_rules! Depcrate_read_pe_filePeComdat {
() => {
// Module: crate::read::pe::file
// Provides: {"PeComdat"}
// Dependencies: {}
# [doc = " A COMDAT section group in a [`PeFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct PeComdat < 'data , 'file , Pe , R = & 'data [u8] > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { # [allow (unused)] file : & 'file PeFile < 'data , Pe , R > , }
};
}
