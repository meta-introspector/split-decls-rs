macro_rules! deps {
    () => {
        Enum!();
        HasSource!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl HasSource for Enum { type Ast = ast :: Enum ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
    };
}

impl_53!();