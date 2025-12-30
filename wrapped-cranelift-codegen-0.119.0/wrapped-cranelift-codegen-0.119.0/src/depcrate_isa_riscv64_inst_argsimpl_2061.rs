// Generated macro for impl_2061 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2061 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2061"}
// Dependencies: {}
impl FFlagsException { # [inline] # [allow (dead_code)] pub (crate) fn mask (self) -> u32 { match self { FFlagsException :: NV => 1 << 4 , FFlagsException :: DZ => 1 << 3 , FFlagsException :: OF => 1 << 2 , FFlagsException :: UF => 1 << 1 , FFlagsException :: NX => 1 << 0 , } } }
};
}
