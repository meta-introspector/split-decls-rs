// Generated macro for impl_1354 (impl)
macro_rules! Depcrate_util_timpl_1354 {
() => {
// Module: crate::util::t
// Provides: {"impl_1354"}
// Dependencies: {}
impl From < Constant > for i8 { fn from (c : Constant) -> i8 { # [cfg (not (debug_assertions))] { c . value () as i8 } # [cfg (debug_assertions)] { i8 :: try_from (c . value ()) . unwrap_or_else (| _ | { panic ! ("{c:?} is out of range {:?}..={:?}" , i8 :: MIN , i8 :: MAX) ; }) } } }
};
}
