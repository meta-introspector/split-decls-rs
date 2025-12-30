// Generated macro for impl_1199 (impl)
macro_rules! Depcrate_read_xcoff_symbolimpl_1199 {
() => {
// Module: crate::read::xcoff::symbol
// Provides: {"impl_1199"}
// Dependencies: {}
impl CsectAux for xcoff :: CsectAux32 { fn x_scnlen (& self) -> u64 { self . x_scnlen . get (BE) as u64 } fn x_parmhash (& self) -> u32 { self . x_parmhash . get (BE) } fn x_snhash (& self) -> u16 { self . x_snhash . get (BE) } fn x_smtyp (& self) -> u8 { self . x_smtyp } fn x_smclas (& self) -> u8 { self . x_smclas } fn x_stab (& self) -> Option < u32 > { Some (self . x_stab . get (BE)) } fn x_snstab (& self) -> Option < u16 > { Some (self . x_snstab . get (BE)) } fn x_auxtype (& self) -> Option < u8 > { None } }
};
}
