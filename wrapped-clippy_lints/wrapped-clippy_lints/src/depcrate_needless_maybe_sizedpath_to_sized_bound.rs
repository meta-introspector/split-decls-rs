// Generated macro for path_to_sized_bound (function)
macro_rules! Depcrate_needless_maybe_sizedpath_to_sized_bound {
() => {
// Module: crate::needless_maybe_sized
// Provides: {"path_to_sized_bound"}
// Dependencies: {}
# [doc = " Searches the supertraits of the trait referred to by `trait_bound` recursively, returning the"] # [doc = " path taken to find a `Sized` bound if one is found"] fn path_to_sized_bound (cx : & LateContext < '_ > , trait_bound : & PolyTraitRef < '_ >) -> Option < Vec < DefId > > { fn search (cx : & LateContext < '_ > , path : & mut Vec < DefId >) -> bool { let trait_def_id = * path . last () . unwrap () ; if Some (trait_def_id) == cx . tcx . lang_items () . sized_trait () { return true ; } for (predicate , _) in cx . tcx . explicit_super_predicates_of (trait_def_id) . iter_identity_copied () { if let ClauseKind :: Trait (trait_predicate) = predicate . kind () . skip_binder () && trait_predicate . polarity == PredicatePolarity :: Positive && ! path . contains (& trait_predicate . def_id ()) { path . push (trait_predicate . def_id ()) ; if search (cx , path) { return true ; } path . pop () ; } } false } let mut path = vec ! [trait_bound . trait_ref . trait_def_id () ?] ; search (cx , & mut path) . then_some (path) }
};
}
