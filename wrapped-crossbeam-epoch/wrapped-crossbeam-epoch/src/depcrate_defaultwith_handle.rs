// Generated macro for with_handle (function)
macro_rules! Depcrate_defaultwith_handle {
() => {
// Module: crate::default
// Provides: {"with_handle"}
// Dependencies: {}
# [inline] fn with_handle < F , R > (mut f : F) -> R where F : FnMut (& LocalHandle) -> R , { HANDLE . try_with (| h | f (h)) . unwrap_or_else (| _ | f (& collector () . register ())) }
};
}
