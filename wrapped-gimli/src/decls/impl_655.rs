macro_rules! deps {
    () => {
        Endianity!();
        LittleEndian!();
        DebugTypes!();
        EndianSlice!();
    };
}

macro_rules! impl_655 {
    () => {
        deps!();
        impl < 'input , Endian > DebugTypes < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugTypes` instance from the data in the `.debug_types`"] # [doc = " section."] # [doc = ""] # [doc = " It is the caller's responsibility to read the `.debug_types` section and"] # [doc = " present it as a `&[u8]` slice. That means using some ELF loader on"] # [doc = " Linux, a Mach-O loader on macOS, etc."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugTypes, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [0x00, 0x01, 0x02, 0x03];"] # [doc = " # let read_debug_types_section_somehow = || &buf;"] # [doc = " let debug_types = DebugTypes::new(read_debug_types_section_somehow(), LittleEndian);"] # [doc = " ```"] pub fn new (debug_types_section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (debug_types_section , endian)) } }
    };
}

impl_655!()