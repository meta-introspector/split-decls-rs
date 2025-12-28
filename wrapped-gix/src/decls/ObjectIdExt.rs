macro_rules! deps {
    () => {
        AncestorsIter!();
        Repository!();
        Id!();
        Sealed!();
    };
}

macro_rules! ObjectIdExt {
    () => {
        deps!();
        # [doc = " An extension trait to add functionality to [`ObjectId`]s."] pub trait ObjectIdExt : Sealed { # [doc = " Create an iterator over the ancestry of the commits reachable from this id, which must be a commit."] fn ancestors < Find > (self , find : Find) -> AncestorsIter < Find > where Find : gix_object :: Find ; # [doc = " Infuse this object id `repo` access."] fn attach (self , repo : & crate :: Repository) -> crate :: Id < '_ > ; }
    };
}

ObjectIdExt!();