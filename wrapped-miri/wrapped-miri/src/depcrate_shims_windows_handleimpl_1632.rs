// Generated macro for impl_1632 (impl)
macro_rules! Depcrate_shims_windows_handleimpl_1632 {
() => {
// Module: crate::shims::windows::handle
// Provides: {"impl_1632"}
// Dependencies: {}
impl PseudoHandle { const CURRENT_THREAD_VALUE : u32 = 0 ; const CURRENT_PROCESS_VALUE : u32 = 1 ; fn value (self) -> u32 { match self { Self :: CurrentThread => Self :: CURRENT_THREAD_VALUE , Self :: CurrentProcess => Self :: CURRENT_PROCESS_VALUE , } } fn from_value (value : u32) -> Option < Self > { match value { Self :: CURRENT_THREAD_VALUE => Some (Self :: CurrentThread) , Self :: CURRENT_PROCESS_VALUE => Some (Self :: CurrentProcess) , _ => None , } } }
};
}
