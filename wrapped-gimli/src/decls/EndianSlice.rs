macro_rules! deps {
    () => {
        Endianity!();
        Reader!();
    };
}

macro_rules! EndianSlice {
    () => {
        deps!();
        # [doc = " A `&[u8]` slice with endianity metadata."] # [doc = ""] # [doc = " This implements the `Reader` trait, which is used for all reading of DWARF sections."] # [derive (Default , Clone , Copy , PartialEq , Eq , Hash)] pub struct EndianSlice < 'input , Endian > where Endian : Endianity , { slice : & 'input [u8] , endian : Endian , }
    };
}

EndianSlice!();