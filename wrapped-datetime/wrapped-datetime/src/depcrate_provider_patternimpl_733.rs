// Generated macro for impl_733 (impl)
macro_rules! Depcrate_provider_patternimpl_733 {
() => {
// Module: crate::provider::pattern
// Provides: {"impl_733"}
// Dependencies: {}
impl From < PatternItem > for TimeGranularity { # [doc = " Retrieves the granularity of time represented by a [`PatternItem`]."] # [doc = " If the [`PatternItem`] is not time-related, returns [`None`]."] fn from (item : PatternItem) -> Self { match item { PatternItem :: Field (field) => match field . symbol { fields :: FieldSymbol :: Hour (_) => Self :: Hours , fields :: FieldSymbol :: Minute => Self :: Minutes , fields :: FieldSymbol :: Second (_) => Self :: Seconds , fields :: FieldSymbol :: DecimalSecond (_) => Self :: Nanoseconds , _ => Self :: None , } , _ => Self :: None , } } }
};
}
