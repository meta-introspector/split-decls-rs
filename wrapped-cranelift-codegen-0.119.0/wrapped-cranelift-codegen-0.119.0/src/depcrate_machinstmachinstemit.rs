// Generated macro for MachInstEmit (trait)
macro_rules! Depcrate_machinstMachInstEmit {
() => {
// Module: crate::machinst
// Provides: {"MachInstEmit"}
// Dependencies: {}
# [doc = " A trait describing the ability to encode a MachInst into binary machine code."] pub trait MachInstEmit : MachInst { # [doc = " Persistent state carried across `emit` invocations."] type State : MachInstEmitState < Self > ; # [doc = " Constant information used in `emit` invocations."] type Info ; # [doc = " Emit the instruction."] fn emit (& self , code : & mut MachBuffer < Self > , info : & Self :: Info , state : & mut Self :: State) ; # [doc = " Pretty-print the instruction."] fn pretty_print_inst (& self , state : & mut Self :: State) -> String ; }
};
}
