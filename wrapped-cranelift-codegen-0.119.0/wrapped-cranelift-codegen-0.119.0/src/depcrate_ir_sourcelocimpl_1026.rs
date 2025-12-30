// Generated macro for impl_1026 (impl)
macro_rules! Depcrate_ir_sourcelocimpl_1026 {
() => {
// Module: crate::ir::sourceloc
// Provides: {"impl_1026"}
// Dependencies: {}
impl fmt :: Display for RelSourceLoc { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if self . is_default () { write ! (f , "@-") } else { write ! (f , "@+{:04x}" , self . 0) } } }
};
}
