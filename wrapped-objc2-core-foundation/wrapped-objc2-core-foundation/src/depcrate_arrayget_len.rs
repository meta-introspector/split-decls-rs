// Generated macro for get_len (function)
macro_rules! Depcrate_arrayget_len {
() => {
// Module: crate::array
// Provides: {"get_len"}
// Dependencies: {}
# [inline] fn get_len < T > (objects : & [T]) -> CFIndex { let len = objects . len () ; debug_assert ! (len < CFIndex :: MAX as usize) ; len as CFIndex }
};
}
