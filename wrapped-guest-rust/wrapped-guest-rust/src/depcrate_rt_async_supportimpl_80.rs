// Generated macro for impl_80 (impl)
macro_rules! Depcrate_rt_async_supportimpl_80 {
() => {
// Module: crate::rt::async_support
// Provides: {"impl_80"}
// Dependencies: {}
impl ReturnCode { fn decode (val : u32) -> ReturnCode { if val == BLOCKED { return ReturnCode :: Blocked ; } let amt = val >> 4 ; match val & 0xf { COMPLETED => ReturnCode :: Completed (amt) , DROPPED => ReturnCode :: Dropped (amt) , CANCELLED => ReturnCode :: Cancelled (amt) , _ => panic ! ("unknown return code {val:#x}") , } } }
};
}
