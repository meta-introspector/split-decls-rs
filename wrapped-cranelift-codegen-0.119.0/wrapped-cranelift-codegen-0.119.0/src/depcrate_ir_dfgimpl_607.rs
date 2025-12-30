// Generated macro for impl_607 (impl)
macro_rules! Depcrate_ir_dfgimpl_607 {
() => {
// Module: crate::ir::dfg
// Provides: {"impl_607"}
// Dependencies: {}
impl ValueDef { # [doc = " Unwrap the instruction where the value was defined, or panic."] pub fn unwrap_inst (& self) -> Inst { self . inst () . expect ("Value is not an instruction result") } # [doc = " Get the instruction where the value was defined, if any."] pub fn inst (& self) -> Option < Inst > { match * self { Self :: Result (inst , _) => Some (inst) , _ => None , } } # [doc = " Unwrap the block there the parameter is defined, or panic."] pub fn unwrap_block (& self) -> Block { match * self { Self :: Param (block , _) => block , _ => panic ! ("Value is not a block parameter") , } } # [doc = " Get the number component of this definition."] # [doc = ""] # [doc = " When multiple values are defined at the same program point, this indicates the index of"] # [doc = " this value."] pub fn num (self) -> usize { match self { Self :: Result (_ , n) | Self :: Param (_ , n) => n , Self :: Union (_ , _) => 0 , } } }
};
}
