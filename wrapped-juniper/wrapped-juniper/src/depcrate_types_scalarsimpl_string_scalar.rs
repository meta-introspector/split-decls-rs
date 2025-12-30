// Generated macro for impl_string_scalar (module)
macro_rules! Depcrate_types_scalarsimpl_string_scalar {
() => {
// Module: crate::types::scalars
// Provides: {"impl_string_scalar"}
// Dependencies: {}
mod impl_string_scalar { use super :: * ; impl < 's , S > FromScalarValue < 's , S > for String where S : TryToPrimitive < 's , Self , Error : IntoFieldError < S > > + 's , { type Error = S :: Error ; fn from_scalar_value (v : & 's S) -> Result < Self , Self :: Error > { v . try_to_primitive () } } pub (super) fn parse_token < S : ScalarValue > (value : ScalarToken < '_ >) -> ParseScalarResult < S > { if let ScalarToken :: String (lit) = value { let parsed = lit . parse () ? ; Ok (parsed . into_owned () . into ()) } else { Err (ParseError :: unexpected_token (Token :: Scalar (value))) } } }
};
}
