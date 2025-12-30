// Generated macro for WeakInner (struct)
macro_rules! Depcrate_arcWeakInner {
() => {
// Module: crate::arc
// Provides: {"WeakInner"}
// Dependencies: {}
# [doc = " Helper type to allow accessing the reference counts without"] # [doc = " making any assertions about the data field."] struct WeakInner < 'a > { weak : & 'a atomic :: AtomicUsize , strong : & 'a atomic :: AtomicUsize , }
};
}
