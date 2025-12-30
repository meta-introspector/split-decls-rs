// Generated macro for impl_1010 (impl)
macro_rules! Depcrate_ir_progpointimpl_1010 {
() => {
// Module: crate::ir::progpoint
// Provides: {"impl_1010"}
// Dependencies: {}
impl ProgramPoint { # [doc = " Get the instruction we know is inside."] pub fn unwrap_inst (self) -> Inst { match self { Self :: Inst (x) => x , Self :: Block (x) => panic ! ("expected inst: {x}") , } } }
};
}
