macro_rules! deps {
    () => {
        UpmappingResult!();
        NavigationTarget!();
        TryToNav!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl TryToNav for hir :: BuiltinType { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { let db = sema . db ; let krate = db . all_crates () . iter () . copied () . find (| & krate | matches ! (krate . data (db) . origin , CrateOrigin :: Lang (LangCrateOrigin :: Std))) . map (Crate :: from) ? ; let edition = krate . edition (db) ; let fd = FamousDefs (sema , krate) ; let primitive_mod = format ! ("prim_{}" , self . name () . display (fd . 0 . db , edition)) ; let doc_owner = find_std_module (& fd , & primitive_mod , edition) ? ; Some (doc_owner . to_nav (db)) } }
    };
}

impl_53!();