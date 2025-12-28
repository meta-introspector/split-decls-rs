macro_rules! deps {
    () => {
        HirDatabase!();
        InherentImpls!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        # [salsa :: tracked] impl InherentImpls { # [salsa :: tracked (returns (ref))] pub fn for_crate (db : & dyn HirDatabase , krate : Crate) -> Self { let _p = tracing :: info_span ! ("inherent_impls_in_crate_query" , ? krate) . entered () ; let crate_def_map = crate_def_map (db , krate) ; Self :: collect_def_map (db , crate_def_map) } # [salsa :: tracked (returns (ref))] pub fn for_block (db : & dyn HirDatabase , block : BlockId) -> Option < Box < Self > > { let _p = tracing :: info_span ! ("inherent_impls_in_block_query") . entered () ; let block_def_map = block_def_map (db , block) ; let result = Self :: collect_def_map (db , block_def_map) ; if result . map . is_empty () { None } else { Some (Box :: new (result)) } } }
    };
}

impl_395!();