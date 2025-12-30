// Generated macro for impl_1174 (impl)
macro_rules! Depcrate_isa_x64_abiimpl_1174 {
() => {
// Module: crate::isa::x64::abi
// Provides: {"impl_1174"}
// Dependencies: {}
impl From < StackAMode > for SyntheticAmode { fn from (amode : StackAMode) -> Self { match amode { StackAMode :: IncomingArg (off , stack_args_size) => { let offset = u32 :: try_from (off) . expect ("Offset in IncomingArg is greater than 4GB; should hit impl limit first" ,) ; SyntheticAmode :: IncomingArg { offset : stack_args_size - offset , } } StackAMode :: Slot (off) => { let off = i32 :: try_from (off) . expect ("Offset in Slot is greater than 2GB; should hit impl limit first") ; SyntheticAmode :: slot_offset (off) } StackAMode :: OutgoingArg (off) => { let off = i32 :: try_from (off) . expect ("Offset in OutgoingArg is greater than 2GB; should hit impl limit first" ,) ; SyntheticAmode :: Real (Amode :: ImmReg { simm32 : off , base : regs :: rsp () , flags : MemFlags :: trusted () , }) } } } }
};
}
