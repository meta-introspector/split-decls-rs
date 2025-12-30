// Generated macro for impl_458 (impl)
macro_rules! Depcrate_read_arangesimpl_458 {
() => {
// Module: crate::read::aranges
// Provides: {"impl_458"}
// Dependencies: {}
impl < 'input , Endian > DebugAranges < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugAranges` instance from the data in the `.debug_aranges`"] # [doc = " section."] # [doc = ""] # [doc = " It is the caller's responsibility to read the `.debug_aranges` section and"] # [doc = " present it as a `&[u8]` slice. That means using some ELF loader on"] # [doc = " Linux, a Mach-O loader on macOS, etc."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugAranges, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [];"] # [doc = " # let read_debug_aranges_section = || &buf;"] # [doc = " let debug_aranges ="] # [doc = "     DebugAranges::new(read_debug_aranges_section(), LittleEndian);"] # [doc = " ```"] pub fn new (section : & 'input [u8] , endian : Endian) -> Self { DebugAranges { section : EndianSlice :: new (section , endian) , } } }
};
}
