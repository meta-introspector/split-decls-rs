// Generated macro for impl_364 (impl)
macro_rules! Depcrate_meta_errorimpl_364 {
() => {
// Module: crate::meta::error
// Provides: {"impl_364"}
// Dependencies: {}
impl From < MatchError > for RetryFailError { fn from (merr : MatchError) -> RetryFailError { use crate :: util :: search :: MatchErrorKind :: * ; match * merr . kind () { Quit { offset , .. } => RetryFailError :: from_offset (offset) , GaveUp { offset } => RetryFailError :: from_offset (offset) , HaystackTooLong { .. } | UnsupportedAnchored { .. } => { unreachable ! ("found impossible error in meta engine: {}" , merr) } } } }
};
}
