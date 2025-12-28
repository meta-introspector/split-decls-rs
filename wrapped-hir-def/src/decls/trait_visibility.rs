macro_rules! deps {
    () => {
        Visibility!();
        DefDatabase!();
    };
}

macro_rules! trait_visibility {
    () => {
        deps!();
        fn trait_visibility (db : & dyn DefDatabase , def : TraitId) -> Visibility { let loc = def . lookup (db) ; let source = loc . source (db) ; visibility_from_ast (db , def , source . map (| src | src . visibility ())) }
    };
}

trait_visibility!();