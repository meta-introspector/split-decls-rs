// Generated macro for finalize (function)
macro_rules! Depcrate_debuginfofinalize {
() => {
// Module: crate::debuginfo
// Provides: {"finalize"}
// Dependencies: {}
# [doc = " Creates any deferred debug metadata nodes"] pub (crate) fn finalize (cx : & CodegenCx < '_ , '_ >) { if let Some (dbg_cx) = & cx . dbg_cx { debug ! ("finalize") ; if gdb :: needs_gdb_debug_scripts_section (cx) { gdb :: get_or_insert_gdb_debug_scripts_section_global (cx) ; } dbg_cx . finalize (cx . sess ()) ; } }
};
}
