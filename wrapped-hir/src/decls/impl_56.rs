macro_rules! deps {
    () => {
        Const!();
        HasSource!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl HasSource for Const { type Ast = ast :: Const ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
    };
}

impl_56!()