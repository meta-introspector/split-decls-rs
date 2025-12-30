// Generated macro for impl_356 (impl)
macro_rules! Depcrate_meta_errorimpl_356 {
() => {
// Module: crate::meta::error
// Provides: {"impl_356"}
// Dependencies: {}
impl From < MatchError > for RetryError { fn from (merr : MatchError) -> RetryError { RetryError :: Fail (RetryFailError :: from (merr)) } }
};
}
