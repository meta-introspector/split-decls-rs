macro_rules! deps {
    () => {
        EndianSlice!();
        DebugPubNames!();
        LittleEndian!();
        Endianity!();
    };
}

macro_rules! impl_531 {
    () => {
        deps!();
        impl < 'input , Endian > DebugPubNames < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugPubNames` instance from the data in the `.debug_pubnames`"] # [doc = " section."] # [doc = ""] # [doc = " It is the caller's responsibility to read the `.debug_pubnames` section and"] # [doc = " present it as a `&[u8]` slice. That means using some ELF loader on"] # [doc = " Linux, a Mach-O loader on macOS, etc."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugPubNames, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [];"] # [doc = " # let read_debug_pubnames_section_somehow = || &buf;"] # [doc = " let debug_pubnames ="] # [doc = "     DebugPubNames::new(read_debug_pubnames_section_somehow(), LittleEndian);"] # [doc = " ```"] pub fn new (debug_pubnames_section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (debug_pubnames_section , endian)) } }
    };
}

impl_531!();