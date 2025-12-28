macro_rules! deps {
    () => {
        PathResolution!();
    };
}

macro_rules! resolve_hir_path {
    () => {
        deps!();
        # [inline] pub (crate) fn resolve_hir_path (db : & dyn HirDatabase , resolver : & Resolver < '_ > , path : & Path , hygiene : HygieneId , store : Option < & ExpressionStore > ,) -> Option < PathResolution > { resolve_hir_path_ (db , resolver , path , false , hygiene , store , false) . any () }
    };
}

resolve_hir_path!()