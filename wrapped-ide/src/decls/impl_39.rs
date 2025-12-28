macro_rules! deps {
    () => {
        NavigationTarget!();
        UpmappingResult!();
        TryToNav!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl TryToNav for hir :: ExternCrateDecl { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { let db = sema . db ; let src = self . source (db) ? ; let InFile { file_id , value } = src ; let focus = value . rename () . map_or_else (| | value . name_ref () . map (Either :: Left) , | it | it . name () . map (Either :: Right)) ; let krate = self . module (db) . krate () ; Some (orig_range_with_focus (db , file_id , value . syntax () , focus) . map (| (FileRange { file_id , range : full_range } , focus_range) | { let mut res = NavigationTarget :: from_syntax (file_id , self . alias_or_name (db) . unwrap_or_else (| | self . name (db)) . symbol () . clone () , focus_range , full_range , SymbolKind :: Module ,) ; res . docs = self . docs (db) ; res . description = Some (self . display (db , krate . to_display_target (db)) . to_string ()) ; res . container_name = container_name (db , * self) ; res } ,)) } }
    };
}

impl_39!()