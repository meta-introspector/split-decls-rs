macro_rules! deps {
    () => {
        DebugRngLists!();
        EndianSlice!();
        LittleEndian!();
        Endianity!();
    };
}

macro_rules! impl_557 {
    () => {
        deps!();
        impl < 'input , Endian > DebugRngLists < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugRngLists` instance from the data in the"] # [doc = " `.debug_rnglists` section."] # [doc = ""] # [doc = " It is the caller's responsibility to read the `.debug_rnglists`"] # [doc = " section and present it as a `&[u8]` slice. That means using some ELF"] # [doc = " loader on Linux, a Mach-O loader on macOS, etc."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugRngLists, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [0x00, 0x01, 0x02, 0x03];"] # [doc = " # let read_debug_rnglists_section_somehow = || &buf;"] # [doc = " let debug_rnglists ="] # [doc = "     DebugRngLists::new(read_debug_rnglists_section_somehow(), LittleEndian);"] # [doc = " ```"] pub fn new (section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (section , endian)) } }
    };
}

impl_557!()