macro_rules! deps {
    () => {
        Error!();
        WriteTo!();
        Kind!();
    };
}

macro_rules! Write {
    () => {
        deps!();
        # [doc = " Describe the capability to write git objects into an object store."] pub trait Write { # [doc = " Write objects using the intrinsic kind of [`hash`](gix_hash::Kind) into the database,"] # [doc = " returning id to reference it in subsequent reads."] fn write (& self , object : & dyn WriteTo) -> Result < gix_hash :: ObjectId , crate :: write :: Error > { let mut buf = Vec :: with_capacity (2048) ; object . write_to (& mut buf) ? ; self . write_stream (object . kind () , buf . len () as u64 , & mut buf . as_slice ()) } # [doc = " As [`write`](Write::write), but takes an [`object` kind](Kind) along with its encoded bytes."] fn write_buf (& self , object : crate :: Kind , mut from : & [u8]) -> Result < gix_hash :: ObjectId , crate :: write :: Error > { self . write_stream (object , from . len () as u64 , & mut from) } # [doc = " As [`write`](Write::write), but takes an input stream."] # [doc = " This is commonly used for writing blobs directly without reading them to memory first."] fn write_stream (& self , kind : crate :: Kind , size : u64 , from : & mut dyn io :: Read ,) -> Result < gix_hash :: ObjectId , crate :: write :: Error > ; }
    };
}

Write!();