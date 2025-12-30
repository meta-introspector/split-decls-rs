// Generated macro for impl_258 (impl)
macro_rules! Depcrate_debuginfo_emitimpl_258 {
() => {
// Module: crate::debuginfo::emit
// Provides: {"impl_258"}
// Dependencies: {}
impl DebugContext { pub (crate) fn emit (& mut self , product : & mut ObjectProduct) { let unit_range_list_id = self . dwarf . unit . ranges . add (self . unit_range_list . clone ()) ; let root = self . dwarf . unit . root () ; let root = self . dwarf . unit . get_mut (root) ; root . set (gimli :: DW_AT_ranges , AttributeValue :: RangeListRef (unit_range_list_id)) ; let mut sections = Sections :: new (WriterRelocate :: new (self . endian)) ; self . dwarf . write (& mut sections) . unwrap () ; let mut section_map = FxHashMap :: default () ; let _ : Result < () > = sections . for_each_mut (| id , section | { if ! section . writer . slice () . is_empty () { let section_id = product . add_debug_section (id , section . writer . take ()) ; section_map . insert (id , section_id) ; } Ok (()) }) ; let _ : Result < () > = sections . for_each (| id , section | { if let Some (section_id) = section_map . get (& id) { for reloc in & section . relocs { product . add_debug_reloc (& section_map , section_id , reloc) ; } } Ok (()) }) ; } }
};
}
