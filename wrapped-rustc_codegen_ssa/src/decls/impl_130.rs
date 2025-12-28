macro_rules! deps {
    () => {
        Linker!();
        AixLinker!();
        Command!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < 'a > Linker for AixLinker < 'a > { fn cmd (& mut self) -> & mut Command { & mut self . cmd } fn set_output_kind (& mut self , output_kind : LinkOutputKind , _crate_type : CrateType , out_filename : & Path ,) { match output_kind { LinkOutputKind :: DynamicDylib => { self . hint_dynamic () ; self . build_dylib (out_filename) ; } LinkOutputKind :: StaticDylib => { self . hint_static () ; self . build_dylib (out_filename) ; } _ => { } } } fn link_dylib_by_name (& mut self , name : & str , verbatim : bool , _as_needed : bool) { self . hint_dynamic () ; self . link_or_cc_arg (if verbatim { String :: from (name) } else { format ! ("-l{name}") }) ; } fn link_dylib_by_path (& mut self , path : & Path , _as_needed : bool) { self . hint_dynamic () ; self . link_or_cc_arg (path) ; } fn link_staticlib_by_name (& mut self , name : & str , verbatim : bool , whole_archive : bool) { self . hint_static () ; if ! whole_archive { self . link_or_cc_arg (if verbatim { String :: from (name) } else { format ! ("-l{name}") }) ; } else { let mut arg = OsString :: from ("-bkeepfile:") ; arg . push (find_native_static_library (name , verbatim , self . sess)) ; self . link_or_cc_arg (arg) ; } } fn link_staticlib_by_path (& mut self , path : & Path , whole_archive : bool) { self . hint_static () ; if ! whole_archive { self . link_or_cc_arg (path) ; } else { let mut arg = OsString :: from ("-bkeepfile:") ; arg . push (path) ; self . link_arg (arg) ; } } fn full_relro (& mut self) { } fn partial_relro (& mut self) { } fn no_relro (& mut self) { } fn gc_sections (& mut self , _keep_metadata : bool) { self . link_arg ("-bgc") ; } fn optimize (& mut self) { } fn pgo_gen (& mut self) { self . link_arg ("-bdbg:namedsects:ss") ; self . link_arg ("-u") ; self . link_arg ("__llvm_profile_runtime") ; } fn control_flow_guard (& mut self) { } fn ehcont_guard (& mut self) { } fn debuginfo (& mut self , _ : Strip , _ : & [PathBuf]) { } fn no_crt_objects (& mut self) { } fn no_default_libraries (& mut self) { } fn export_symbols (& mut self , tmpdir : & Path , _crate_type : CrateType , symbols : & [(String , SymbolExportKind)] ,) { let path = tmpdir . join ("list.exp") ; let res : io :: Result < () > = try { let mut f = File :: create_buffered (& path) ? ; for (symbol , _) in symbols { debug ! ("  _{symbol}") ; writeln ! (f , "  {symbol}") ? ; } } ; if let Err (e) = res { self . sess . dcx () . fatal (format ! ("failed to write export file: {e}")) ; } self . link_arg (format ! ("-bE:{}" , path . to_str () . unwrap ())) ; } fn subsystem (& mut self , _subsystem : & str) { } fn reset_per_library_state (& mut self) { self . hint_dynamic () ; } fn linker_plugin_lto (& mut self) { } fn add_eh_frame_header (& mut self) { } fn add_no_exec (& mut self) { } fn add_as_needed (& mut self) { } }
    };
}

impl_130!()