macro_rules! deps {
    () => {
        TryToNav!();
        UpmappingResult!();
        NavigationTarget!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl TryToNav for hir :: LifetimeParam { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { let db = sema . db ; let InFile { file_id , value } = self . source (db) ? ; let name = self . name (db) . symbol () . clone () ; Some (orig_range (db , file_id , value . syntax ()) . map (| (FileRange { file_id , range : full_range } , focus_range) | NavigationTarget { file_id , name : name . clone () , alias : None , kind : Some (SymbolKind :: LifetimeParam) , full_range , focus_range , container_name : None , description : None , docs : None , } ,)) } }
    };
}

impl_50!();