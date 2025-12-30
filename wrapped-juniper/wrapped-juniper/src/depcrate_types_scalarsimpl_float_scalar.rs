// Generated macro for impl_float_scalar (module)
macro_rules! Depcrate_types_scalarsimpl_float_scalar {
() => {
// Module: crate::types::scalars
// Provides: {"impl_float_scalar"}
// Dependencies: {}
mod impl_float_scalar { use super :: * ; impl < 's , S > FromScalarValue < 's , S > for Float where S : TryToPrimitive < 's , Self , Error : IntoFieldError < S > > + 's , { type Error = S :: Error ; fn from_scalar_value (v : & 's S) -> Result < Self , Self :: Error > { v . try_to_primitive () } } pub (super) fn to_output < S : ScalarValue > (v : & Float) -> S { (* v) . into () } pub (super) fn parse_token < S : ScalarValue > (value : ScalarToken < '_ >) -> ParseScalarResult < S > { match value { ScalarToken :: Int (v) => v . parse () . map_err (| _ | ParseError :: unexpected_token (Token :: Scalar (value))) . map (| s : i32 | f64 :: from (s) . into ()) , ScalarToken :: Float (v) => v . parse () . map_err (| _ | ParseError :: unexpected_token (Token :: Scalar (value))) . map (| s : f64 | s . into ()) , ScalarToken :: String (_) => Err (ParseError :: unexpected_token (Token :: Scalar (value))) , } } }
};
}
