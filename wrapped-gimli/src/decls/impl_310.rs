macro_rules! deps {
    () => {
        EndianReader!();
        Endianity!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < Endian , T > Index < usize > for EndianReader < Endian , T > where Endian : Endianity , T : CloneStableDeref < Target = [u8] > + Debug , { type Output = u8 ; fn index (& self , idx : usize) -> & Self :: Output { & self . bytes () [idx] } }
    };
}

impl_310!()