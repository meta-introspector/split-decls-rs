// Generated macro for ParsedEhFrameHdr (struct)
macro_rules! Depcrate_read_cfiParsedEhFrameHdr {
() => {
// Module: crate::read::cfi
// Provides: {"ParsedEhFrameHdr"}
// Dependencies: {}
# [doc = " `ParsedEhFrameHdr` contains the parsed information from the `.eh_frame_hdr` section."] # [derive (Clone , Debug)] pub struct ParsedEhFrameHdr < R : Reader > { address_size : u8 , section : R , eh_frame_ptr : Pointer , fde_count : u64 , table_enc : DwEhPe , table : R , }
};
}
