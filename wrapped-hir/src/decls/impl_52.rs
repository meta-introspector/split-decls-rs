macro_rules! deps {
    () => {
        HasSource!();
        Union!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl HasSource for Union { type Ast = ast :: Union ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
    };
}

impl_52!();