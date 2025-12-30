// Generated macro for did_you_mean (function)
macro_rules! Depcrate_parser_features_suggestionsdid_you_mean {
() => {
// Module: crate::parser::features::suggestions
// Provides: {"did_you_mean"}
// Dependencies: {}
# [cfg (not (feature = "suggestions"))] pub (crate) fn did_you_mean < T , I > (_ : & str , _ : I) -> Vec < String > where T : AsRef < str > , I : IntoIterator < Item = T > , { Vec :: new () }
};
}
