// Generated macro for impl_1309 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1309 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1309"}
// Dependencies: {}
impl PrettyPrint for RegMemImm { fn pretty_print (& self , size : u8) -> String { match self { Self :: Reg { reg } => pretty_print_reg (* reg , size) , Self :: Mem { addr } => addr . pretty_print (size) , Self :: Imm { simm32 } => format ! ("${}" , * simm32 as i32) , } } }
};
}
