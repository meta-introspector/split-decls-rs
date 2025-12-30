// Generated macro for release (function)
macro_rules! Depcraterelease {
() => {
// Module: crate
// Provides: {"release"}
// Dependencies: {}
# [doc = " Release the critical section."] # [doc = ""] # [doc = " This function is extremely low level. Strongly prefer using [`with`] instead."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`acquire`] for the safety contract description."] # [inline (always)] pub unsafe fn release (restore_state : RestoreState) { extern "Rust" { fn _critical_section_1_0_release (restore_state : RawRestoreState) ; } # [allow (clippy :: unit_arg)] _critical_section_1_0_release (restore_state . 0) }
};
}
