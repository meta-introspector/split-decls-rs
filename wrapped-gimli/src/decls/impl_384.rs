macro_rules! deps {
    () => {
        EndianSlice!();
        DebugTuIndex!();
        Endianity!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl < 'input , Endian > DebugTuIndex < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugTuIndex` instance from the data in the `.debug_tu_index`"] # [doc = " section."] pub fn new (section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (section , endian)) } }
    };
}

impl_384!()