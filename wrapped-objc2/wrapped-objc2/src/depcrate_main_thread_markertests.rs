// Generated macro for tests (module)
macro_rules! Depcrate_main_thread_markertests {
() => {
// Module: crate::main_thread_marker
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use std :: panic :: { RefUnwindSafe , UnwindSafe } ; static_assertions :: assert_impl_all ! (MainThreadMarker : Unpin , UnwindSafe , RefUnwindSafe , Sized) ; static_assertions :: assert_not_impl_any ! (MainThreadMarker : Send , Sync) ; # [test] fn debug () { let marker = unsafe { MainThreadMarker :: new_unchecked () } ; assert_eq ! (std :: format ! ("{marker:?}") , "MainThreadMarker") ; } # [test] fn test_not_main_thread () { let res = std :: thread :: spawn (| | MainThreadMarker :: new () . is_none ()) . join () . unwrap () ; assert ! (res) ; } }
};
}
