// Generated macro for impl_1385 (impl)
macro_rules! Depcrate_range_inclusiveimpl_1385 {
() => {
// Module: crate::range_inclusive
// Provides: {"impl_1385"}
// Dependencies: {}
impl < T > Iter < T > where RangeInclusive < T > : Eq , T : Ord + Copy , { # [doc = " Returns `Some((start, end))` for `start..=end`, or `None` if it is exhausted."] # [doc = ""] # [doc = " Note that `RangeInclusive` does not specify the bounds of an exhausted iterator,"] # [doc = " so this is a way for us to figure out what we've got.  Thankfully, all of the"] # [doc = " integer types we care about can be trivially cloned."] fn bounds (& self) -> Option < (T , T) > { let start = * self . range . start () ; let end = * self . range . end () ; if start <= end && self . range == (start ..= end) { Some ((start , end)) } else { None } } }
};
}
