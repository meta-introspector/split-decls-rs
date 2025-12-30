// Generated macro for linear_ints (function)
macro_rules! Depcrate_numlinear_ints {
() => {
// Module: crate::num
// Provides: {"linear_ints"}
// Dependencies: {}
# [doc = " Returns an iterator of up to `steps` integers evenly distributed."] pub fn linear_ints (range : RangeInclusive < i32 > , steps : u64 ,) -> (impl Iterator < Item = i32 > + Clone , u64) { let steps = steps . checked_sub (1) . unwrap () ; let between = u64 :: from (range . start () . abs_diff (* range . end ())) ; let spacing = i32 :: try_from ((between / steps) . max (1)) . unwrap () ; let steps = steps . min (between) ; let mut x : i32 = * range . start () ; ((0 ..= steps) . map (move | _ | { let res = x ; x = x . wrapping_add (spacing) ; res }) , steps + 1 ,) }
};
}
