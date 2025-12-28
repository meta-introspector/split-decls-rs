macro_rules! deps {
    () => {
        TryToNav!();
        UpmappingResult!();
        NavigationTarget!();
        ToNavFromAst!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < D > TryToNav for D where D : HasSource + ToNavFromAst + Copy + HasDocs + for < 'db > HirDisplay < 'db > + HasCrate , D :: Ast : ast :: HasName , { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { let db = sema . db ; let src = self . source (db) ? ; Some (NavigationTarget :: from_named (db , src . as_ref () . map (| it | it as & dyn ast :: HasName) , D :: KIND ,) . map (| mut res | { res . docs = self . docs (db) ; res . description = hir :: attach_db (db , | | { Some (self . display (db , self . krate (db) . to_display_target (db)) . to_string ()) }) ; res . container_name = self . container_name (db) ; res }) ,) } }
    };
}

impl_35!()