// Generated macro for impl_1789 (impl)
macro_rules! Depcrate_perfcnt_intelimpl_1789 {
() => {
// Module: crate::perfcnt::intel
// Provides: {"impl_1789"}
// Dependencies: {}
impl Write for ModelWriter { fn write_str (& mut self , s : & str) -> Result { for c in s . chars () { if self . index >= self . buffer . len () { return Err (Error) ; } self . buffer [self . index] = c as u8 ; self . index += 1 ; } Ok (()) } }
};
}
