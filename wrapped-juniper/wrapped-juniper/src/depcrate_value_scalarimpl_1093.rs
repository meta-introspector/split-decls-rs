// Generated macro for impl_1093 (impl)
macro_rules! Depcrate_value_scalarimpl_1093 {
() => {
// Module: crate::value::scalar
// Provides: {"impl_1093"}
// Dependencies: {}
impl < S : ScalarValue > Display for Scalar < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (s) = self . 0 . try_as_str () { write ! (f , "\"{s}\"") } else { Display :: fmt (& self . 0 , f) } } }
};
}
