macro_rules! deps {
    () => {
        Command!();
        Linker!();
        EmLinker!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < 'a > Linker for EmLinker < 'a > { fn cmd (& mut self) -> & mut Command { & mut self . cmd } fn is_cc (& self) -> bool { true } fn set_output_kind (& mut self , _output_kind : LinkOutputKind , _crate_type : CrateType , _out_filename : & Path ,) { } fn link_dylib_by_name (& mut self , name : & str , _verbatim : bool , _as_needed : bool) { self . link_or_cc_args (& ["-l" , name]) ; } fn link_dylib_by_path (& mut self , path : & Path , _as_needed : bool) { self . link_or_cc_arg (path) ; } fn link_staticlib_by_name (& mut self , name : & str , _verbatim : bool , _whole_archive : bool) { self . link_or_cc_args (& ["-l" , name]) ; } fn link_staticlib_by_path (& mut self , path : & Path , _whole_archive : bool) { self . link_or_cc_arg (path) ; } fn full_relro (& mut self) { } fn partial_relro (& mut self) { } fn no_relro (& mut self) { } fn gc_sections (& mut self , _keep_metadata : bool) { } fn optimize (& mut self) { self . cc_arg (match self . sess . opts . optimize { OptLevel :: No => "-O0" , OptLevel :: Less => "-O1" , OptLevel :: More => "-O2" , OptLevel :: Aggressive => "-O3" , OptLevel :: Size => "-Os" , OptLevel :: SizeMin => "-Oz" , }) ; } fn pgo_gen (& mut self) { } fn control_flow_guard (& mut self) { } fn ehcont_guard (& mut self) { } fn debuginfo (& mut self , _strip : Strip , _ : & [PathBuf]) { self . cc_arg (match self . sess . opts . debuginfo { DebugInfo :: None => "-g0" , DebugInfo :: Limited | DebugInfo :: LineTablesOnly | DebugInfo :: LineDirectivesOnly => { "--profiling-funcs" } DebugInfo :: Full => "-g" , }) ; } fn no_crt_objects (& mut self) { } fn no_default_libraries (& mut self) { self . cc_arg ("-nodefaultlibs") ; } fn export_symbols (& mut self , _tmpdir : & Path , _crate_type : CrateType , symbols : & [(String , SymbolExportKind)] ,) { debug ! ("EXPORTED SYMBOLS:") ; self . cc_arg ("-s") ; let mut arg = OsString :: from ("EXPORTED_FUNCTIONS=") ; let encoded = serde_json :: to_string (& symbols . iter () . map (| (sym , _) | "_" . to_owned () + sym) . collect :: < Vec < _ > > () ,) . unwrap () ; debug ! ("{encoded}") ; arg . push (encoded) ; self . cc_arg (arg) ; } fn subsystem (& mut self , _subsystem : & str) { } fn linker_plugin_lto (& mut self) { } }
    };
}

impl_120!()