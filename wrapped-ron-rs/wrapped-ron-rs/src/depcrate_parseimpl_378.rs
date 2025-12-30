// Generated macro for impl_378 (impl)
macro_rules! Depcrate_parseimpl_378 {
() => {
// Module: crate::parse
// Provides: {"impl_378"}
// Dependencies: {}
impl Float for ParsedFloat { fn parse (float : & str) -> Result < Self > { let value = f64 :: from_str (float) . map_err (| _ | Error :: ExpectedFloat) ? ; # [allow (clippy :: cast_possible_truncation)] if value . total_cmp (& f64 :: from (value as f32)) . is_eq () { Ok (ParsedFloat :: F32 (value as f32)) } else { Ok (ParsedFloat :: F64 (value)) } } fn try_from_parsed_float (parsed : ParsedFloat , _ron : & str) -> Result < Self > { Ok (parsed) } }
};
}
