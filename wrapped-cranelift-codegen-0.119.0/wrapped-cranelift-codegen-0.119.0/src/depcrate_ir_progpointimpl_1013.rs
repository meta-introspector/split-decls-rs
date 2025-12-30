// Generated macro for impl_1013 (impl)
macro_rules! Depcrate_ir_progpointimpl_1013 {
() => {
// Module: crate::ir::progpoint
// Provides: {"impl_1013"}
// Dependencies: {}
impl fmt :: Display for ProgramPoint { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { Self :: Inst (x) => write ! (f , "{x}") , Self :: Block (x) => write ! (f , "{x}") , } } }
};
}
