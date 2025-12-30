// Generated macro for AliasableWeight (trait)
macro_rules! Depcrate_weighted_weighted_aliasAliasableWeight {
() => {
// Module: crate::weighted::weighted_alias
// Provides: {"AliasableWeight"}
// Dependencies: {}
# [doc = " Weight bound for [`WeightedAliasIndex`]"] # [doc = ""] # [doc = " Currently no guarantees on the correctness of [`WeightedAliasIndex`] are"] # [doc = " given for custom implementations of this trait."] pub trait AliasableWeight : Sized + Copy + SampleUniform + PartialOrd + Add < Output = Self > + AddAssign + Sub < Output = Self > + SubAssign + Mul < Output = Self > + MulAssign + Div < Output = Self > + DivAssign + Sum { # [doc = " Maximum number representable by `Self`."] const MAX : Self ; # [doc = " Element of `Self` equivalent to 0."] const ZERO : Self ; # [doc = " Produce an instance of `Self` from a `u32` value, or return `None` if"] # [doc = " out of range. Loss of precision (where `Self` is a floating point type)"] # [doc = " is acceptable."] fn try_from_u32_lossy (n : u32) -> Option < Self > ; # [doc = " Sums all values in slice `values`."] fn sum (values : & [Self]) -> Self { values . iter () . copied () . sum () } }
};
}
