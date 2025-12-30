// Generated macro for impl_683 (impl)
macro_rules! Depcrate_read_pubtypesimpl_683 {
() => {
// Module: crate::read::pubtypes
// Provides: {"impl_683"}
// Dependencies: {}
impl < R : Reader > PubTypesEntry < R > { # [doc = " Returns the name of the type this entry refers to."] pub fn name (& self) -> & R { & self . name } # [doc = " Returns the offset into the .debug_info section for the header of the compilation unit"] # [doc = " which contains the type with this name."] pub fn unit_header_offset (& self) -> DebugInfoOffset < R :: Offset > { self . unit_header_offset } # [doc = " Returns the offset into the compilation unit for the debugging information entry which"] # [doc = " the type with this name."] pub fn die_offset (& self) -> UnitOffset < R :: Offset > { self . die_offset } }
};
}
