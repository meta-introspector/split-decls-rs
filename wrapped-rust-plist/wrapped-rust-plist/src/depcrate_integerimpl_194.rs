// Generated macro for impl_194 (impl)
macro_rules! Depcrate_integerimpl_194 {
() => {
// Module: crate::integer
// Provides: {"impl_194"}
// Dependencies: {}
impl Integer { # [doc = " Returns the value as an `i64` if it can be represented by that type."] pub fn as_signed (self) -> Option < i64 > { i64 :: try_from (self . value) . ok () } # [doc = " Returns the value as a `u64` if it can be represented by that type."] pub fn as_unsigned (self) -> Option < u64 > { u64 :: try_from (self . value) . ok () } pub (crate) fn from_str (s : & str) -> Result < Self , ParseIntError > { if s . starts_with ("0x") { let s = s . trim_start_matches ("0x") ; u64 :: from_str_radix (s , 16) . map (Into :: into) } else { Ok (match s . parse :: < i64 > () { Ok (v) => v . into () , Err (_) => s . parse :: < u64 > () ? . into () , }) } } }
};
}
