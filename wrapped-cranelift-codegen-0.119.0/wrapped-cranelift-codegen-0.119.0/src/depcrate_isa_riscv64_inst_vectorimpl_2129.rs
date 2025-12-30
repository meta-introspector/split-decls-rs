// Generated macro for impl_2129 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2129 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2129"}
// Dependencies: {}
impl VType { pub fn encode (& self) -> u32 { let mut bits = 0 ; bits |= self . lmul . encode () ; bits |= self . sew . encode () << 3 ; bits |= self . tail_mode . encode () << 6 ; bits |= self . mask_mode . encode () << 7 ; bits } }
};
}
