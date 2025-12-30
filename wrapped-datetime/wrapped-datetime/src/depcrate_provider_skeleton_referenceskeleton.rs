// Generated macro for Skeleton (struct)
macro_rules! Depcrate_provider_skeleton_referenceSkeleton {
() => {
// Module: crate::provider::skeleton::reference
// Provides: {"Skeleton"}
// Dependencies: {}
# [doc = " A [`Skeleton`] is used to represent what types of `Field`s are present in a [`Pattern`]. The"] # [doc = " ordering of the [`Skeleton`]'s `Field`s have no bearing on the ordering of the `Field`s and"] # [doc = " `Literal`s in the [`Pattern`]."] # [doc = ""] # [doc = " A [`Skeleton`] is a [`Vec`]`<Field>`, but with the invariant that it is sorted according to the canonical"] # [doc = " sort order. This order is sorted according to the most significant `Field` to the least significant."] # [doc = " For example, a field with a `Minute` symbol would precede a field with a `Second` symbol."] # [doc = " This order is documented as the order of fields as presented in the"] # [doc = " [UTS 35 Date Field Symbol Table](https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table)"] # [doc = ""] # [doc = " The `Field`s are only sorted in the [`Skeleton`] in order to provide a deterministic"] # [doc = " serialization strategy, and to provide a faster [`Skeleton`] matching operation."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , Eq , PartialEq , Clone , Ord , PartialOrd)] pub struct Skeleton (pub (crate) SmallVec < [fields :: Field ; 5] >) ;
};
}
