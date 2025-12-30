// Generated macro for impl_641 (impl)
macro_rules! Depcrate_provider_pattern_reference_parserimpl_641 {
() => {
// Module: crate::provider::pattern::reference::parser
// Provides: {"impl_641"}
// Dependencies: {}
impl SegmentSymbol { fn finish (self , result : & mut Vec < PatternItem >) -> Result < () , PatternError > { let length = FieldLength :: from_idx (self . length) . map_err (| _ | PatternError :: FieldLengthInvalid (self . symbol)) ? ; result . push (PatternItem :: from ((self . symbol , length))) ; Ok (()) } }
};
}
