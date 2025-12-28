macro_rules! deps {
    () => {
        Writer!();
        Endianity!();
    };
}

macro_rules! EndianVec {
    () => {
        deps!();
        # [doc = " A `Vec<u8>` with endianity metadata."] # [doc = ""] # [doc = " This implements the `Writer` trait, which is used for all writing of DWARF sections."] # [derive (Debug , Clone)] pub struct EndianVec < Endian > where Endian : Endianity , { vec : Vec < u8 > , endian : Endian , }
    };
}

EndianVec!();