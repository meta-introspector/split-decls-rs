// Generated macro for DebugContext (struct)
macro_rules! Depcrate_debuginfoDebugContext {
() => {
// Module: crate::debuginfo
// Provides: {"DebugContext"}
// Dependencies: {}
pub (crate) struct DebugContext { endian : RunTimeEndian , dwarf : DwarfUnit , unit_range_list : RangeList , created_files : FxHashMap < (StableSourceFileId , SourceFileHash) , FileId > , stack_pointer_register : Register , namespace_map : DefIdMap < UnitEntryId > , array_size_type : UnitEntryId , filename_display_preference : FileNameDisplayPreference , }
};
}
