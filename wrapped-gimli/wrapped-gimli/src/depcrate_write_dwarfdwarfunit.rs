// Generated macro for DwarfUnit (struct)
macro_rules! Depcrate_write_dwarfDwarfUnit {
() => {
// Module: crate::write::dwarf
// Provides: {"DwarfUnit"}
// Dependencies: {}
# [doc = " Writable DWARF information for a single unit."] # [derive (Debug)] pub struct DwarfUnit { # [doc = " A unit. This is primarily stored in the `.debug_info` section,"] # [doc = " but also contains information that is stored in other sections."] pub unit : Unit , # [doc = " A table of strings that will be stored in the `.debug_line_str` section."] pub line_strings : LineStringTable , # [doc = " A table of strings that will be stored in the `.debug_str` section."] pub strings : StringTable , }
};
}
