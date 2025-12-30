// Generated macro for impl_687 (impl)
macro_rules! Depcrate_read_pubtypesimpl_687 {
() => {
// Module: crate::read::pubtypes
// Provides: {"impl_687"}
// Dependencies: {}
impl < R : Reader > DebugPubTypes < R > { # [doc = " Iterate the pubtypes in the `.debug_pubtypes` section."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugPubTypes, EndianSlice, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [];"] # [doc = " # let read_debug_pubtypes_section_somehow = || &buf;"] # [doc = " let debug_pubtypes ="] # [doc = "     DebugPubTypes::new(read_debug_pubtypes_section_somehow(), LittleEndian);"] # [doc = ""] # [doc = " let mut iter = debug_pubtypes.items();"] # [doc = " while let Some(pubtype) = iter.next().unwrap() {"] # [doc = "   println!(\"pubtype {} found!\", pubtype.name().to_string_lossy());"] # [doc = " }"] # [doc = " ```"] pub fn items (& self) -> PubTypesEntryIter < R > { PubTypesEntryIter (self . 0 . items ()) } }
};
}
