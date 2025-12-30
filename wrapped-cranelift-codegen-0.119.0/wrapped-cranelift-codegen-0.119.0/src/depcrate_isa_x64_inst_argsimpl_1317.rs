// Generated macro for impl_1317 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1317 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1317"}
// Dependencies: {}
impl PrettyPrint for RegMem { fn pretty_print (& self , size : u8) -> String { match self { RegMem :: Reg { reg } => pretty_print_reg (* reg , size) , RegMem :: Mem { addr , .. } => addr . pretty_print (size) , } } }
};
}
