macro_rules! deps {
    () => {
        Struct!();
        HasSource!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl HasSource for Struct { type Ast = ast :: Struct ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
    };
}

impl_51!();