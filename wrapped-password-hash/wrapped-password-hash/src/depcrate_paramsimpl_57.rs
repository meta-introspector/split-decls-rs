// Generated macro for impl_57 (impl)
macro_rules! Depcrate_paramsimpl_57 {
() => {
// Module: crate::params
// Provides: {"impl_57"}
// Dependencies: {}
impl FromStr for ParamsString { type Err = Error ; fn from_str (s : & str) -> Result < Self > { if s . len () > MAX_LENGTH { return Err (Error :: ParamsMaxExceeded) ; } if s . is_empty () { return Ok (ParamsString :: new ()) ; } for mut param in s . split (PARAMS_DELIMITER) . map (| p | p . split (PAIR_DELIMITER)) { param . next () . ok_or (Error :: ParamNameInvalid) . and_then (Ident :: try_from) ? ; param . next () . ok_or (Error :: ParamValueInvalid (InvalidValue :: Malformed)) . and_then (Value :: try_from) ? ; if param . next () . is_some () { return Err (Error :: ParamValueInvalid (InvalidValue :: Malformed)) ; } } let mut bytes = [0u8 ; MAX_LENGTH] ; bytes [.. s . len ()] . copy_from_slice (s . as_bytes ()) ; Ok (Self (Buffer { bytes , length : s . len () as u8 , })) } }
};
}
