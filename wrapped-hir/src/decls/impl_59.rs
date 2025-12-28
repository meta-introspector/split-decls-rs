macro_rules! deps {
    () => {
        HasSource!();
        TypeAlias!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl HasSource for TypeAlias { type Ast = ast :: TypeAlias ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
    };
}

impl_59!()