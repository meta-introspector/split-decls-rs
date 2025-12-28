macro_rules! deps {
    () => {
        Read!();
        Error!();
        Kind!();
        Repository!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl gix_object :: Write for crate :: Repository { fn write (& self , object : & dyn gix_object :: WriteTo) -> Result < gix_hash :: ObjectId , gix_object :: write :: Error > { let mut buf = self . empty_reusable_buffer () ; object . write_to (buf . deref_mut ()) ? ; self . write_buf (object . kind () , & buf) } fn write_buf (& self , object : gix_object :: Kind , from : & [u8]) -> Result < gix_hash :: ObjectId , gix_object :: write :: Error > { let oid = gix_object :: compute_hash (self . object_hash () , object , from) ? ; if self . objects . exists (& oid) { return Ok (oid) ; } self . objects . write_buf (object , from) } fn write_stream (& self , kind : gix_object :: Kind , size : u64 , from : & mut dyn std :: io :: Read ,) -> Result < gix_hash :: ObjectId , gix_object :: write :: Error > { let mut buf = self . empty_reusable_buffer () ; let bytes = std :: io :: copy (from , buf . deref_mut ()) ? ; if size != bytes { return Err (format ! ("Found {bytes} bytes in stream, but had {size} bytes declared") . into ()) ; } self . write_buf (kind , & buf) } }
    };
}

impl_335!();