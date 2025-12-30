// Generated macro for get_dit_enabled (function)
macro_rules! Depcrateget_dit_enabled {
() => {
// Module: crate
// Provides: {"get_dit_enabled"}
// Dependencies: {}
# [doc = " Detect if DIT is enabled for the current thread by checking the processor state register."] # [target_feature (enable = "dit")] unsafe fn get_dit_enabled () -> bool { let mut dit : u64 ; unsafe { asm ! ("mrs {dit}, DIT" , dit = out (reg) dit , options (nomem , nostack , preserves_flags)) ; } (dit >> 24) & 1 != 0 }
};
}
