// Generated macro for Overlap (enum)
macro_rules! DepcrateOverlap {
() => {
// Module: crate
// Provides: {"Overlap"}
// Dependencies: {}
# [doc = " The degree of overlap between 2 places for borrow-checking."] enum Overlap { # [doc = " The places might partially overlap - in this case, we give"] # [doc = " up and say that they might conflict. This occurs when"] # [doc = " different fields of a union are borrowed. For example,"] # [doc = " if `u` is a union, we have no way of telling how disjoint"] # [doc = " `u.a.x` and `a.b.y` are."] Arbitrary , # [doc = " The places have the same type, and are either completely disjoint"] # [doc = " or equal - i.e., they can't \"partially\" overlap as can occur with"] # [doc = " unions. This is the \"base case\" on which we recur for extensions"] # [doc = " of the place."] EqualOrDisjoint , # [doc = " The places are disjoint, so we know all extensions of them"] # [doc = " will also be disjoint."] Disjoint , }
};
}
