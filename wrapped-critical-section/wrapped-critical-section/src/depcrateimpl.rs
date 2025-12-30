// Generated macro for Impl (trait)
macro_rules! DepcrateImpl {
() => {
// Module: crate
// Provides: {"Impl"}
// Dependencies: {}
# [doc = " Methods required for a critical section implementation."] # [doc = ""] # [doc = " This trait is not intended to be used except when implementing a critical section."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Implementations must uphold the contract specified in [`crate::acquire`] and [`crate::release`]."] pub unsafe trait Impl { # [doc = " Acquire the critical section."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must uphold the contract specified in [`crate::acquire`] and [`crate::release`]."] unsafe fn acquire () -> RawRestoreState ; # [doc = " Release the critical section."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must uphold the contract specified in [`crate::acquire`] and [`crate::release`]."] unsafe fn release (restore_state : RawRestoreState) ; }
};
}
