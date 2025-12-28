macro_rules! deps {
    () => {
        Intern!();
        Lookup!();
    };
}

macro_rules! impl_intern_lookup {
    () => {
        deps!();
        # [macro_export] macro_rules ! impl_intern_lookup { ($ db : ident , $ id : ident , $ loc : ident , $ intern : ident , $ lookup : ident) => { impl $ crate :: Intern for $ loc { type Database = dyn $ db ; type ID = $ id ; fn intern (self , db : & Self :: Database) -> Self :: ID { db .$ intern (self) } } impl $ crate :: Lookup for $ id { type Database = dyn $ db ; type Data = $ loc ; fn lookup (& self , db : & Self :: Database) -> Self :: Data { db .$ lookup (* self) } } } ; }
    };
}

impl_intern_lookup!()