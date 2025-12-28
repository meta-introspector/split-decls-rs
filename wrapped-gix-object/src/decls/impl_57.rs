macro_rules! deps {
    () => {
        Entry!();
        EntryRef!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < 'a > From < & 'a tree :: Entry > for tree :: EntryRef < 'a > { fn from (other : & 'a tree :: Entry) -> tree :: EntryRef < 'a > { let tree :: Entry { mode , filename , oid } = other ; tree :: EntryRef { mode : * mode , filename : filename . as_ref () , oid , } } }
    };
}

impl_57!();