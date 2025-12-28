macro_rules! deps {
    () => {
        Store!();
        Error!();
        Handle!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < S > gix_object :: Find for super :: Handle < S > where S : Deref < Target = super :: Store > + Clone , Self : gix_pack :: Find , { fn try_find < 'a > (& self , id : & gix_hash :: oid , buffer : & 'a mut Vec < u8 > ,) -> Result < Option < gix_object :: Data < 'a > > , gix_object :: find :: Error > { gix_pack :: Find :: try_find (self , id , buffer) . map (| t | t . map (| t | t . 0)) } }
    };
}

impl_6!();