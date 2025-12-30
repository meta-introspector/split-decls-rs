// Generated macro for ThreadKindAutoTraits (struct)
macro_rules! Depcrate___macros_define_class_checksThreadKindAutoTraits {
() => {
// Module: crate::__macros::define_class::checks
// Provides: {"ThreadKindAutoTraits"}
// Dependencies: {}
# [doc = " Helper for determining auto traits of defined classes."] # [doc = ""] # [doc = " This will contain either `dyn AnyThread` or `dyn MainThreadOnly`, so it"] # [doc = " will have no auto traits by default."] # [derive (Debug)] pub struct ThreadKindAutoTraits < T : ? Sized > (T) ;
};
}
