// Generated macro for impl_boolean_scalar (module)
macro_rules! Depcrate_types_scalarsimpl_boolean_scalar {
() => {
// Module: crate::types::scalars
// Provides: {"impl_boolean_scalar"}
// Dependencies: {}
mod impl_boolean_scalar { use super :: * ; impl < 's , S > FromScalarValue < 's , S > for Boolean where S : TryToPrimitive < 's , Self , Error : IntoFieldError < S > > + 's , { type Error = S :: Error ; fn from_scalar_value (v : & 's S) -> Result < Self , Self :: Error > { v . try_to_primitive () } } pub (super) fn to_output < S : ScalarValue > (v : & Boolean) -> S { (* v) . into () } pub (super) fn parse_token < S : ScalarValue > (value : ScalarToken < '_ >) -> ParseScalarResult < S > { Err (ParseError :: unexpected_token (Token :: Scalar (value))) } }
};
}
