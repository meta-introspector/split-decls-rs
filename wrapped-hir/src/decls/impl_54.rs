macro_rules! deps {
    () => {
        Variant!();
        HasSource!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl HasSource for Variant { type Ast = ast :: Variant ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < ast :: Variant > > { Some (self . id . lookup (db) . source (db)) } }
    };
}

impl_54!();