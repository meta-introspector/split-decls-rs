macro_rules! deps {
    () => {
        Endianity!();
        EndianSlice!();
        LittleEndian!();
        DebugLoc!();
    };
}

macro_rules! impl_442 {
    () => {
        deps!();
        impl < 'input , Endian > DebugLoc < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugLoc` instance from the data in the `.debug_loc`"] # [doc = " section."] # [doc = ""] # [doc = " It is the caller's responsibility to read the `.debug_loc` section and"] # [doc = " present it as a `&[u8]` slice. That means using some ELF loader on"] # [doc = " Linux, a Mach-O loader on macOS, etc."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugLoc, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [0x00, 0x01, 0x02, 0x03];"] # [doc = " # let read_debug_loc_section_somehow = || &buf;"] # [doc = " let debug_loc = DebugLoc::new(read_debug_loc_section_somehow(), LittleEndian);"] # [doc = " ```"] pub fn new (section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (section , endian)) } }
    };
}

impl_442!()