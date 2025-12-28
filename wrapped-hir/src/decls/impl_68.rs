macro_rules! deps {
    () => {
        ExternCrateDecl!();
        HasSource!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl HasSource for ExternCrateDecl { type Ast = ast :: ExternCrate ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
    };
}

impl_68!();