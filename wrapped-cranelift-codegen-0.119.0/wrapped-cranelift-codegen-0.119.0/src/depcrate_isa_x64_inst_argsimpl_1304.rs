// Generated macro for impl_1304 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1304 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1304"}
// Dependencies: {}
impl PrettyPrint for SyntheticAmode { fn pretty_print (& self , _size : u8) -> String { match self { SyntheticAmode :: Real (addr) => addr . pretty_print (8) , & SyntheticAmode :: IncomingArg { offset } => { format ! ("rbp(stack args max - {offset})") } SyntheticAmode :: SlotOffset { simm32 } => { format ! ("rsp({} + virtual offset)" , * simm32) } SyntheticAmode :: ConstantOffset (c) => format ! ("const({})" , c . as_u32 ()) , } } }
};
}
