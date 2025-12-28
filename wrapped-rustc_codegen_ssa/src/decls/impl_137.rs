macro_rules! deps {
    () => {
        Command!();
        Linker!();
        PtxLinker!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < 'a > Linker for PtxLinker < 'a > { fn cmd (& mut self) -> & mut Command { & mut self . cmd } fn set_output_kind (& mut self , _output_kind : LinkOutputKind , _crate_type : CrateType , _out_filename : & Path ,) { } fn link_staticlib_by_name (& mut self , _name : & str , _verbatim : bool , _whole_archive : bool) { panic ! ("staticlibs not supported") } fn link_staticlib_by_path (& mut self , path : & Path , _whole_archive : bool) { self . link_arg ("--rlib") . link_arg (path) ; } fn debuginfo (& mut self , _strip : Strip , _ : & [PathBuf]) { self . link_arg ("--debug") ; } fn add_object (& mut self , path : & Path) { self . link_arg ("--bitcode") . link_arg (path) ; } fn optimize (& mut self) { match self . sess . lto () { Lto :: Thin | Lto :: Fat | Lto :: ThinLocal => { self . link_arg ("-Olto") ; } Lto :: No => { } } } fn full_relro (& mut self) { } fn partial_relro (& mut self) { } fn no_relro (& mut self) { } fn gc_sections (& mut self , _keep_metadata : bool) { } fn pgo_gen (& mut self) { } fn no_crt_objects (& mut self) { } fn no_default_libraries (& mut self) { } fn control_flow_guard (& mut self) { } fn ehcont_guard (& mut self) { } fn export_symbols (& mut self , _tmpdir : & Path , _crate_type : CrateType , _symbols : & [(String , SymbolExportKind)] ,) { } fn subsystem (& mut self , _subsystem : & str) { } fn linker_plugin_lto (& mut self) { } }
    };
}

impl_137!();