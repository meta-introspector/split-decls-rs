macro_rules! deps {
    () => {
        EntryRef!();
        Entry!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl From < tree :: EntryRef < '_ > > for tree :: Entry { fn from (other : tree :: EntryRef < '_ >) -> tree :: Entry { let tree :: EntryRef { mode , filename , oid } = other ; tree :: Entry { mode , filename : filename . to_owned () , oid : oid . into () , } } }
    };
}

impl_56!()