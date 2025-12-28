macro_rules! deps {
    () => {
        Endianity!();
        EndianReader!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl < Endian , T1 , T2 > PartialEq < EndianReader < Endian , T2 > > for EndianReader < Endian , T1 > where Endian : Endianity , T1 : CloneStableDeref < Target = [u8] > + Debug , T2 : CloneStableDeref < Target = [u8] > + Debug , { fn eq (& self , rhs : & EndianReader < Endian , T2 >) -> bool { self . bytes () == rhs . bytes () } }
    };
}

impl_301!()