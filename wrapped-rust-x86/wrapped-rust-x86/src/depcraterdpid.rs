// Generated macro for rdpid (function)
macro_rules! Depcraterdpid {
() => {
// Module: crate
// Provides: {"rdpid"}
// Dependencies: {}
# [doc = " Read Processor ID"] # [doc = ""] # [doc = " Reads the value of the IA32_TSC_AUX MSR (address C0000103H) into the"] # [doc = " destination register."] # [doc = ""] # [doc = " # See also"] # [doc = " `IA32_TSC_AUX` can also be read calling [`crate::time::rdtscp`]."] # [doc = ""] # [doc = " # Safety"] # [doc = " May fail with #UD if rdpid is not supported (check CPUID)."] # [inline (always)] pub unsafe fn rdpid () -> u64 { # [cfg (target_pointer_width = "64")] let mut pid : u64 ; # [cfg (target_pointer_width = "32")] let mut pid : u32 ; asm ! ("rdpid {pid}" , pid = out (reg) pid , options (att_syntax)) ; pid . into () }
};
}
