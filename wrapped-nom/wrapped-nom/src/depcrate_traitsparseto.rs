// Generated macro for ParseTo (trait)
macro_rules! Depcrate_traitsParseTo {
() => {
// Module: crate::traits
// Provides: {"ParseTo"}
// Dependencies: {}
# [doc = " Used to integrate `str`'s `parse()` method"] pub trait ParseTo < R > { # [doc = " Succeeds if `parse()` succeeded. The byte slice implementation"] # [doc = " will first convert it to a `&str`, then apply the `parse()` function"] fn parse_to (& self) -> Option < R > ; }
};
}
