// Generated macro for pairwise_sum (function)
macro_rules! Depcrate_weighted_weighted_aliaspairwise_sum {
() => {
// Module: crate::weighted::weighted_alias
// Provides: {"pairwise_sum"}
// Dependencies: {}
# [doc = " In comparison to naive accumulation, the pairwise sum algorithm reduces"] # [doc = " rounding errors when there are many floating point values."] fn pairwise_sum < T : AliasableWeight > (values : & [T]) -> T { if values . len () <= 32 { values . iter () . copied () . sum () } else { let mid = values . len () / 2 ; let (a , b) = values . split_at (mid) ; pairwise_sum (a) + pairwise_sum (b) } }
};
}
