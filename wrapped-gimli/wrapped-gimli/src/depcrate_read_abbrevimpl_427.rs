// Generated macro for impl_427 (impl)
macro_rules! Depcrate_read_abbrevimpl_427 {
() => {
// Module: crate::read::abbrev
// Provides: {"impl_427"}
// Dependencies: {}
impl < 'input , Endian > DebugAbbrev < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugAbbrev` instance from the data in the `.debug_abbrev`"] # [doc = " section."] # [doc = ""] # [doc = " It is the caller's responsibility to read the `.debug_abbrev` section and"] # [doc = " present it as a `&[u8]` slice. That means using some ELF loader on"] # [doc = " Linux, a Mach-O loader on macOS, etc."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugAbbrev, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [0x00, 0x01, 0x02, 0x03];"] # [doc = " # let read_debug_abbrev_section_somehow = || &buf;"] # [doc = " let debug_abbrev = DebugAbbrev::new(read_debug_abbrev_section_somehow(), LittleEndian);"] # [doc = " ```"] pub fn new (debug_abbrev_section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (debug_abbrev_section , endian)) } }
};
}
