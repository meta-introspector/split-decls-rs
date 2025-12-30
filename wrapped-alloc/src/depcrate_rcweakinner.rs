// Generated macro for WeakInner (struct)
macro_rules! Depcrate_rcWeakInner {
() => {
// Module: crate::rc
// Provides: {"WeakInner"}
// Dependencies: {}
# [doc = " Helper type to allow accessing the reference counts without"] # [doc = " making any assertions about the data field."] struct WeakInner < 'a > { weak : & 'a Cell < usize > , strong : & 'a Cell < usize > , }
};
}
