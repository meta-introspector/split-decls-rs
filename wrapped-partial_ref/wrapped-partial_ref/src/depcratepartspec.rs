// Generated macro for PartSpec (trait)
macro_rules! DepcratePartSpec {
() => {
// Module: crate
// Provides: {"PartSpec"}
// Dependencies: {}
# [doc = " Helper trait to strip lifetimes from a part."] # [doc = ""] # [doc = " Every part `SomePart<'a, ...>` should implement `PartSpec<SomePart<'b, ...>>`. This is used by"] # [doc = " the [`part`](PartialRef::part), [`part_mut`](PartialRef::part_mut),"] # [doc = " [`split_part`](PartialRef::split_part) and [`split_part_mut`](PartialRef::split_part_mut)"] # [doc = " functions. This allows the passed parameter to have a different lifetime than the accessed part."] # [doc = " This in turn enables part selection using globals with static lifetimes as declared by the"] # [doc = " [`part`] macro."] pub trait PartSpec < Part > { }
};
}
