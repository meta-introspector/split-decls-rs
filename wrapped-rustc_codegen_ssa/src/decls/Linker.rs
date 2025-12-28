macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! Linker {
    () => {
        deps!();
        # [doc = " Linker abstraction used by `back::link` to build up the command to invoke a"] # [doc = " linker."] # [doc = ""] # [doc = " This trait is the total list of requirements needed by `back::link` and"] # [doc = " represents the meaning of each option being passed down. This trait is then"] # [doc = " used to dispatch on whether a GNU-like linker (generally `ld.exe`) or an"] # [doc = " MSVC linker (e.g., `link.exe`) is being used."] pub (crate) trait Linker { fn cmd (& mut self) -> & mut Command ; fn is_cc (& self) -> bool { false } fn set_output_kind (& mut self , output_kind : LinkOutputKind , crate_type : CrateType , out_filename : & Path ,) ; fn link_dylib_by_name (& mut self , _name : & str , _verbatim : bool , _as_needed : bool) { bug ! ("dylib linked with unsupported linker") } fn link_dylib_by_path (& mut self , _path : & Path , _as_needed : bool) { bug ! ("dylib linked with unsupported linker") } fn link_framework_by_name (& mut self , _name : & str , _verbatim : bool , _as_needed : bool) { bug ! ("framework linked with unsupported linker") } fn link_staticlib_by_name (& mut self , name : & str , verbatim : bool , whole_archive : bool) ; fn link_staticlib_by_path (& mut self , path : & Path , whole_archive : bool) ; fn include_path (& mut self , path : & Path) { link_or_cc_args (link_or_cc_args (self , & ["-L"]) , & [path]) ; } fn framework_path (& mut self , _path : & Path) { bug ! ("framework path set with unsupported linker") } fn output_filename (& mut self , path : & Path) { link_or_cc_args (link_or_cc_args (self , & ["-o"]) , & [path]) ; } fn add_object (& mut self , path : & Path) { link_or_cc_args (self , & [path]) ; } fn gc_sections (& mut self , keep_metadata : bool) ; fn full_relro (& mut self) ; fn partial_relro (& mut self) ; fn no_relro (& mut self) ; fn optimize (& mut self) ; fn pgo_gen (& mut self) ; fn control_flow_guard (& mut self) ; fn ehcont_guard (& mut self) ; fn debuginfo (& mut self , strip : Strip , natvis_debugger_visualizers : & [PathBuf]) ; fn no_crt_objects (& mut self) ; fn no_default_libraries (& mut self) ; fn export_symbols (& mut self , tmpdir : & Path , crate_type : CrateType , symbols : & [(String , SymbolExportKind)] ,) ; fn subsystem (& mut self , subsystem : & str) ; fn linker_plugin_lto (& mut self) ; fn add_eh_frame_header (& mut self) { } fn add_no_exec (& mut self) { } fn add_as_needed (& mut self) { } fn reset_per_library_state (& mut self) { } }
    };
}

Linker!()