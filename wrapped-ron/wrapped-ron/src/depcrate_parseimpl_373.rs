// Generated macro for impl_373 (impl)
macro_rules! Depcrate_parseimpl_373 {
() => {
// Module: crate::parse
// Provides: {"impl_373"}
// Dependencies: {}
impl Integer for ParsedInteger { fn parse (parser : & mut Parser , sign : i8) -> Result < Self > { if sign < 0 { let signed = parser . parse_integer :: < LargeSInt > (- 1) ? ; return if let Ok (x) = i8 :: try_from (signed) { Ok (ParsedInteger :: I8 (x)) } else if let Ok (x) = i16 :: try_from (signed) { Ok (ParsedInteger :: I16 (x)) } else if let Ok (x) = i32 :: try_from (signed) { Ok (ParsedInteger :: I32 (x)) } else { # [cfg (not (feature = "integer128"))] { Ok (ParsedInteger :: I64 (signed)) } # [cfg (feature = "integer128")] if let Ok (x) = i64 :: try_from (signed) { Ok (ParsedInteger :: I64 (x)) } else { Ok (ParsedInteger :: I128 (signed)) } } ; } let unsigned = parser . parse_integer :: < LargeUInt > (1) ? ; if let Ok (x) = u8 :: try_from (unsigned) { Ok (ParsedInteger :: U8 (x)) } else if let Ok (x) = u16 :: try_from (unsigned) { Ok (ParsedInteger :: U16 (x)) } else if let Ok (x) = u32 :: try_from (unsigned) { Ok (ParsedInteger :: U32 (x)) } else { # [cfg (not (feature = "integer128"))] { Ok (ParsedInteger :: U64 (unsigned)) } # [cfg (feature = "integer128")] if let Ok (x) = u64 :: try_from (unsigned) { Ok (ParsedInteger :: U64 (x)) } else { Ok (ParsedInteger :: U128 (unsigned)) } } } fn try_from_parsed_integer (parsed : ParsedInteger , _ron : & str) -> Result < Self > { Ok (parsed) } }
};
}
