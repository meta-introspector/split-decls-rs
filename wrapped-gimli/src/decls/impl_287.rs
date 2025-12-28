macro_rules! deps {
    () => {
        Endianity!();
        EndianSlice!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl < 'input , Endian > Deref for EndianSlice < 'input , Endian > where Endian : Endianity , { type Target = [u8] ; fn deref (& self) -> & Self :: Target { self . slice } }
    };
}

impl_287!()