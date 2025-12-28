macro_rules! deps {
    () => {
        Kind!();
        Blob!();
        Note!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        # [doc = " Note that the `data` written here might not correspond to the `id` of the `Blob` anymore if it was modified."] # [doc = " Also, this is merely for convenience when writing empty blobs to the ODB. For writing any blob, use"] # [doc = " [`Repository::write_blob()`](crate::Repository::write_blob())."] impl gix_object :: WriteTo for Blob < '_ > { fn write_to (& self , out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { out . write_all (& self . data) } fn kind (& self) -> gix_object :: Kind { gix_object :: Kind :: Blob } fn size (& self) -> u64 { self . data . len () as u64 } }
    };
}

impl_175!()