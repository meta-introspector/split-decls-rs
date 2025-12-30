// Generated macro for impl_650 (impl)
macro_rules! Depcrate_statsimpl_650 {
() => {
// Module: crate::stats
// Provides: {"impl_650"}
// Dependencies: {}
impl < A > Distribution < A > where A : Float , { # [doc = " Create a distribution from the given values"] pub fn from (values : Box < [A] >) -> Distribution < A > { Distribution (values) } # [doc = " Computes the confidence interval of the population parameter using percentiles"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the `confidence_level` is not in the `(0, 1)` range."] pub fn confidence_interval (& self , confidence_level : A) -> (A , A) where usize : cast :: From < A , Output = Result < usize , cast :: Error > > , { let _0 = A :: cast (0) ; let _1 = A :: cast (1) ; let _50 = A :: cast (50) ; assert ! (confidence_level > _0 && confidence_level < _1) ; let percentiles = self . percentiles () ; (percentiles . at (_50 * (_1 - confidence_level)) , percentiles . at (_50 * (_1 + confidence_level)) ,) } # [doc = " Computes the \"likelihood\" of seeing the value `t` or \"more extreme\" values in the"] # [doc = " distribution."] pub fn p_value (& self , t : A , tails : & Tails) -> A { use std :: cmp ; let n = self . 0 . len () ; let hits = self . 0 . iter () . filter (| & & x | x < t) . count () ; let tails = A :: cast (match * tails { Tails :: One => 1 , Tails :: Two => 2 , }) ; A :: cast (cmp :: min (hits , n - hits)) / A :: cast (n) * tails } }
};
}
