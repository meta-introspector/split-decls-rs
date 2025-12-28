macro_rules! deps {
    () => {
        Binder!();
        HirDatabase!();
    };
}

macro_rules! associated_type_by_name_including_super_traits {
    () => {
        deps!();
        pub (crate) fn associated_type_by_name_including_super_traits < 'db > (db : & 'db dyn HirDatabase , trait_ref : TraitRef < 'db > , name : & Name ,) -> Option < (TraitRef < 'db > , TypeAliasId) > { let interner = DbInterner :: new_with (db , None , None) ; rustc_type_ir :: elaborate :: supertraits (interner , Binder :: dummy (trait_ref)) . find_map (| t | { let trait_id = t . as_ref () . skip_binder () . def_id . 0 ; let assoc_type = trait_id . trait_items (db) . associated_type_by_name (name) ? ; Some ((t . skip_binder () , assoc_type)) }) }
    };
}

associated_type_by_name_including_super_traits!()