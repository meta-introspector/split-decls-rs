macro_rules! deps {
    () => {
        TraitImpls!();
        HirDatabase!();
    };
}

macro_rules! impl_401 {
    () => {
        deps!();
        # [salsa :: tracked] impl TraitImpls { # [salsa :: tracked (returns (ref))] pub fn for_crate (db : & dyn HirDatabase , krate : Crate) -> Arc < Self > { let _p = tracing :: info_span ! ("inherent_impls_in_crate_query" , ? krate) . entered () ; let crate_def_map = crate_def_map (db , krate) ; let result = Self :: collect_def_map (db , crate_def_map) ; Arc :: new (result) } # [salsa :: tracked (returns (ref))] pub fn for_block (db : & dyn HirDatabase , block : BlockId) -> Option < Box < Self > > { let _p = tracing :: info_span ! ("inherent_impls_in_block_query") . entered () ; let block_def_map = block_def_map (db , block) ; let result = Self :: collect_def_map (db , block_def_map) ; if result . map . is_empty () { None } else { Some (Box :: new (result)) } } # [salsa :: tracked (returns (ref))] pub fn for_crate_and_deps (db : & dyn HirDatabase , krate : Crate) -> Box < [Arc < Self >] > { krate . transitive_deps (db) . iter () . map (| & dep | Self :: for_crate (db , dep) . clone ()) . collect () } }
    };
}

impl_401!()