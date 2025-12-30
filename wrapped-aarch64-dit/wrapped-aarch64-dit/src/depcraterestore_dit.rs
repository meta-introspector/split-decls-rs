// Generated macro for restore_dit (function)
macro_rules! Depcraterestore_dit {
() => {
// Module: crate
// Provides: {"restore_dit"}
// Dependencies: {}
# [doc = " Restore DIT state depending on the enabled bit."] # [target_feature (enable = "dit")] unsafe fn restore_dit (enabled : bool) { if ! enabled { unsafe { asm ! ("msr DIT, #0" , options (nomem , nostack , preserves_flags)) } ; } }
};
}
