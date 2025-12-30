// Generated macro for impl_1022 (impl)
macro_rules! Depcrate_ir_sourcelocimpl_1022 {
() => {
// Module: crate::ir::sourceloc
// Provides: {"impl_1022"}
// Dependencies: {}
impl fmt :: Display for SourceLoc { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if self . is_default () { write ! (f , "@-") } else { write ! (f , "@{:04x}" , self . 0) } } }
};
}
