// Generated macro for impl_803 (impl)
macro_rules! Depcrate_ir_immediatesimpl_803 {
() => {
// Module: crate::ir::immediates
// Provides: {"impl_803"}
// Dependencies: {}
impl Display for Imm64 { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { let x = self . 0 ; if x < 10_000 { write ! (f , "{x}") } else { write_hex (x as u64 , f) } } }
};
}
