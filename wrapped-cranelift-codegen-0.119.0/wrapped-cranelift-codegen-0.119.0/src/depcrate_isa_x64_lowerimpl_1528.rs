// Generated macro for impl_1528 (impl)
macro_rules! Depcrate_isa_x64_lowerimpl_1528 {
() => {
// Module: crate::isa::x64::lower
// Provides: {"impl_1528"}
// Dependencies: {}
impl Lower < '_ , Inst > { # [inline] pub fn temp_writable_gpr (& mut self) -> WritableGpr { WritableGpr :: from_writable_reg (self . alloc_tmp (types :: I64) . only_reg () . unwrap ()) . unwrap () } # [inline] pub fn temp_writable_xmm (& mut self) -> WritableXmm { WritableXmm :: from_writable_reg (self . alloc_tmp (types :: F64) . only_reg () . unwrap ()) . unwrap () } }
};
}
