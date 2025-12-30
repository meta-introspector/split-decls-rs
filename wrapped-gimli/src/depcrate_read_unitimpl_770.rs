// Generated macro for impl_770 (impl)
macro_rules! Depcrate_read_unitimpl_770 {
() => {
// Module: crate::read::unit
// Provides: {"impl_770"}
// Dependencies: {}
impl < R : Reader > DebugInfo < R > { # [doc = " Iterate the units in this `.debug_info` section."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugInfo, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [];"] # [doc = " # let read_debug_info_section_somehow = || &buf;"] # [doc = " let debug_info = DebugInfo::new(read_debug_info_section_somehow(), LittleEndian);"] # [doc = ""] # [doc = " let mut iter = debug_info.units();"] # [doc = " while let Some(unit) = iter.next().unwrap() {"] # [doc = "     println!(\"unit's length is {}\", unit.unit_length());"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] pub fn units (& self) -> DebugInfoUnitHeadersIter < R > { DebugInfoUnitHeadersIter { input : self . debug_info_section . clone () , offset : UnitSectionOffset (R :: Offset :: from_u8 (0)) , } } # [doc = " Get the UnitHeader located at offset from this .debug_info section."] # [doc = ""] # [doc = ""] pub fn header_from_offset (& self , offset : DebugInfoOffset < R :: Offset >) -> Result < UnitHeader < R > > { let input = & mut self . debug_info_section . clone () ; input . skip (offset . 0) ? ; parse_unit_header (input , SectionId :: DebugInfo , UnitSectionOffset (offset . 0)) } }
};
}
