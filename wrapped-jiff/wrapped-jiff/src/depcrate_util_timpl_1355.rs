// Generated macro for impl_1355 (impl)
macro_rules! Depcrate_util_timpl_1355 {
() => {
// Module: crate::util::t
// Provides: {"impl_1355"}
// Dependencies: {}
impl From < Constant > for i16 { fn from (c : Constant) -> i16 { # [cfg (not (debug_assertions))] { c . value () as i16 } # [cfg (debug_assertions)] { i16 :: try_from (c . value ()) . unwrap_or_else (| _ | { panic ! ("{c:?} is out of range {:?}..={:?}" , i16 :: MIN , i16 :: MAX) ; }) } } }
};
}
