// Generated macro for impl_1605 (impl)
macro_rules! Depcrate_isa_aarch64_abiimpl_1605 {
() => {
// Module: crate::isa::aarch64::abi
// Provides: {"impl_1605"}
// Dependencies: {}
impl Into < AMode > for StackAMode { fn into (self) -> AMode { match self { StackAMode :: IncomingArg (off , stack_args_size) => AMode :: IncomingArg { off : i64 :: from (stack_args_size) - off , } , StackAMode :: Slot (off) => AMode :: SlotOffset { off } , StackAMode :: OutgoingArg (off) => AMode :: SPOffset { off } , } } }
};
}
