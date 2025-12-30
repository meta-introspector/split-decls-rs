// Generated macro for impl_1356 (impl)
macro_rules! Depcrate_util_timpl_1356 {
() => {
// Module: crate::util::t
// Provides: {"impl_1356"}
// Dependencies: {}
impl From < Constant > for i32 { fn from (c : Constant) -> i32 { # [cfg (not (debug_assertions))] { c . value () as i32 } # [cfg (debug_assertions)] { i32 :: try_from (c . value ()) . unwrap_or_else (| _ | { panic ! ("{c:?} is out of range {:?}..={:?}" , i32 :: MIN , i32 :: MAX) ; }) } } }
};
}
