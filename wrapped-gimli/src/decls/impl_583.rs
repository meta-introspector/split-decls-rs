macro_rules! deps {
    () => {
        LittleEndian!();
        EndianSlice!();
        Endianity!();
        DebugStr!();
    };
}

macro_rules! impl_583 {
    () => {
        deps!();
        impl < 'input , Endian > DebugStr < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugStr` instance from the data in the `.debug_str`"] # [doc = " section."] # [doc = ""] # [doc = " It is the caller's responsibility to read the `.debug_str` section and"] # [doc = " present it as a `&[u8]` slice. That means using some ELF loader on"] # [doc = " Linux, a Mach-O loader on macOS, etc."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugStr, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [0x00, 0x01, 0x02, 0x03];"] # [doc = " # let read_debug_str_section_somehow = || &buf;"] # [doc = " let debug_str = DebugStr::new(read_debug_str_section_somehow(), LittleEndian);"] # [doc = " ```"] pub fn new (debug_str_section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (debug_str_section , endian)) } }
    };
}

impl_583!()