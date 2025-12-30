// Generated macro for impl_1299 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1299 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1299"}
// Dependencies: {}
impl PrettyPrint for Amode { fn pretty_print (& self , _size : u8) -> String { match self { Amode :: ImmReg { simm32 , base , .. } => { format ! ("{}({})" , * simm32 , pretty_print_reg (* base , 8)) } Amode :: ImmRegRegShift { simm32 , base , index , shift , .. } => format ! ("{}({},{},{})" , * simm32 , pretty_print_reg (base . to_reg () , 8) , pretty_print_reg (index . to_reg () , 8) , 1 << shift) , Amode :: RipRelative { target } => format ! ("label{}(%rip)" , target . as_u32 ()) , } } }
};
}
