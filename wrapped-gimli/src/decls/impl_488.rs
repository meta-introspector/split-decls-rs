macro_rules! deps {
    () => {
        DebugMacro!();
        EndianSlice!();
        LittleEndian!();
        Endianity!();
    };
}

macro_rules! impl_488 {
    () => {
        deps!();
        impl < 'input , Endian > DebugMacro < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugMacro` instance from the data in the `.debug_macro`"] # [doc = " section."] # [doc = ""] # [doc = " It is the caller's responsibility to read the `.debug_macro` section and"] # [doc = " present it as a `&[u8]` slice. That means using some ELF loader on"] # [doc = " Linux, a Mach-O loader on macOS, etc."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugMacro, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [1, 0, 95, 95, 83, 84, 68, 67, 95, 95, 32, 49, 0];"] # [doc = " # let read_section_somehow = || &buf;"] # [doc = " let debug_str = DebugMacro::new(read_section_somehow(), LittleEndian);"] # [doc = " ```"] pub fn new (macro_section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (macro_section , endian)) } }
    };
}

impl_488!()