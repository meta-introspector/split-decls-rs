macro_rules! deps {
    () => {
        SymbolFileWriteFailure!();
        Command!();
        BpfLinker!();
        Linker!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < 'a > Linker for BpfLinker < 'a > { fn cmd (& mut self) -> & mut Command { & mut self . cmd } fn set_output_kind (& mut self , _output_kind : LinkOutputKind , _crate_type : CrateType , _out_filename : & Path ,) { } fn link_staticlib_by_name (& mut self , _name : & str , _verbatim : bool , _whole_archive : bool) { panic ! ("staticlibs not supported") } fn link_staticlib_by_path (& mut self , path : & Path , _whole_archive : bool) { self . link_or_cc_arg (path) ; } fn debuginfo (& mut self , _strip : Strip , _ : & [PathBuf]) { self . link_arg ("--debug") ; } fn optimize (& mut self) { self . link_arg (match self . sess . opts . optimize { OptLevel :: No => "-O0" , OptLevel :: Less => "-O1" , OptLevel :: More => "-O2" , OptLevel :: Aggressive => "-O3" , OptLevel :: Size => "-Os" , OptLevel :: SizeMin => "-Oz" , }) ; } fn full_relro (& mut self) { } fn partial_relro (& mut self) { } fn no_relro (& mut self) { } fn gc_sections (& mut self , _keep_metadata : bool) { } fn pgo_gen (& mut self) { } fn no_crt_objects (& mut self) { } fn no_default_libraries (& mut self) { } fn control_flow_guard (& mut self) { } fn ehcont_guard (& mut self) { } fn export_symbols (& mut self , tmpdir : & Path , _crate_type : CrateType , symbols : & [(String , SymbolExportKind)] ,) { let path = tmpdir . join ("symbols") ; let res : io :: Result < () > = try { let mut f = File :: create_buffered (& path) ? ; for (sym , _) in symbols { writeln ! (f , "{sym}") ? ; } } ; if let Err (error) = res { self . sess . dcx () . emit_fatal (errors :: SymbolFileWriteFailure { error }) ; } else { self . link_arg ("--export-symbols") . link_arg (& path) ; } } fn subsystem (& mut self , _subsystem : & str) { } fn linker_plugin_lto (& mut self) { } }
    };
}

impl_141!()