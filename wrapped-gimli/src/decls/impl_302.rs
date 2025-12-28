macro_rules! deps {
    () => {
        EndianReader!();
        Endianity!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < Endian , T > Eq for EndianReader < Endian , T > where Endian : Endianity , T : CloneStableDeref < Target = [u8] > + Debug , { }
    };
}

impl_302!();