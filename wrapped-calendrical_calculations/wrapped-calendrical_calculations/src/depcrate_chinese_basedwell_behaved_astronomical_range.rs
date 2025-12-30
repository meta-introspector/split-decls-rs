// Generated macro for WELL_BEHAVED_ASTRONOMICAL_RANGE (const)
macro_rules! Depcrate_chinese_basedWELL_BEHAVED_ASTRONOMICAL_RANGE {
() => {
// Module: crate::chinese_based
// Provides: {"WELL_BEHAVED_ASTRONOMICAL_RANGE"}
// Dependencies: {}
# [doc = " For astronomical calendars in this module, the range in which they are expected to be well-behaved."] # [doc = ""] # [doc = " With astronomical calendars, for dates in the far past or far future, floating point error, algorithm inaccuracies,"] # [doc = " and other issues may cause the calendar algorithm to behave unexpectedly."] # [doc = ""] # [doc = " Our code has a number of debug assertions for various calendrical invariants (for example, lunar calendar months"] # [doc = " must be 29 or 30 days), but it will turn these off outside of these ranges."] # [doc = ""] # [doc = " Consumers of this code are encouraged to disallow such out-of-range values; or, if allowing them, not expect too"] # [doc = " much in terms of calendrical invariants. Once we have proleptic approximations of these calendars (#5778),"] # [doc = " developers will be encouraged to use them when dates are out of range."] # [doc = ""] # [doc = " This value is not stable and may change. It's currently somewhat arbitrarily chosen to be"] # [doc = " approximately ±10,000 years from 0 CE."] pub const WELL_BEHAVED_ASTRONOMICAL_RANGE : Range < RataDie > = RataDie :: new (365 * - 10_000) .. RataDie :: new (365 * 10_000) ;
};
}
