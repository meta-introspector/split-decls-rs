// Generated macro for set_impl (macro)
macro_rules! Depcrateset_impl {
() => {
// Module: crate
// Provides: {"set_impl"}
// Dependencies: {}
# [doc = " Set the critical section implementation."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(not(feature = \"std\"))] // needed for `cargo test --features std`"] # [doc = " # mod no_std {"] # [doc = " use critical_section::RawRestoreState;"] # [doc = ""] # [doc = " struct MyCriticalSection;"] # [doc = " critical_section::set_impl!(MyCriticalSection);"] # [doc = ""] # [doc = " unsafe impl critical_section::Impl for MyCriticalSection {"] # [doc = "     unsafe fn acquire() -> RawRestoreState {"] # [doc = "         // ..."] # [doc = "     }"] # [doc = ""] # [doc = "     unsafe fn release(restore_state: RawRestoreState) {"] # [doc = "         // ..."] # [doc = "     }"] # [doc = " }"] # [doc = " # }"] # [macro_export] macro_rules ! set_impl { ($ t : ty) => { # [no_mangle] unsafe fn _critical_section_1_0_acquire () -> $ crate :: RawRestoreState { <$ t as $ crate :: Impl >:: acquire () } # [no_mangle] unsafe fn _critical_section_1_0_release (restore_state : $ crate :: RawRestoreState) { <$ t as $ crate :: Impl >:: release (restore_state) } } ; }
};
}
