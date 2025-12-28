macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl Entry { fn new (entry : & tree :: EntryRef < '_ > , filepath : BString) -> Self { Entry { filepath , oid : entry . oid . to_owned () , mode : entry . mode , } } }
    };
}

impl_42!();