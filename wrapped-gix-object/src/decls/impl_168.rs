macro_rules! deps {
    () => {
        WriteTo!();
        Error!();
        Kind!();
        Write!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < T > crate :: Write for & T where T : crate :: Write , { fn write (& self , object : & dyn WriteTo) -> Result < ObjectId , crate :: write :: Error > { (* self) . write (object) } fn write_buf (& self , object : Kind , from : & [u8]) -> Result < ObjectId , crate :: write :: Error > { (* self) . write_buf (object , from) } fn write_stream (& self , kind : Kind , size : u64 , from : & mut dyn Read) -> Result < ObjectId , crate :: write :: Error > { (* self) . write_stream (kind , size , from) } }
    };
}

impl_168!()