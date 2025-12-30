// Generated macro for Dwarf (struct)
macro_rules! Depcrate_read_dwarfDwarf {
() => {
// Module: crate::read::dwarf
// Provides: {"Dwarf"}
// Dependencies: {}
# [doc = " All of the commonly used DWARF sections, and other common information."] # [derive (Debug , Default)] pub struct Dwarf < R > { # [doc = " The `.debug_abbrev` section."] pub debug_abbrev : DebugAbbrev < R > , # [doc = " The `.debug_addr` section."] pub debug_addr : DebugAddr < R > , # [doc = " The `.debug_aranges` section."] pub debug_aranges : DebugAranges < R > , # [doc = " The `.debug_info` section."] pub debug_info : DebugInfo < R > , # [doc = " The `.debug_line` section."] pub debug_line : DebugLine < R > , # [doc = " The `.debug_line_str` section."] pub debug_line_str : DebugLineStr < R > , # [doc = " The `.debug_macinfo` section."] pub debug_macinfo : DebugMacinfo < R > , # [doc = " The `.debug_macro` section."] pub debug_macro : DebugMacro < R > , # [doc = " The `.debug_str` section."] pub debug_str : DebugStr < R > , # [doc = " The `.debug_str_offsets` section."] pub debug_str_offsets : DebugStrOffsets < R > , # [doc = " The `.debug_types` section."] pub debug_types : DebugTypes < R > , # [doc = " The location lists in the `.debug_loc` and `.debug_loclists` sections."] pub locations : LocationLists < R > , # [doc = " The range lists in the `.debug_ranges` and `.debug_rnglists` sections."] pub ranges : RangeLists < R > , # [doc = " The type of this file."] pub file_type : DwarfFileType , # [doc = " The DWARF sections for a supplementary object file."] pub sup : Option < Arc < Dwarf < R > > > , # [doc = " A cache of previously parsed abbreviations for units in this file."] pub abbreviations_cache : AbbreviationsCache , }
};
}
