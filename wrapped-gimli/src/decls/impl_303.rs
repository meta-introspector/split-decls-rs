macro_rules! deps {
    () => {
        EndianReader!();
        Endianity!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl < Endian , T > Hash for EndianReader < Endian , T > where Endian : Endianity , T : CloneStableDeref < Target = [u8] > + Debug , { fn hash < H : Hasher > (& self , state : & mut H) { self . bytes () . hash (state) ; } }
    };
}

impl_303!()