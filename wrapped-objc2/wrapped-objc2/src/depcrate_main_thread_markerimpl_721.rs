// Generated macro for impl_721 (impl)
macro_rules! Depcrate_main_thread_markerimpl_721 {
() => {
// Module: crate::main_thread_marker
// Provides: {"impl_721"}
// Dependencies: {}
# [doc = " Get a [`MainThreadMarker`] from a main-thread-only object."] # [doc = ""] # [doc = " This is a shorthand for [`MainThreadOnly::mtm`]."] impl < T : ? Sized + MainThreadOnly > From < & T > for MainThreadMarker { # [inline] fn from (obj : & T) -> Self { obj . mtm () } }
};
}
