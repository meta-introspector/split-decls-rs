// Generated macro for insert_reference_to_gdb_debug_scripts_section_global (function)
macro_rules! Depcrate_debuginfo_gdbinsert_reference_to_gdb_debug_scripts_section_global {
() => {
// Module: crate::debuginfo::gdb
// Provides: {"insert_reference_to_gdb_debug_scripts_section_global"}
// Dependencies: {}
# [doc = " Inserts a side-effect free instruction sequence that makes sure that the"] # [doc = " .debug_gdb_scripts global is referenced, so it isn't removed by the linker."] pub (crate) fn insert_reference_to_gdb_debug_scripts_section_global (bx : & mut Builder < '_ , '_ , '_ >) { if needs_gdb_debug_scripts_section (bx) { let gdb_debug_scripts_section = get_or_insert_gdb_debug_scripts_section_global (bx) ; let volatile_load_instruction = bx . volatile_load (bx . type_i8 () , gdb_debug_scripts_section) ; unsafe { llvm :: LLVMSetAlignment (volatile_load_instruction , 1) ; } } }
};
}
