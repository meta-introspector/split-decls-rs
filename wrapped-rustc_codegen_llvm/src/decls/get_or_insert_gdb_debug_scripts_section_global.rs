macro_rules! deps {
    () => {
        Linkage!();
        CodegenCx!();
        UnnamedAddr!();
    };
}

macro_rules! get_or_insert_gdb_debug_scripts_section_global {
    () => {
        deps!();
        # [doc = " Allocates the global variable responsible for the .debug_gdb_scripts binary"] # [doc = " section."] pub (crate) fn get_or_insert_gdb_debug_scripts_section_global < 'll > (cx : & CodegenCx < 'll , '_ > ,) -> & 'll Value { let c_section_var_name = c"__rustc_debug_gdb_scripts_section__" ; let section_var_name = c_section_var_name . to_str () . unwrap () ; let section_var = unsafe { llvm :: LLVMGetNamedGlobal (cx . llmod , c_section_var_name . as_ptr ()) } ; section_var . unwrap_or_else (| | { let mut section_contents = Vec :: new () ; section_contents . extend_from_slice (b"\x01gdb_load_rust_pretty_printers.py\0") ; let visualizers = collect_debugger_visualizers_transitive (cx . tcx , DebuggerVisualizerType :: GdbPrettyPrinter ,) ; let crate_name = cx . tcx . crate_name (LOCAL_CRATE) ; for (index , visualizer) in visualizers . iter () . enumerate () { section_contents . extend_from_slice (b"\x04") ; let vis_name = format ! ("pretty-printer-{crate_name}-{index}\n") ; section_contents . extend_from_slice (vis_name . as_bytes ()) ; section_contents . extend_from_slice (& visualizer . src) ; section_contents . extend_from_slice (b"\0") ; } unsafe { let section_contents = section_contents . as_slice () ; let llvm_type = cx . type_array (cx . type_i8 () , section_contents . len () as u64) ; let section_var = cx . define_global (section_var_name , llvm_type) . unwrap_or_else (| | bug ! ("symbol `{}` is already defined" , section_var_name)) ; llvm :: set_section (section_var , c".debug_gdb_scripts") ; llvm :: set_initializer (section_var , cx . const_bytes (section_contents)) ; llvm :: LLVMSetGlobalConstant (section_var , llvm :: TRUE) ; llvm :: set_unnamed_address (section_var , llvm :: UnnamedAddr :: Global) ; llvm :: set_linkage (section_var , llvm :: Linkage :: LinkOnceODRLinkage) ; llvm :: LLVMSetAlignment (section_var , 1) ; section_var } }) }
    };
}

get_or_insert_gdb_debug_scripts_section_global!();