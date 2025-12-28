macro_rules! deps {
    () => {
        Struct!();
        HasVisibility!();
        Adt!();
        Union!();
        Enum!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl HasVisibility for Adt { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { match self { Adt :: Struct (it) => it . visibility (db) , Adt :: Union (it) => it . visibility (db) , Adt :: Enum (it) => it . visibility (db) , } } }
    };
}

impl_60!()