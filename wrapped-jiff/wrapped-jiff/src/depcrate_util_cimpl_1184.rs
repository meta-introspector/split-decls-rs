// Generated macro for impl_1184 (impl)
macro_rules! Depcrate_util_cimpl_1184 {
() => {
// Module: crate::util::c
// Provides: {"impl_1184"}
// Dependencies: {}
impl Sign { pub (crate) fn is_negative (& self) -> bool { matches ! (* self , Sign :: Negative) } pub (crate) fn as_ranged_integer (& self) -> t :: Sign { match * self { Sign :: Zero => t :: Sign :: N :: < 0 > () , Sign :: Positive => t :: Sign :: N :: < 1 > () , Sign :: Negative => t :: Sign :: N :: < - 1 > () , } } }
};
}
