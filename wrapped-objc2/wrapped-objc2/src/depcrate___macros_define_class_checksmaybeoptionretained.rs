// Generated macro for MaybeOptionRetained (trait)
macro_rules! Depcrate___macros_define_class_checksMaybeOptionRetained {
() => {
// Module: crate::__macros::define_class::checks
// Provides: {"MaybeOptionRetained"}
// Dependencies: {}
# [doc = " Helper trait for specifying an `Retained<T>` or an `Option<Retained<T>>`."] # [doc = ""] # [doc = " (Both of those are valid return types from define_class!"] # [doc = " `#[unsafe(method_id)]`)."] pub trait MaybeOptionRetained { type Inner ; fn consumed_return (self) -> RetainedReturnValue ; fn autorelease_return (self) -> RetainedReturnValue ; }
};
}
