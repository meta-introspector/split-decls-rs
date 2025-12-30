// Generated macro for impl_565 (impl)
macro_rules! Depcrate_read_loclistsimpl_565 {
() => {
// Module: crate::read::loclists
// Provides: {"impl_565"}
// Dependencies: {}
impl < 'input , Endian > DebugLocLists < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugLocLists` instance from the data in the `.debug_loclists`"] # [doc = " section."] # [doc = ""] # [doc = " It is the caller's responsibility to read the `.debug_loclists` section and"] # [doc = " present it as a `&[u8]` slice. That means using some ELF loader on"] # [doc = " Linux, a Mach-O loader on macOS, etc."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugLocLists, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [0x00, 0x01, 0x02, 0x03];"] # [doc = " # let read_debug_loclists_section_somehow = || &buf;"] # [doc = " let debug_loclists = DebugLocLists::new(read_debug_loclists_section_somehow(), LittleEndian);"] # [doc = " ```"] pub fn new (section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (section , endian)) } }
};
}
