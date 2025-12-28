macro_rules! deps {
    () => {
        Repository!();
        AncestorsIter!();
        ObjectIdExt!();
        Id!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl ObjectIdExt for ObjectId { fn ancestors < Find > (self , find : Find) -> AncestorsIter < Find > where Find : gix_object :: Find , { gix_traverse :: commit :: Simple :: new (Some (self) , find) } fn attach (self , repo : & crate :: Repository) -> crate :: Id < '_ > { crate :: Id :: from_id (self , repo) } }
    };
}

impl_17!()