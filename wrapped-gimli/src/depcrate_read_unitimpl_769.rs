// Generated macro for impl_769 (impl)
macro_rules! Depcrate_read_unitimpl_769 {
() => {
// Module: crate::read::unit
// Provides: {"impl_769"}
// Dependencies: {}
impl < 'input , Endian > DebugInfo < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugInfo` instance from the data in the `.debug_info`"] # [doc = " section."] # [doc = ""] # [doc = " It is the caller's responsibility to read the `.debug_info` section and"] # [doc = " present it as a `&[u8]` slice. That means using some ELF loader on"] # [doc = " Linux, a Mach-O loader on macOS, etc."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugInfo, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [0x00, 0x01, 0x02, 0x03];"] # [doc = " # let read_debug_info_section_somehow = || &buf;"] # [doc = " let debug_info = DebugInfo::new(read_debug_info_section_somehow(), LittleEndian);"] # [doc = " ```"] pub fn new (debug_info_section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (debug_info_section , endian)) } }
};
}
