macro_rules! deps {
    () => {
        Repository!();
        TreeEntryRefExt!();
        EntryRef!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < 'a > TreeEntryRefExt < 'a > for gix_object :: tree :: EntryRef < 'a > { fn attach < 'repo > (self , repo : & 'repo crate :: Repository) -> crate :: object :: tree :: EntryRef < 'repo , 'a > { crate :: object :: tree :: EntryRef { inner : self , repo } } }
    };
}

impl_30!();