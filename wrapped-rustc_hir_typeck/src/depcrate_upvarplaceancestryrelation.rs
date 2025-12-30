// Generated macro for PlaceAncestryRelation (enum)
macro_rules! Depcrate_upvarPlaceAncestryRelation {
() => {
// Module: crate::upvar
// Provides: {"PlaceAncestryRelation"}
// Dependencies: {}
# [doc = " Describe the relationship between the paths of two places"] # [doc = " eg:"] # [doc = " - `foo` is ancestor of `foo.bar.baz`"] # [doc = " - `foo.bar.baz` is an descendant of `foo.bar`"] # [doc = " - `foo.bar` and `foo.baz` are divergent"] enum PlaceAncestryRelation { Ancestor , Descendant , SamePlace , Divergent , }
};
}
