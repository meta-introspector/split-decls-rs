macro_rules! deps {
    () => {
        Command!();
        L4BenderExportingSymbolsUnimplemented!();
        Linker!();
        L4Bender!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < 'a > Linker for L4Bender < 'a > { fn cmd (& mut self) -> & mut Command { & mut self . cmd } fn set_output_kind (& mut self , _output_kind : LinkOutputKind , _crate_type : CrateType , _out_filename : & Path ,) { } fn link_staticlib_by_name (& mut self , name : & str , _verbatim : bool , whole_archive : bool) { self . hint_static () ; if ! whole_archive { self . link_arg (format ! ("-PC{name}")) ; } else { self . link_arg ("--whole-archive") . link_or_cc_arg (format ! ("-l{name}")) . link_arg ("--no-whole-archive") ; } } fn link_staticlib_by_path (& mut self , path : & Path , whole_archive : bool) { self . hint_static () ; if ! whole_archive { self . link_or_cc_arg (path) ; } else { self . link_arg ("--whole-archive") . link_or_cc_arg (path) . link_arg ("--no-whole-archive") ; } } fn full_relro (& mut self) { self . link_args (& ["-z" , "relro" , "-z" , "now"]) ; } fn partial_relro (& mut self) { self . link_args (& ["-z" , "relro"]) ; } fn no_relro (& mut self) { self . link_args (& ["-z" , "norelro"]) ; } fn gc_sections (& mut self , keep_metadata : bool) { if ! keep_metadata { self . link_arg ("--gc-sections") ; } } fn optimize (& mut self) { if self . sess . opts . optimize == config :: OptLevel :: More || self . sess . opts . optimize == config :: OptLevel :: Aggressive { self . link_arg ("-O1") ; } } fn pgo_gen (& mut self) { } fn debuginfo (& mut self , strip : Strip , _ : & [PathBuf]) { match strip { Strip :: None => { } Strip :: Debuginfo => { self . link_arg ("--strip-debug") ; } Strip :: Symbols => { self . link_arg ("--strip-all") ; } } } fn no_default_libraries (& mut self) { self . cc_arg ("-nostdlib") ; } fn export_symbols (& mut self , _ : & Path , _ : CrateType , _ : & [(String , SymbolExportKind)]) { self . sess . dcx () . emit_warn (errors :: L4BenderExportingSymbolsUnimplemented) ; } fn subsystem (& mut self , subsystem : & str) { self . link_arg (& format ! ("--subsystem {subsystem}")) ; } fn reset_per_library_state (& mut self) { self . hint_static () ; } fn linker_plugin_lto (& mut self) { } fn control_flow_guard (& mut self) { } fn ehcont_guard (& mut self) { } fn no_crt_objects (& mut self) { } }
    };
}

impl_126!()