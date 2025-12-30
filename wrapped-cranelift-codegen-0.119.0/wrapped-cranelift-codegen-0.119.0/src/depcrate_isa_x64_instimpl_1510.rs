// Generated macro for impl_1510 (impl)
macro_rules! Depcrate_isa_x64_instimpl_1510 {
() => {
// Module: crate::isa::x64::inst
// Provides: {"impl_1510"}
// Dependencies: {}
impl MachInstEmit for Inst { type State = EmitState ; type Info = EmitInfo ; fn emit (& self , sink : & mut MachBuffer < Inst > , info : & Self :: Info , state : & mut Self :: State) { emit :: emit (self , sink , info , state) ; } fn pretty_print_inst (& self , _ : & mut Self :: State) -> String { PrettyPrint :: pretty_print (self , 0) } }
};
}
