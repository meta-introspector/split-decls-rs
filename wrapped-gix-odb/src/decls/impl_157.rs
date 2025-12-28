macro_rules! deps {
    () => {
        Error!();
        Proxy!();
        Header!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < T > gix_object :: FindHeader for Proxy < T > where T : gix_object :: FindHeader , { fn try_header (& self , id : & gix_hash :: oid) -> Result < Option < gix_object :: Header > , gix_object :: find :: Error > { if let Some (map) = self . memory . as_ref () { let map = map . borrow () ; if let Some ((kind , data)) = map . get (id) { return Ok (Some (gix_object :: Header { kind : * kind , size : data . len () as u64 , })) ; } } self . inner . try_header (id) } }
    };
}

impl_157!()