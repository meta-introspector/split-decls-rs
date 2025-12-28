macro_rules! deps {
    () => {
        UpmappingResult!();
        TryToNav!();
        NavigationTarget!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl TryToNav for hir :: Field { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { let db = sema . db ; let src = self . source (db) ? ; let krate = self . parent_def (db) . module (db) . krate () ; let field_source = match & src . value { FieldSource :: Named (it) => { NavigationTarget :: from_named (db , src . with_value (it) , SymbolKind :: Field) . map (| mut res | { res . docs = self . docs (db) ; res . description = hir :: attach_db (db , | | { Some (self . display (db , krate . to_display_target (db)) . to_string ()) }) ; res } ,) } FieldSource :: Pos (it) => orig_range (db , src . file_id , it . syntax ()) . map (| (FileRange { file_id , range : full_range } , focus_range) | { NavigationTarget :: from_syntax (file_id , Symbol :: integer (self . index ()) , focus_range , full_range , SymbolKind :: Field ,) } ,) , } ; Some (field_source) } }
    };
}

impl_40!()