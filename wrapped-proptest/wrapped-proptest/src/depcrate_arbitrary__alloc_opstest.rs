// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__alloc_opstest {
() => {
// Module: crate::arbitrary::_alloc::ops
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { no_panic_test ! (range_full => RangeFull , range_from => RangeFrom < usize >, range_to => RangeTo < usize >, range => Range < usize >, range_inclusive => RangeInclusive < usize >, range_to_inclusive => RangeToInclusive < usize >) ; # [cfg (feature = "unstable")] no_panic_test ! (generator_state => CoroutineState < u32 , u64 >) ; }
};
}
