// Generated macro for MainThreadOnlyDoesNotImplSendSync (trait)
macro_rules! Depcrate___macros_extern_class_checksMainThreadOnlyDoesNotImplSendSync {
() => {
// Module: crate::__macros::extern_class::checks
// Provides: {"MainThreadOnlyDoesNotImplSendSync"}
// Dependencies: {}
# [doc = " Check that `MainThreadOnly` types do not implement `Send` and `Sync`."] # [doc = ""] # [doc = " Check implemented using type inference:"] # [doc = " let _ = <MyType as MainThreadOnlyDoesNotImplSendSync<_>>::check"] pub trait MainThreadOnlyDoesNotImplSendSync < Inferred > { fn check () { } }
};
}
