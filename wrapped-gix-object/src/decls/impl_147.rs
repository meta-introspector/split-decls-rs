macro_rules! deps {
    () => {
        BlobRef!();
        Kind!();
        Write!();
        Blob!();
        WriteTo!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl crate :: WriteTo for BlobRef < '_ > { # [doc = " Write the blobs data to `out` verbatim."] fn write_to (& self , out : & mut dyn io :: Write) -> io :: Result < () > { out . write_all (self . data) } fn kind (& self) -> Kind { Kind :: Blob } fn size (& self) -> u64 { self . data . len () as u64 } }
    };
}

impl_147!();