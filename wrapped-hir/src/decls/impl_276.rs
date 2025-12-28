macro_rules! deps {
    () => {
        HasVisibility!();
        Struct!();
        Enum!();
        Adt!();
        Union!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl HasVisibility for Adt { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { match self { Adt :: Struct (it) => it . visibility (db) , Adt :: Union (it) => it . visibility (db) , Adt :: Enum (it) => it . visibility (db) , } } }
    };
}

impl_276!()