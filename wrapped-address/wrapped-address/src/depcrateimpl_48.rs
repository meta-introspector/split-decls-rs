// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
# [cfg (feature = "decode")] impl FromStr for Address { type Err = ParseAddressError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { use five8 :: DecodeError ; if s . len () > MAX_BASE58_LEN { return Err (ParseAddressError :: WrongSize) ; } let mut bytes = [0 ; ADDRESS_BYTES] ; five8 :: decode_32 (s , & mut bytes) . map_err (| e | match e { DecodeError :: InvalidChar (_) => ParseAddressError :: Invalid , DecodeError :: TooLong | DecodeError :: TooShort | DecodeError :: LargestTermTooHigh | DecodeError :: OutputTooLong => ParseAddressError :: WrongSize , }) ? ; Ok (Address (bytes)) } }
};
}
