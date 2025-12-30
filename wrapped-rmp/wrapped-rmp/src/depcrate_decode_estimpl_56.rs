// Generated macro for impl_56 (impl)
macro_rules! Depcrate_decode_estimpl_56 {
() => {
// Module: crate::decode::est
// Provides: {"impl_56"}
// Dependencies: {}
impl LenError { # [doc = " Get expected min length or 0 on error"] pub fn len (& self) -> usize { match * self { Self :: ParseError => 0 , Self :: Truncated (l) => l . get () , } } }
};
}
