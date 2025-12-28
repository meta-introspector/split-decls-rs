macro_rules! deps {
    () => {
        Function!();
        HasSource!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl HasSource for Function { type Ast = ast :: Fn ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
    };
}

impl_55!()