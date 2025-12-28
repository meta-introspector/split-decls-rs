macro_rules! Lookup {
    () => {
        pub trait Lookup { type Database : ? Sized ; type Data ; fn lookup (& self , db : & Self :: Database) -> Self :: Data ; }
    };
}

Lookup!()