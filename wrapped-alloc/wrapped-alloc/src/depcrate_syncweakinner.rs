// Generated macro for WeakInner (struct)
macro_rules! Depcrate_syncWeakInner {
() => {
// Module: crate::sync
// Provides: {"WeakInner"}
// Dependencies: {}
# [doc = " Helper type to allow accessing the reference counts without"] # [doc = " making any assertions about the data field."] struct WeakInner < 'a > { weak : & 'a Atomic < usize > , strong : & 'a Atomic < usize > , }
};
}
