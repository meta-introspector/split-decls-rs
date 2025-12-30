// Generated macro for impl_232 (impl)
macro_rules! Depcrate_read_cfiimpl_232 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_232"}
// Dependencies: {}
impl BaseAddresses { # [doc = " Set the `.eh_frame_hdr` section base address."] # [inline] pub fn set_eh_frame_hdr (mut self , addr : u64) -> Self { self . eh_frame_hdr . section = Some (addr) ; self . eh_frame_hdr . data = Some (addr) ; self } # [doc = " Set the `.eh_frame` section base address."] # [inline] pub fn set_eh_frame (mut self , addr : u64) -> Self { self . eh_frame . section = Some (addr) ; self } # [doc = " Set the `.text` section base address."] # [inline] pub fn set_text (mut self , addr : u64) -> Self { self . eh_frame_hdr . text = Some (addr) ; self . eh_frame . text = Some (addr) ; self } # [doc = " Set the `.got` section base address."] # [inline] pub fn set_got (mut self , addr : u64) -> Self { self . eh_frame . data = Some (addr) ; self } }
};
}
