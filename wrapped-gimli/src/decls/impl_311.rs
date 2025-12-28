macro_rules! deps {
    () => {
        EndianReader!();
        Endianity!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        impl < Endian , T > Index < RangeFrom < usize > > for EndianReader < Endian , T > where Endian : Endianity , T : CloneStableDeref < Target = [u8] > + Debug , { type Output = [u8] ; fn index (& self , idx : RangeFrom < usize >) -> & Self :: Output { & self . bytes () [idx] } }
    };
}

impl_311!()