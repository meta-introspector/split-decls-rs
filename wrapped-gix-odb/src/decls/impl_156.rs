macro_rules! deps {
    () => {
        Header!();
        Proxy!();
        Error!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < T > crate :: Header for Proxy < T > where T : crate :: Header , { fn try_header (& self , id : & gix_hash :: oid) -> Result < Option < Header > , gix_object :: find :: Error > { if let Some (map) = self . memory . as_ref () { let map = map . borrow () ; if let Some ((kind , data)) = map . get (id) { return Ok (Some (Header :: Loose { kind : * kind , size : data . len () as u64 , })) ; } } self . inner . try_header (id) } }
    };
}

impl_156!()