// Generated macro for direct_super_traits_cb (function)
macro_rules! Depcrate_utilsdirect_super_traits_cb {
() => {
// Module: crate::utils
// Provides: {"direct_super_traits_cb"}
// Dependencies: {}
fn direct_super_traits_cb (db : & dyn DefDatabase , trait_ : TraitId , cb : impl FnMut (TraitId)) { let resolver = LazyCell :: new (| | trait_ . resolver (db)) ; let (generic_params , store) = db . generic_params_and_store (trait_ . into ()) ; let trait_self = generic_params . trait_self_param () ; generic_params . where_predicates () . iter () . filter_map (| pred | match pred { WherePredicate :: ForLifetime { target , bound , .. } | WherePredicate :: TypeBound { target , bound } => { let is_trait = match & store [* target] { TypeRef :: Path (p) => p . is_self_type () , TypeRef :: TypeParam (p) => Some (p . local_id ()) == trait_self , _ => false , } ; match is_trait { true => bound . as_path (& store) , false => None , } } WherePredicate :: Lifetime { .. } => None , }) . filter (| (_ , bound_modifier) | matches ! (bound_modifier , TraitBoundModifier :: None)) . filter_map (| (path , _) | match resolver . resolve_path_in_type_ns_fully (db , path) { Some (TypeNs :: TraitId (t)) => Some (t) , _ => None , }) . for_each (cb) ; }
};
}
