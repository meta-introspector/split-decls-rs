// Generated macro for impl_228 (impl)
macro_rules! Depcrate_validatorsimpl_228 {
() => {
// Module: crate::validators
// Provides: {"impl_228"}
// Dependencies: {}
impl FromMeta for Number { fn from_value (value : & Lit) -> darling :: Result < Self > { match value { Lit :: Int (n) => Ok (Number :: I64 (n . base10_parse :: < i64 > () ?)) , Lit :: Float (n) => Ok (Number :: F64 (n . base10_parse :: < f64 > () ?)) , _ => Err (darling :: Error :: unexpected_type ("number")) , } } }
};
}
