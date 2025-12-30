// Generated macro for impl_250 (impl)
macro_rules! Depcrate_pattern_formatterimpl_250 {
() => {
// Module: crate::pattern::formatter
// Provides: {"impl_250"}
// Dependencies: {}
impl < 'a , C : CldrCalendar , FSet > DateTimePatternFormatter < 'a , C , FSet > { pub (crate) fn new (pattern : DateTimePatternBorrowed < 'a > , names : RawDateTimeNamesBorrowed < 'a > ,) -> Self { Self { inner : RawDateTimePatternFormatter { pattern , names } , _calendar : PhantomData , _marker : PhantomData , } } }
};
}
