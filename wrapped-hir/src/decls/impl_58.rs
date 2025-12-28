macro_rules! deps {
    () => {
        Trait!();
        HasSource!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl HasSource for Trait { type Ast = ast :: Trait ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
    };
}

impl_58!()