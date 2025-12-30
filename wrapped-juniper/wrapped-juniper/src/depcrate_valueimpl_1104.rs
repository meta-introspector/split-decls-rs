// Generated macro for impl_1104 (impl)
macro_rules! Depcrate_valueimpl_1104 {
() => {
// Module: crate::value
// Provides: {"impl_1104"}
// Dependencies: {}
impl < S : Clone > ToInputValue < S > for Value < S > { fn to_input_value (& self) -> InputValue < S > { match self { Self :: Null => InputValue :: Null , Self :: Scalar (s) => InputValue :: Scalar (s . clone ()) , Self :: List (l) => InputValue :: List (l . iter () . map (| x | Spanning :: unlocated (x . to_input_value ())) . collect () ,) , Self :: Object (o) => InputValue :: Object (o . iter () . map (| (k , v) | { (Spanning :: unlocated (k . clone ()) , Spanning :: unlocated (v . to_input_value ()) ,) }) . collect () ,) , } } }
};
}
