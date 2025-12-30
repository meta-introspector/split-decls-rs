// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
# [cfg (feature = "decode")] impl FromStr for Hash { type Err = ParseHashError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { use five8 :: DecodeError ; if s . len () > MAX_BASE58_LEN { return Err (ParseHashError :: WrongSize) ; } let mut bytes = [0 ; HASH_BYTES] ; five8 :: decode_32 (s , & mut bytes) . map_err (| e | match e { DecodeError :: InvalidChar (_) => ParseHashError :: Invalid , DecodeError :: TooLong | DecodeError :: TooShort | DecodeError :: LargestTermTooHigh | DecodeError :: OutputTooLong => ParseHashError :: WrongSize , }) ? ; Ok (Self :: from (bytes)) } }
};
}
