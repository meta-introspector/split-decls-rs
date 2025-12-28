macro_rules! deps {
    () => {
        Proxy!();
        Error!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < T > gix_object :: Find for Proxy < T > where T : gix_object :: Find , { fn try_find < 'a > (& self , id : & gix_hash :: oid , buffer : & 'a mut Vec < u8 > ,) -> Result < Option < Data < 'a > > , gix_object :: find :: Error > { if let Some (map) = self . memory . as_ref () { let map = map . borrow () ; if let Some ((kind , data)) = map . get (id) { buffer . clear () ; buffer . extend_from_slice (data) ; return Ok (Some (Data { kind : * kind , data : & * buffer , })) ; } } self . inner . try_find (id , buffer) } }
    };
}

impl_154!();