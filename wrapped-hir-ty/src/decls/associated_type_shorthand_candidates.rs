macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! associated_type_shorthand_candidates {
    () => {
        deps!();
        pub fn associated_type_shorthand_candidates (db : & dyn HirDatabase , def : GenericDefId , res : TypeNs , mut cb : impl FnMut (& Name , TypeAliasId) -> bool ,) -> Option < TypeAliasId > { let interner = DbInterner :: new_with (db , None , None) ; named_associated_type_shorthand_candidates (interner , def , res , None , | name , _ , id | { cb (name , id) . then_some (id) }) }
    };
}

associated_type_shorthand_candidates!();