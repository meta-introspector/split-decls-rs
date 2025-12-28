macro_rules! deps {
    () => {
        Proxy!();
        Error!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < T > gix_object :: Write for Proxy < T > where T : gix_object :: Write , { fn write_stream (& self , kind : gix_object :: Kind , size : u64 , from : & mut dyn std :: io :: Read ,) -> Result < gix_hash :: ObjectId , gix_object :: write :: Error > { let Some (map) = self . memory . as_ref () else { return self . inner . write_stream (kind , size , from) ; } ; let mut buf = Vec :: new () ; from . read_to_end (& mut buf) ? ; let id = gix_object :: compute_hash (self . object_hash , kind , & buf) ? ; map . borrow_mut () . insert (id , (kind , buf)) ; Ok (id) } }
    };
}

impl_158!();