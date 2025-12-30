// Generated macro for DestinationSlot (struct)
macro_rules! Depcrate_typesDestinationSlot {
() => {
// Module: crate::types
// Provides: {"DestinationSlot"}
// Dependencies: {}
# [doc = " A destination slot for sending fixed resources"] # [doc = " (e.g. [`opcode::MsgRingSendFd`](crate::opcode::MsgRingSendFd))."] # [derive (Debug , Clone , Copy)] pub struct DestinationSlot { # [doc = " Fixed slot as indexed by the kernel (target+1)."] dest : NonZeroU32 , }
};
}
