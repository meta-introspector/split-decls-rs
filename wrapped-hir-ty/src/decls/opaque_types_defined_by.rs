macro_rules! deps {
    () => {
        ImplTraits!();
        HirDatabase!();
        ImplTraitId!();
        EarlyBinder!();
        InternedOpaqueTyId!();
        ImplTraitIdx!();
    };
}

macro_rules! opaque_types_defined_by {
    () => {
        deps!();
        pub (crate) fn opaque_types_defined_by (db : & dyn HirDatabase , def_id : DefWithBodyId , result : & mut Vec < SolverDefId > ,) { if let DefWithBodyId :: FunctionId (func) = def_id { extend_with_opaques (db , ImplTraits :: return_type_impl_traits (db , func) , | opaque_idx | ImplTraitId :: ReturnTypeImplTrait (func , opaque_idx) , result ,) ; } let extend_with_taits = | type_alias | { extend_with_opaques (db , ImplTraits :: type_alias_impl_traits (db , type_alias) , | opaque_idx | ImplTraitId :: TypeAliasImplTrait (type_alias , opaque_idx) , result ,) ; } ; let extend_with_atpit_from_assoc_items = | assoc_items : & [(Name , AssocItemId)] | { assoc_items . iter () . filter_map (| & (_ , assoc_id) | match assoc_id { AssocItemId :: TypeAliasId (it) => Some (it) , AssocItemId :: FunctionId (_) | AssocItemId :: ConstId (_) => None , }) . for_each (extend_with_taits) ; } ; let extend_with_atpit_from_container = | container | match container { ItemContainerId :: ImplId (impl_id) => { if db . impl_signature (impl_id) . target_trait . is_some () { extend_with_atpit_from_assoc_items (& impl_id . impl_items (db) . items) ; } } ItemContainerId :: TraitId (trait_id) => { extend_with_atpit_from_assoc_items (& trait_id . trait_items (db) . items) ; } _ => { } } ; match def_id { DefWithBodyId :: ConstId (id) => extend_with_atpit_from_container (id . loc (db) . container) , DefWithBodyId :: FunctionId (id) => extend_with_atpit_from_container (id . loc (db) . container) , DefWithBodyId :: StaticId (_) | DefWithBodyId :: VariantId (_) => { } } fn extend_with_opaques < 'db > (db : & 'db dyn HirDatabase , opaques : & Option < Box < EarlyBinder < 'db , ImplTraits < 'db > > > > , mut make_impl_trait : impl FnMut (ImplTraitIdx < 'db >) -> ImplTraitId < 'db > , result : & mut Vec < SolverDefId > ,) { if let Some (opaques) = opaques { for (opaque_idx , _) in (* * opaques) . as_ref () . skip_binder () . impl_traits . iter () { let opaque_id = InternedOpaqueTyId :: new (db , make_impl_trait (opaque_idx)) ; result . push (opaque_id . into ()) ; } } } }
    };
}

opaque_types_defined_by!();