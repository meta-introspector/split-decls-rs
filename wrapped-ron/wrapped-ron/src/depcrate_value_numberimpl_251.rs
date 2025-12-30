// Generated macro for impl_251 (impl)
macro_rules! Depcrate_value_numberimpl_251 {
() => {
// Module: crate::value::number
// Provides: {"impl_251"}
// Dependencies: {}
impl Number { # [doc = " Construct a new number."] pub fn new (v : impl Into < Number >) -> Self { v . into () } # [doc = " Returns the [`f64`] representation of the [`Number`] regardless of"] # [doc = " whether the number is stored as a float or integer."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use ron::value::Number;"] # [doc = " let i = Number::new(5);"] # [doc = " let f = Number::new(2.0);"] # [doc = " assert_eq!(i.into_f64(), 5.0);"] # [doc = " assert_eq!(f.into_f64(), 2.0);"] # [doc = " ```"] # [must_use] pub fn into_f64 (self) -> f64 { # [allow (clippy :: cast_precision_loss)] match self { Self :: I8 (v) => f64 :: from (v) , Self :: I16 (v) => f64 :: from (v) , Self :: I32 (v) => f64 :: from (v) , Self :: I64 (v) => v as f64 , # [cfg (feature = "integer128")] Self :: I128 (v) => v as f64 , Self :: U8 (v) => f64 :: from (v) , Self :: U16 (v) => f64 :: from (v) , Self :: U32 (v) => f64 :: from (v) , Self :: U64 (v) => v as f64 , # [cfg (feature = "integer128")] Self :: U128 (v) => v as f64 , Self :: F32 (v) => f64 :: from (v . get ()) , Self :: F64 (v) => v . get () , # [cfg (not (doc))] Self :: __NonExhaustive (never) => never . never () , } } }
};
}
