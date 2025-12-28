macro_rules! trait_self_param_idx {
    () => {
        pub (crate) fn trait_self_param_idx (db : & dyn DefDatabase , def : GenericDefId) -> Option < usize > { match def { GenericDefId :: TraitId (_) => { let params = db . generic_params (def) ; params . trait_self_param () . map (| idx | idx . into_raw () . into_u32 () as usize) } GenericDefId :: ImplId (_) => None , _ => { let parent_def = parent_generic_def (db , def) ? ; let parent_params = db . generic_params (parent_def) ; let parent_self_idx = parent_params . trait_self_param () ? . into_raw () . into_u32 () as usize ; Some (parent_self_idx) } } }
    };
}

trait_self_param_idx!();