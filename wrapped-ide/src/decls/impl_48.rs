macro_rules! deps {
    () => {
        NavigationTarget!();
        TryToNav!();
        UpmappingResult!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl TryToNav for hir :: TypeParam { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { let db = sema . db ; let InFile { file_id , value } = self . merge () . source (db) ? ; let name = self . name (db) . symbol () . clone () ; let value = match value { Either :: Left (ast :: TypeOrConstParam :: Type (x)) => Either :: Left (x) , Either :: Left (ast :: TypeOrConstParam :: Const (_)) => { never ! () ; return None ; } Either :: Right (x) => Either :: Right (x) , } ; let syntax = match & value { Either :: Left (type_param) => type_param . syntax () , Either :: Right (trait_) => trait_ . syntax () , } ; let focus = value . as_ref () . either (| it | it . name () , | it | it . name ()) ; Some (orig_range_with_focus (db , file_id , syntax , focus) . map (| (FileRange { file_id , range : full_range } , focus_range) | NavigationTarget { file_id , name : name . clone () , alias : None , kind : Some (SymbolKind :: TypeParam) , full_range , focus_range , container_name : None , description : None , docs : None , } ,)) } }
    };
}

impl_48!()