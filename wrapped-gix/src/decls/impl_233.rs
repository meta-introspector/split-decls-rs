macro_rules! deps {
    () => {
        Object!();
        Entry!();
        Error!();
        Id!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        # [doc = " Access"] impl < 'repo > Entry < 'repo > { # [doc = " The kind of object to which `oid` is pointing to."] pub fn mode (& self) -> gix_object :: tree :: EntryMode { self . inner . mode } # [doc = " The name of the file in the parent tree."] pub fn filename (& self) -> & BStr { self . inner . filename . as_ref () } # [doc = " Return the object id of the entry."] pub fn id (& self) -> crate :: Id < 'repo > { self . inner . oid . attach (self . repo) } # [doc = " Return the object this entry points to."] pub fn object (& self) -> Result < crate :: Object < 'repo > , crate :: object :: find :: existing :: Error > { self . id () . object () } # [doc = " Return the plain object id of this entry, without access to the repository."] pub fn oid (& self) -> & gix_hash :: oid { & self . inner . oid } # [doc = " Return the plain object id of this entry, without access to the repository."] pub fn object_id (& self) -> gix_hash :: ObjectId { self . inner . oid } }
    };
}

impl_233!();