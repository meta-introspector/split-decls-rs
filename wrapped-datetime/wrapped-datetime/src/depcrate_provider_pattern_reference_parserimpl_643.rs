// Generated macro for impl_643 (impl)
macro_rules! Depcrate_provider_pattern_reference_parserimpl_643 {
() => {
// Module: crate::provider::pattern::reference::parser
// Provides: {"impl_643"}
// Dependencies: {}
impl SegmentSecondSymbol { fn finish (self , result : & mut Vec < PatternItem >) -> Result < () , PatternError > { let second_symbol = FieldSymbol :: Second (fields :: Second :: Second) ; let symbol = if self . fraction_digits == 0 { second_symbol } else { let decimal_second = fields :: DecimalSecond :: from_idx (self . fraction_digits) . map_err (| _ | PatternError :: FieldLengthInvalid (second_symbol)) ? ; FieldSymbol :: DecimalSecond (decimal_second) } ; let length = FieldLength :: from_idx (self . integer_digits) . map_err (| _ | PatternError :: FieldLengthInvalid (symbol)) ? ; result . push (PatternItem :: Field (Field { symbol , length })) ; if self . seen_decimal_separator && self . fraction_digits == 0 { result . push (PatternItem :: Literal ('.')) ; } Ok (()) } }
};
}
