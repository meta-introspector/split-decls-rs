macro_rules! deps {
    () => {
        Endianity!();
        EndianReader!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl < Endian , T > Deref for EndianReader < Endian , T > where Endian : Endianity , T : CloneStableDeref < Target = [u8] > + Debug , { type Target = [u8] ; fn deref (& self) -> & Self :: Target { self . bytes () } }
    };
}

impl_312!()