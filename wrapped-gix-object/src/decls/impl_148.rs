macro_rules! deps {
    () => {
        WriteTo!();
        Kind!();
        Write!();
        Blob!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl crate :: WriteTo for Blob { # [doc = " Write the blobs data to `out` verbatim."] fn write_to (& self , out : & mut dyn io :: Write) -> io :: Result < () > { self . to_ref () . write_to (out) } fn kind (& self) -> Kind { Kind :: Blob } fn size (& self) -> u64 { self . to_ref () . size () } }
    };
}

impl_148!()