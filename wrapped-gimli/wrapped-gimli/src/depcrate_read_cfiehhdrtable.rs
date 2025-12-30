// Generated macro for EhHdrTable (struct)
macro_rules! Depcrate_read_cfiEhHdrTable {
() => {
// Module: crate::read::cfi
// Provides: {"EhHdrTable"}
// Dependencies: {}
# [doc = " The CFI binary search table that is an optional part of the `.eh_frame_hdr` section."] # [derive (Debug , Clone)] pub struct EhHdrTable < 'a , R : Reader > { hdr : & 'a ParsedEhFrameHdr < R > , }
};
}
