// Generated macro for set_dit_enabled (function)
macro_rules! Depcrateset_dit_enabled {
() => {
// Module: crate
// Provides: {"set_dit_enabled"}
// Dependencies: {}
# [doc = " Enable DIT for the current thread."] # [doc = ""] # [doc = " Returns the previous DIT state prior to enabling DIT."] # [target_feature (enable = "dit")] unsafe fn set_dit_enabled () -> bool { unsafe { let was_enabled = get_dit_enabled () ; asm ! ("msr DIT, #1" , options (nomem , nostack , preserves_flags)) ; was_enabled } }
};
}
