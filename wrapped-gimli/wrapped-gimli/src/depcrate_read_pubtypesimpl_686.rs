// Generated macro for impl_686 (impl)
macro_rules! Depcrate_read_pubtypesimpl_686 {
() => {
// Module: crate::read::pubtypes
// Provides: {"impl_686"}
// Dependencies: {}
impl < 'input , Endian > DebugPubTypes < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugPubTypes` instance from the data in the `.debug_pubtypes`"] # [doc = " section."] # [doc = ""] # [doc = " It is the caller's responsibility to read the `.debug_pubtypes` section and"] # [doc = " present it as a `&[u8]` slice. That means using some ELF loader on"] # [doc = " Linux, a Mach-O loader on macOS, etc."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugPubTypes, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [];"] # [doc = " # let read_debug_pubtypes_somehow = || &buf;"] # [doc = " let debug_pubtypes ="] # [doc = "     DebugPubTypes::new(read_debug_pubtypes_somehow(), LittleEndian);"] # [doc = " ```"] pub fn new (debug_pubtypes_section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (debug_pubtypes_section , endian)) } }
};
}
