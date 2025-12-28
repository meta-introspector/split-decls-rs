macro_rules! deps {
    () => {
        TraitEnvironment!();
    };
}

macro_rules! lookup_impl_assoc_item_for_trait_ref {
    () => {
        deps!();
        fn lookup_impl_assoc_item_for_trait_ref < 'db > (infcx : & InferCtxt < 'db > , trait_ref : TraitRef < 'db > , env : Arc < TraitEnvironment < 'db > > , name : & Name ,) -> Option < (AssocItemId , GenericArgs < 'db >) > { let (impl_id , impl_subst) = find_matching_impl (infcx , & env , trait_ref) ? ; let item = impl_id . impl_items (infcx . interner . db) . items . iter () . find_map (| (n , it) | match * it { AssocItemId :: FunctionId (f) => (n == name) . then_some (AssocItemId :: FunctionId (f)) , AssocItemId :: ConstId (c) => (n == name) . then_some (AssocItemId :: ConstId (c)) , AssocItemId :: TypeAliasId (_) => None , }) ? ; Some ((item , impl_subst)) }
    };
}

lookup_impl_assoc_item_for_trait_ref!()