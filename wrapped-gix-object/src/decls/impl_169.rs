macro_rules! deps {
    () => {
        Error!();
        Kind!();
        Write!();
        WriteTo!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < T > crate :: Write for Arc < T > where T : crate :: Write , { fn write (& self , object : & dyn WriteTo) -> Result < ObjectId , crate :: write :: Error > { self . deref () . write (object) } fn write_buf (& self , object : Kind , from : & [u8]) -> Result < ObjectId , crate :: write :: Error > { self . deref () . write_buf (object , from) } fn write_stream (& self , kind : Kind , size : u64 , from : & mut dyn Read) -> Result < ObjectId , crate :: write :: Error > { self . deref () . write_stream (kind , size , from) } }
    };
}

impl_169!();