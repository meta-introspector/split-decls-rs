macro_rules! deps {
    () => {
        TryToNav!();
        NavigationTarget!();
        UpmappingResult!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl TryToNav for hir :: ConstParam { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { let db = sema . db ; let InFile { file_id , value } = self . merge () . source (db) ? ; let name = self . name (db) . symbol () . clone () ; let value = match value { Either :: Left (ast :: TypeOrConstParam :: Const (x)) => x , _ => { never ! () ; return None ; } } ; Some (orig_range_with_focus (db , file_id , value . syntax () , value . name ()) . map (| (FileRange { file_id , range : full_range } , focus_range) | NavigationTarget { file_id , name : name . clone () , alias : None , kind : Some (SymbolKind :: ConstParam) , full_range , focus_range , container_name : None , description : None , docs : None , } ,)) } }
    };
}

impl_51!()