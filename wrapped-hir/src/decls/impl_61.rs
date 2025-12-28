macro_rules! deps {
    () => {
        Impl!();
        HasSource!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl HasSource for Impl { type Ast = ast :: Impl ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
    };
}

impl_61!();