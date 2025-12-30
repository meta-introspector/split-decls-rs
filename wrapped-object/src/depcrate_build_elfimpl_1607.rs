// Generated macro for impl_1607 (impl)
macro_rules! Depcrate_build_elfimpl_1607 {
() => {
// Module: crate::build::elf
// Provides: {"impl_1607"}
// Dependencies: {}
impl < 'data > Section < 'data > { # [doc = " The ID used for referring to this section."] pub fn id (& self) -> SectionId { self . id } # [doc = " Returns true if the section flags include `SHF_ALLOC`."] pub fn is_alloc (& self) -> bool { self . sh_flags & u64 :: from (elf :: SHF_ALLOC) != 0 } # [doc = " Return the segment permission flags that are equivalent to the section flags."] pub fn p_flags (& self) -> u32 { let mut p_flags = elf :: PF_R ; if self . sh_flags & u64 :: from (elf :: SHF_WRITE) != 0 { p_flags |= elf :: PF_W ; } if self . sh_flags & u64 :: from (elf :: SHF_EXECINSTR) != 0 { p_flags |= elf :: PF_X ; } p_flags } }
};
}
