// Generated macro for impl_22 (impl)
macro_rules! Depcrate_errorimpl_22 {
() => {
// Module: crate::error
// Provides: {"impl_22"}
// Dependencies: {}
impl < E , T > ResultExt < E , T > for Result < E , T > { fn committed (self) -> ParseResult < E , T > { match self { Ok (x) => CommitOk (x) , Err (x) => CommitErr (x) , } } }
};
}
