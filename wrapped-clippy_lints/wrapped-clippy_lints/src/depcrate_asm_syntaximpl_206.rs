// Generated macro for impl_206 (impl)
macro_rules! Depcrate_asm_syntaximpl_206 {
() => {
// Module: crate::asm_syntax
// Provides: {"impl_206"}
// Dependencies: {}
impl fmt :: Display for AsmStyle { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { AsmStyle :: Intel => f . write_str ("Intel") , AsmStyle :: Att => f . write_str ("AT&T") , } } }
};
}
