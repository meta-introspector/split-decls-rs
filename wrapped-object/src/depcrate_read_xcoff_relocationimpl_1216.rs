// Generated macro for impl_1216 (impl)
macro_rules! Depcrate_read_xcoff_relocationimpl_1216 {
() => {
// Module: crate::read::xcoff::relocation
// Provides: {"impl_1216"}
// Dependencies: {}
impl Rel for xcoff :: Rel32 { type Word = u32 ; fn r_vaddr (& self) -> Self :: Word { self . r_vaddr . get (BE) } fn r_symndx (& self) -> u32 { self . r_symndx . get (BE) } fn r_rsize (& self) -> u8 { self . r_rsize } fn r_rtype (& self) -> u8 { self . r_rtype } }
};
}
