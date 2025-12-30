// Generated macro for impl_209 (impl)
macro_rules! Depcrate_asm_syntaximpl_209 {
() => {
// Module: crate::asm_syntax
// Provides: {"impl_209"}
// Dependencies: {}
impl fmt :: Display for AsmStyle { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { AsmStyle :: Intel => f . write_str ("Intel") , AsmStyle :: Att => f . write_str ("AT&T") , } } }
};
}
