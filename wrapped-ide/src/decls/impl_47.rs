macro_rules! deps {
    () => {
        NavigationTarget!();
        TryToNav!();
        UpmappingResult!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl TryToNav for hir :: Label { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { let db = sema . db ; let InFile { file_id , value } = self . source (db) ? ; let name = self . name (db) . symbol () . clone () ; Some (orig_range_with_focus (db , file_id , value . syntax () , value . lifetime ()) . map (| (FileRange { file_id , range : full_range } , focus_range) | NavigationTarget { file_id , name : name . clone () , alias : None , kind : Some (SymbolKind :: Label) , full_range , focus_range , container_name : None , description : None , docs : None , } ,)) } }
    };
}

impl_47!()