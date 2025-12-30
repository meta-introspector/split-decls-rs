// Generated macro for impl_1217 (impl)
macro_rules! Depcrate_read_xcoff_relocationimpl_1217 {
() => {
// Module: crate::read::xcoff::relocation
// Provides: {"impl_1217"}
// Dependencies: {}
impl Rel for xcoff :: Rel64 { type Word = u64 ; fn r_vaddr (& self) -> Self :: Word { self . r_vaddr . get (BE) } fn r_symndx (& self) -> u32 { self . r_symndx . get (BE) } fn r_rsize (& self) -> u8 { self . r_rsize } fn r_rtype (& self) -> u8 { self . r_rtype } }
};
}
