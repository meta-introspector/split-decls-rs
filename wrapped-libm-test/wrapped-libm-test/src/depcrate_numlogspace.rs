// Generated macro for logspace (function)
macro_rules! Depcrate_numlogspace {
() => {
// Module: crate::num
// Provides: {"logspace"}
// Dependencies: {}
# [doc = " An iterator that returns floats with linearly spaced integer representations, which translates"] # [doc = " to logarithmic spacing of their values."] # [doc = ""] # [doc = " Note that this tends to skip negative zero, so that needs to be checked explicitly."] # [doc = ""] # [doc = " Returns `(iterator, iterator_length)`."] pub fn logspace < F : FloatExt > (start : F , end : F , steps : F :: Int ,) -> (impl Iterator < Item = F > + Clone , F :: Int) where RangeInclusive < F :: Int > : Iterator , { assert ! (! start . is_nan ()) ; assert ! (! end . is_nan ()) ; assert ! (end >= start) ; let steps = steps . checked_sub (F :: Int :: ONE) . expect ("`steps` must be at least 2") ; let between = ulp_between (start , end) . expect ("`start` or `end` is NaN") ; let spacing = (between / steps) . max (F :: Int :: ONE) ; let steps = steps . min (between) ; let mut x = start ; ((F :: Int :: ZERO ..= steps) . map (move | _ | { let ret = x ; x = x . n_up (spacing) ; ret }) , steps + F :: Int :: ONE ,) }
};
}
