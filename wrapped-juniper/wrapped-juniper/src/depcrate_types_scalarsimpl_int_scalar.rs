// Generated macro for impl_int_scalar (module)
macro_rules! Depcrate_types_scalarsimpl_int_scalar {
() => {
// Module: crate::types::scalars
// Provides: {"impl_int_scalar"}
// Dependencies: {}
mod impl_int_scalar { use super :: * ; impl < 's , S > FromScalarValue < 's , S > for Int where S : TryToPrimitive < 's , Self , Error : IntoFieldError < S > > + 's , { type Error = S :: Error ; fn from_scalar_value (v : & 's S) -> Result < Self , Self :: Error > { v . try_to_primitive () } } pub (super) fn to_output < S : ScalarValue > (v : & Int) -> S { (* v) . into () } pub (super) fn parse_token < S : ScalarValue > (value : ScalarToken < '_ >) -> ParseScalarResult < S > { if let ScalarToken :: Int (v) = value { v . parse () . map_err (| _ | ParseError :: unexpected_token (Token :: Scalar (value))) . map (| s : i32 | s . into ()) } else { Err (ParseError :: unexpected_token (Token :: Scalar (value))) } } }
};
}
