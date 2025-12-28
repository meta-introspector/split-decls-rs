macro_rules! deps {
    () => {
        Static!();
        HasSource!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl HasSource for Static { type Ast = ast :: Static ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
    };
}

impl_57!()