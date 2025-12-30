// Generated macro for impl_820 (impl)
macro_rules! Depcrate_read_unitimpl_820 {
() => {
// Module: crate::read::unit
// Provides: {"impl_820"}
// Dependencies: {}
impl < R : Reader > DebugTypes < R > { # [doc = " Iterate the type-units in this `.debug_types` section."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugTypes, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [];"] # [doc = " # let read_debug_types_section_somehow = || &buf;"] # [doc = " let debug_types = DebugTypes::new(read_debug_types_section_somehow(), LittleEndian);"] # [doc = ""] # [doc = " let mut iter = debug_types.units();"] # [doc = " while let Some(unit) = iter.next().unwrap() {"] # [doc = "     println!(\"unit's length is {}\", unit.unit_length());"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] pub fn units (& self) -> DebugTypesUnitHeadersIter < R > { DebugTypesUnitHeadersIter { input : self . debug_types_section . clone () , offset : UnitSectionOffset (R :: Offset :: from_u8 (0)) , } } }
};
}
