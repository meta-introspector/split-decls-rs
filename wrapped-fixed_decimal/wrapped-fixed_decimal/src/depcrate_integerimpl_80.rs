// Generated macro for impl_80 (impl)
macro_rules! Depcrate_integerimpl_80 {
() => {
// Module: crate::integer
// Provides: {"impl_80"}
// Dependencies: {}
impl FixedInteger { # [inline] pub fn try_from_str (s : & str) -> Result < Self , ParseError > { Self :: try_from_utf8 (s . as_bytes ()) } pub fn try_from_utf8 (code_units : & [u8]) -> Result < Self , ParseError > { FixedInteger :: try_from (Decimal :: try_from_utf8 (code_units) ?) . map_err (| LimitError | ParseError :: Limit) } }
};
}
