// Generated macro for impl_367 (impl)
macro_rules! Depcrate_meta_errorimpl_367 {
() => {
// Module: crate::meta::error
// Provides: {"impl_367"}
// Dependencies: {}
impl From < MatchError > for RetryFailError { fn from (merr : MatchError) -> RetryFailError { use crate :: util :: search :: MatchErrorKind :: * ; match * merr . kind () { Quit { offset , .. } => RetryFailError :: from_offset (offset) , GaveUp { offset } => RetryFailError :: from_offset (offset) , HaystackTooLong { .. } | UnsupportedAnchored { .. } => { unreachable ! ("found impossible error in meta engine: {merr}") } } } }
};
}
