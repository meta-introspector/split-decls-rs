// Generated macro for impl_670 (impl)
macro_rules! Depcrate_read_pubnamesimpl_670 {
() => {
// Module: crate::read::pubnames
// Provides: {"impl_670"}
// Dependencies: {}
impl < R : Reader > DebugPubNames < R > { # [doc = " Iterate the pubnames in the `.debug_pubnames` section."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugPubNames, EndianSlice, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [];"] # [doc = " # let read_debug_pubnames_section_somehow = || &buf;"] # [doc = " let debug_pubnames ="] # [doc = "     DebugPubNames::new(read_debug_pubnames_section_somehow(), LittleEndian);"] # [doc = ""] # [doc = " let mut iter = debug_pubnames.items();"] # [doc = " while let Some(pubname) = iter.next().unwrap() {"] # [doc = "   println!(\"pubname {} found!\", pubname.name().to_string_lossy());"] # [doc = " }"] # [doc = " ```"] pub fn items (& self) -> PubNamesEntryIter < R > { PubNamesEntryIter (self . 0 . items ()) } }
};
}
