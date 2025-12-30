// Generated macro for impl_87 (impl)
macro_rules! Depcrate_elfimpl_87 {
() => {
// Module: crate::elf
// Provides: {"impl_87"}
// Dependencies: {}
impl Elf_Rela { # [inline] pub fn type_ (& self) -> u32 { # [cfg (target_pointer_width = "32")] { self . r_info & 0xff } # [cfg (target_pointer_width = "64")] { (self . r_info & 0xffff_ffff) as u32 } } }
};
}
