// Generated macro for impl_90 (impl)
macro_rules! Depcrate_elfimpl_90 {
() => {
// Module: crate::elf
// Provides: {"impl_90"}
// Dependencies: {}
impl Elf_Rel { # [inline] pub fn type_ (& self) -> u32 { # [cfg (target_pointer_width = "32")] { self . r_info & 0xff } # [cfg (target_pointer_width = "64")] { (self . r_info & 0xffff_ffff) as u32 } } }
};
}
