macro_rules! deps {
    () => {
        ToNav!();
        UpmappingResult!();
        NavigationTarget!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl ToNav for LocalSource { fn to_nav (& self , db : & RootDatabase) -> UpmappingResult < NavigationTarget > { let InFile { file_id , value } = & self . source ; let file_id = * file_id ; let local = self . local ; let (node , name) = match & value { Either :: Left (bind_pat) => (bind_pat . syntax () , bind_pat . name ()) , Either :: Right (it) => (it . syntax () , it . name ()) , } ; orig_range_with_focus (db , file_id , node , name) . map (| (FileRange { file_id , range : full_range } , focus_range) | { let name = local . name (db) . symbol () . clone () ; let kind = if local . is_self (db) { SymbolKind :: SelfParam } else if local . is_param (db) { SymbolKind :: ValueParam } else { SymbolKind :: Local } ; NavigationTarget { file_id , name , alias : None , kind : Some (kind) , full_range , focus_range , container_name : None , description : None , docs : None , } } ,) } }
    };
}

impl_45!();