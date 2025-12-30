// Generated macro for impl_209 (impl)
macro_rules! Depcrate_read_cfiimpl_209 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_209"}
// Dependencies: {}
impl < R : Reader > ParsedEhFrameHdr < R > { # [doc = " Returns the address of the binary's `.eh_frame` section."] pub fn eh_frame_ptr (& self) -> Pointer { self . eh_frame_ptr } # [doc = " Retrieves the CFI binary search table, if there is one."] pub fn table (& self) -> Option < EhHdrTable < '_ , R > > { if self . fde_count == 0 { None } else { Some (EhHdrTable { hdr : self }) } } }
};
}
