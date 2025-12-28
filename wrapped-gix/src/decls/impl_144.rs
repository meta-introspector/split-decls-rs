macro_rules! deps {
    () => {
        Id!();
        Repository!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < 'repo > Id < 'repo > { pub (crate) fn from_id (id : impl Into < ObjectId > , repo : & 'repo crate :: Repository) -> Self { Id { inner : id . into () , repo } } # [doc = " Turn this instance into its bare [`ObjectId`]."] pub fn detach (self) -> ObjectId { self . inner } }
    };
}

impl_144!();