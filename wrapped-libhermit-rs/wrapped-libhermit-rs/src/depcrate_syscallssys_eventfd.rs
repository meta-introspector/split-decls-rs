// Generated macro for sys_eventfd (function)
macro_rules! Depcrate_syscallssys_eventfd {
() => {
// Module: crate::syscalls
// Provides: {"sys_eventfd"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_eventfd (initval : u64 , flags : i16) -> i32 { if let Some (flags) = EventFlags :: from_bits (flags) { crate :: fd :: eventfd (initval , flags) . unwrap_or_else (| e | - i32 :: from (e)) } else { - i32 :: from (Errno :: Inval) } }
};
}
