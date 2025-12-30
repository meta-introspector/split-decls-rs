// Generated macro for impl_108 (impl)
macro_rules! Depcrate_timezone_implimpl_108 {
() => {
// Module: crate::timezone_impl
// Provides: {"impl_108"}
// Dependencies: {}
impl TzOffset { fn new (tz : Tz , offset : FixedTimespan) -> Self { TzOffset { tz , offset } } fn map_localresult (tz : Tz , result : LocalResult < FixedTimespan >) -> LocalResult < Self > { match result { LocalResult :: None => LocalResult :: None , LocalResult :: Single (s) => LocalResult :: Single (TzOffset :: new (tz , s)) , LocalResult :: Ambiguous (a , b) => { LocalResult :: Ambiguous (TzOffset :: new (tz , a) , TzOffset :: new (tz , b)) } } } }
};
}
