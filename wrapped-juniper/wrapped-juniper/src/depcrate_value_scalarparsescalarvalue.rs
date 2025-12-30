// Generated macro for ParseScalarValue (trait)
macro_rules! Depcrate_value_scalarParseScalarValue {
() => {
// Module: crate::value::scalar
// Provides: {"ParseScalarValue"}
// Dependencies: {}
# [doc = " A trait used to convert a `ScalarToken` into a certain scalar value type"] pub trait ParseScalarValue < S = DefaultScalarValue > { # [doc = " See the trait documentation"] fn from_str (value : ScalarToken < '_ >) -> ParseScalarResult < S > ; }
};
}
