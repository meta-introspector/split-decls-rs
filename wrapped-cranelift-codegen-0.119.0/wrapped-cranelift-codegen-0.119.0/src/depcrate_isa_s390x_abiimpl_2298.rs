// Generated macro for impl_2298 (impl)
macro_rules! Depcrate_isa_s390x_abiimpl_2298 {
() => {
// Module: crate::isa::s390x::abi
// Provides: {"impl_2298"}
// Dependencies: {}
impl Into < MemArg > for StackAMode { fn into (self) -> MemArg { match self { StackAMode :: IncomingArg (off , _) => MemArg :: InitialSPOffset { off } , StackAMode :: Slot (off) => MemArg :: SlotOffset { off } , StackAMode :: OutgoingArg (off) => MemArg :: NominalSPOffset { off } , } } }
};
}
