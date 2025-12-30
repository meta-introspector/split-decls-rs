// Generated macro for macro_297 (macro)
macro_rules! Depcrate_unistdmacro_297 {
() => {
// Module: crate::unistd
// Provides: {"macro_297"}
// Dependencies: {}
feature ! { #! [feature = "process"] # [cfg (target_os = "freebsd")] # [doc = " Like [`fork`], `rfork` can be used to have a tigher control about which"] # [doc = " resources child and parent process will be sharing, file descriptors,"] # [doc = " address spaces and child exit's behavior."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The same restrictions apply as for [`fork`]."] # [doc = ""] # [doc = " # See Also"] # [doc = ""] # [doc = " * [rfork(2)](https://man.freebsd.org/cgi/man.cgi?query=rfork)"] pub unsafe fn rfork (flags : RforkFlags) -> Result < ForkResult > { use ForkResult ::*; let res = unsafe { libc :: rfork (flags . bits ()) } ; Errno :: result (res) . map (| res | match res { 0 => Child , res => Parent { child : Pid (res) } , }) } }
};
}
