macro_rules! deps {
    () => {
        EndianSlice!();
        LittleEndian!();
        DebugRanges!();
        Endianity!();
    };
}

macro_rules! impl_552 {
    () => {
        deps!();
        impl < 'input , Endian > DebugRanges < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugRanges` instance from the data in the `.debug_ranges`"] # [doc = " section."] # [doc = ""] # [doc = " It is the caller's responsibility to read the `.debug_ranges` section and"] # [doc = " present it as a `&[u8]` slice. That means using some ELF loader on"] # [doc = " Linux, a Mach-O loader on macOS, etc."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugRanges, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [0x00, 0x01, 0x02, 0x03];"] # [doc = " # let read_debug_ranges_section_somehow = || &buf;"] # [doc = " let debug_ranges = DebugRanges::new(read_debug_ranges_section_somehow(), LittleEndian);"] # [doc = " ```"] pub fn new (section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (section , endian)) } }
    };
}

impl_552!()