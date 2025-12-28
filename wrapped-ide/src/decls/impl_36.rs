macro_rules! deps {
    () => {
        ToNav!();
        UpmappingResult!();
        NavigationTarget!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl ToNav for hir :: Module { fn to_nav (& self , db : & RootDatabase) -> UpmappingResult < NavigationTarget > { let InFile { file_id , value } = self . definition_source (db) ; let name = self . name (db) . map (| it | it . symbol () . clone ()) . unwrap_or_else (| | sym :: underscore) ; let (syntax , focus) = match & value { ModuleSource :: SourceFile (node) => (node . syntax () , None) , ModuleSource :: Module (node) => (node . syntax () , node . name ()) , ModuleSource :: BlockExpr (node) => (node . syntax () , None) , } ; orig_range_with_focus (db , file_id , syntax , focus) . map (| (FileRange { file_id , range : full_range } , focus_range) | { NavigationTarget :: from_syntax (file_id , name . clone () , focus_range , full_range , SymbolKind :: Module ,) } ,) } }
    };
}

impl_36!()