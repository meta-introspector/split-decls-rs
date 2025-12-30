// Generated macro for impl_666 (impl)
macro_rules! Depcrate_read_pubnamesimpl_666 {
() => {
// Module: crate::read::pubnames
// Provides: {"impl_666"}
// Dependencies: {}
impl < R : Reader > PubNamesEntry < R > { # [doc = " Returns the name this entry refers to."] pub fn name (& self) -> & R { & self . name } # [doc = " Returns the offset into the .debug_info section for the header of the compilation unit"] # [doc = " which contains this name."] pub fn unit_header_offset (& self) -> DebugInfoOffset < R :: Offset > { self . unit_header_offset } # [doc = " Returns the offset into the compilation unit for the debugging information entry which"] # [doc = " has this name."] pub fn die_offset (& self) -> UnitOffset < R :: Offset > { self . die_offset } }
};
}
