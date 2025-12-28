macro_rules! deps {
    () => {
        Endianity!();
        EndianSlice!();
        DebugCuIndex!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        impl < 'input , Endian > DebugCuIndex < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugCuIndex` instance from the data in the `.debug_cu_index`"] # [doc = " section."] pub fn new (section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (section , endian)) } }
    };
}

impl_378!()