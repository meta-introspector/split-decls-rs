// Generated macro for lookup_impl_assoc_item_for_trait_ref (function)
macro_rules! Depcrate_consteval_tests_method_resolutionlookup_impl_assoc_item_for_trait_ref {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"lookup_impl_assoc_item_for_trait_ref"}
// Dependencies: {}
fn lookup_impl_assoc_item_for_trait_ref < 'db > (infcx : & InferCtxt < 'db > , trait_ref : TraitRef < 'db > , env : Arc < TraitEnvironment < 'db > > , name : & Name ,) -> Option < (AssocItemId , GenericArgs < 'db >) > { let (impl_id , impl_subst) = find_matching_impl (infcx , & env , trait_ref) ? ; let item = impl_id . impl_items (infcx . interner . db) . items . iter () . find_map (| (n , it) | match * it { AssocItemId :: FunctionId (f) => (n == name) . then_some (AssocItemId :: FunctionId (f)) , AssocItemId :: ConstId (c) => (n == name) . then_some (AssocItemId :: ConstId (c)) , AssocItemId :: TypeAliasId (_) => None , }) ? ; Some ((item , impl_subst)) }
};
}
