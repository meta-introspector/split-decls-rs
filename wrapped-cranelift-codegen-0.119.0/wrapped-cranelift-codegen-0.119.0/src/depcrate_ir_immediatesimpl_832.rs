// Generated macro for impl_832 (impl)
macro_rules! Depcrate_ir_immediatesimpl_832 {
() => {
// Module: crate::ir::immediates
// Provides: {"impl_832"}
// Dependencies: {}
impl Display for Offset32 { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { if self . 0 == 0 { return Ok (()) ; } write ! (f , "{}" , if self . 0 < 0 { '-' } else { '+' }) ? ; let val = i64 :: from (self . 0) . abs () ; if val < 10_000 { write ! (f , "{val}") } else { write_hex (val as u64 , f) } } }
};
}
