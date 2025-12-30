// Generated macro for Extractor (struct)
macro_rules! Depcrate_literalExtractor {
() => {
// Module: crate::literal
// Provides: {"Extractor"}
// Dependencies: {}
# [doc = " An inner literal extractor."] # [doc = ""] # [doc = " This is a somewhat stripped down version of the extractor from"] # [doc = " regex-syntax. The main difference is that we try to identify a \"best\" set"] # [doc = " of required literals while traversing the HIR."] # [derive (Debug)] struct Extractor { limit_class : usize , limit_repeat : usize , limit_literal_len : usize , limit_total : usize , }
};
}
