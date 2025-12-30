// Generated macro for _SEGGER_RTT (static)
macro_rules! Depcrate_SEGGER_RTT {
() => {
// Module: crate
// Provides: {"_SEGGER_RTT"}
// Dependencies: {}
# [doc = " Our shared header structure."] # [doc = ""] # [doc = " The host will read this structure so it must be arranged as expected."] # [doc = ""] # [doc = " NOTE the `rtt-target` API is too permissive. It allows writing arbitrary"] # [doc = " data to any channel (`set_print_channel` + `rprint*`) and that can corrupt"] # [doc = " defmt log frames. So we declare the RTT control block here and make it"] # [doc = " impossible to use `rtt-target` together with this crate."] # [no_mangle] static _SEGGER_RTT : Header = Header { id : * b"SEGGER RTT\0\0\0\0\0\0" , max_up_channels : 1 , max_down_channels : 0 , up_channel : Channel { name : NAME . as_ptr () , buffer : BUFFER . get () , size : BUF_SIZE , write : AtomicUsize :: new (0) , read : AtomicUsize :: new (0) , flags : AtomicUsize :: new (MODE_NON_BLOCKING_TRIM) , } , } ;
};
}
