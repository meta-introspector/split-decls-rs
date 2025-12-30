// Generated macro for UseTracker (struct)
macro_rules! Depcrate_use_trackingUseTracker {
() => {
// Module: crate::use_tracking
// Provides: {"UseTracker"}
// Dependencies: {}
# [doc = " `UseTracker` tracks what type variables that have used in `any_with::<Type>`"] # [doc = " or similar and thus needs an `Arbitrary` bound added to them."] pub struct UseTracker { # [doc = " Tracks 'usage' of a type variable name."] # [doc = " Allocation of this \"map\" will happen at once and no further"] # [doc = " allocation will happen after that. Only potential updates"] # [doc = " will happen after initial allocation."] # [doc = " We need to preserve insertion order, thus using Vec instead of BTreeMap"] # [doc = " or HashMap. A potential alternative would be indexmap crate, but our"] # [doc = " maps are so small that it would not bring any significant benefit."] used_map : Vec < (syn :: Ident , bool) > , # [doc = " Extra types to bound by `Arbitrary` in the `where` clause."] where_types : HashSet < syn :: Type > , # [doc = " The generics that we are doing this for."] # [doc = " This what we will modify later once we're done."] generics : syn :: Generics , # [doc = " If set to `true`, then `mark_used` has no effect."] track : bool , }
};
}
