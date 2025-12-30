// Generated macro for impl_353 (impl)
macro_rules! Depcrate_meta_errorimpl_353 {
() => {
// Module: crate::meta::error
// Provides: {"impl_353"}
// Dependencies: {}
impl From < MatchError > for RetryError { fn from (merr : MatchError) -> RetryError { RetryError :: Fail (RetryFailError :: from (merr)) } }
};
}
