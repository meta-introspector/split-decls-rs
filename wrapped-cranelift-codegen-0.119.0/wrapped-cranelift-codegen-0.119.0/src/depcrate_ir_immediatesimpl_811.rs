// Generated macro for impl_811 (impl)
macro_rules! Depcrate_ir_immediatesimpl_811 {
() => {
// Module: crate::ir::immediates
// Provides: {"impl_811"}
// Dependencies: {}
impl Display for Uimm64 { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { let x = self . 0 ; if x < 10_000 { write ! (f , "{x}") } else { write_hex (x , f) } } }
};
}
