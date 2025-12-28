macro_rules! deps {
    () => {
        EndianSlice!();
        EhFrameHdr!();
        Endianity!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < 'input , Endian > EhFrameHdr < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Constructs a new `EhFrameHdr` instance from the data in the `.eh_frame_hdr` section."] pub fn new (section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (section , endian)) } }
    };
}

impl_165!();