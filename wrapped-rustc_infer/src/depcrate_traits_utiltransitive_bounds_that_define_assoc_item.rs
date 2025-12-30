// Generated macro for transitive_bounds_that_define_assoc_item (function)
macro_rules! Depcrate_traits_utiltransitive_bounds_that_define_assoc_item {
() => {
// Module: crate::traits::util
// Provides: {"transitive_bounds_that_define_assoc_item"}
// Dependencies: {}
# [doc = " A specialized variant of `elaborate` that only elaborates trait references that may"] # [doc = " define the given associated item with the name `assoc_name`. It uses the"] # [doc = " `explicit_supertraits_containing_assoc_item` query to avoid enumerating super-predicates that"] # [doc = " aren't related to `assoc_item`. This is used when resolving types like `Self::Item` or"] # [doc = " `T::Item` and helps to avoid cycle errors (see e.g. #35237)."] pub fn transitive_bounds_that_define_assoc_item < 'tcx > (tcx : TyCtxt < 'tcx > , trait_refs : impl Iterator < Item = ty :: PolyTraitRef < 'tcx > > , assoc_name : Ident ,) -> impl Iterator < Item = ty :: PolyTraitRef < 'tcx > > { let mut seen = FxHashSet :: default () ; let mut stack : Vec < _ > = trait_refs . collect () ; std :: iter :: from_fn (move | | { while let Some (trait_ref) = stack . pop () { if ! seen . insert (tcx . anonymize_bound_vars (trait_ref)) { continue ; } stack . extend (tcx . explicit_supertraits_containing_assoc_item ((trait_ref . def_id () , assoc_name)) . iter_identity_copied () . map (| (clause , _) | clause . instantiate_supertrait (tcx , trait_ref)) . filter_map (| clause | clause . as_trait_clause ()) . filter (| clause | clause . polarity () == ty :: PredicatePolarity :: Positive) . map (| clause | clause . map_bound (| clause | clause . trait_ref)) ,) ; return Some (trait_ref) ; } None }) }
};
}
