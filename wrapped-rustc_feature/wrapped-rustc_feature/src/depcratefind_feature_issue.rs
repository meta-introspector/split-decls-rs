// Generated macro for find_feature_issue (function)
macro_rules! Depcratefind_feature_issue {
() => {
// Module: crate
// Provides: {"find_feature_issue"}
// Dependencies: {}
pub fn find_feature_issue (feature : Symbol , issue : GateIssue) -> Option < NonZero < u32 > > { match issue { GateIssue :: Language => find_lang_feature_issue (feature) , GateIssue :: Library (lib) => lib , } }
};
}
