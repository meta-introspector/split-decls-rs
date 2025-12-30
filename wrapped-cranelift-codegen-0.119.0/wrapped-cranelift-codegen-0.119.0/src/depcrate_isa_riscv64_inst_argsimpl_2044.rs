// Generated macro for impl_2044 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2044 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2044"}
// Dependencies: {}
impl Into < AMode > for StackAMode { fn into (self) -> AMode { match self { StackAMode :: IncomingArg (offset , stack_args_size) => { AMode :: IncomingArg (i64 :: from (stack_args_size) - offset) } StackAMode :: OutgoingArg (offset) => AMode :: SPOffset (offset) , StackAMode :: Slot (offset) => AMode :: SlotOffset (offset) , } } }
};
}
