macro_rules! deps {
    () => {
        Handle!();
        Store!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < S > gix_object :: Exists for super :: Handle < S > where S : Deref < Target = super :: Store > + Clone , Self : gix_pack :: Find , { fn exists (& self , id : & gix_hash :: oid) -> bool { gix_pack :: Find :: contains (self , id) } }
    };
}

impl_8!();