macro_rules! deps {
    () => {
        Visitor!();
        TraitRef!();
    };
}

macro_rules! walk_trait_ref {
    () => {
        deps!();
        pub fn walk_trait_ref < 'v , V : Visitor < 'v > > (visitor : & mut V , trait_ref : & 'v TraitRef < 'v > ,) -> V :: Result { let TraitRef { hir_ref_id , path } = trait_ref ; try_visit ! (visitor . visit_id (* hir_ref_id)) ; visitor . visit_path (* path , * hir_ref_id) }
    };
}

walk_trait_ref!();