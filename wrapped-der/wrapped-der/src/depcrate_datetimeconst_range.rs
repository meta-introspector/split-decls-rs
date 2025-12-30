// Generated macro for const_range (module)
macro_rules! Depcrate_datetimeconst_range {
() => {
// Module: crate::datetime
// Provides: {"const_range"}
// Dependencies: {}
mod const_range { use core :: ops :: RangeInclusive ; # [doc = " const [`RangeInclusive::contains`]"] # [inline] pub const fn const_contains_u8 (range : RangeInclusive < u8 > , item : u8) -> bool { item >= * range . start () && item <= * range . end () } }
};
}
