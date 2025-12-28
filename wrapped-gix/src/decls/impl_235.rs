macro_rules! deps {
    () => {
        EntryRef!();
        Id!();
        Error!();
        Entry!();
        Object!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl < 'repo , 'a > EntryRef < 'repo , 'a > { # [doc = " The kind of object to which [`id()`][Self::id()] is pointing."] pub fn mode (& self) -> gix_object :: tree :: EntryMode { self . inner . mode } # [doc = " The kind of object to which [`id()`][Self::id()] is pointing, as shortcut to [self.mode().kind()](Self::mode())."] pub fn kind (& self) -> gix_object :: tree :: EntryKind { self . inner . mode . kind () } # [doc = " The name of the file in the parent tree."] pub fn filename (& self) -> & gix_object :: bstr :: BStr { self . inner . filename } # [doc = " Return the entries id, connected to the underlying repository."] pub fn id (& self) -> crate :: Id < 'repo > { crate :: Id :: from_id (self . inner . oid , self . repo) } # [doc = " Return the plain object id of this entry, without access to the repository."] pub fn oid (& self) -> & gix_hash :: oid { self . inner . oid } # [doc = " Return the object this entry points to."] pub fn object (& self) -> Result < crate :: Object < 'repo > , crate :: object :: find :: existing :: Error > { self . id () . object () } # [doc = " Return the plain object id of this entry, without access to the repository."] pub fn object_id (& self) -> gix_hash :: ObjectId { self . inner . oid . to_owned () } # [doc = " Detach the repository from this instance."] pub fn detach (& self) -> gix_object :: tree :: EntryRef < 'a > { self . inner } # [doc = " Create an instance that doesn't bind to a buffer anymore (but that still contains a repository reference)."] pub fn to_owned (& self) -> Entry < 'repo > { Entry { inner : self . inner . into () , repo : self . repo , } } }
    };
}

impl_235!()