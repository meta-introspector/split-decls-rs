// Generated macro for impl_820 (impl)
macro_rules! Depcrate_ir_immediatesimpl_820 {
() => {
// Module: crate::ir::immediates
// Provides: {"impl_820"}
// Dependencies: {}
impl Display for Uimm32 { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { if self . 0 < 10_000 { write ! (f , "{}" , self . 0) } else { write_hex (u64 :: from (self . 0) , f) } } }
};
}
