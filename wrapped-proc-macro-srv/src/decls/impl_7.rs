macro_rules! deps {
    () => {
        ProcMacroLibrary!();
        PanicMessage!();
        LoadProcMacroDylibError!();
        TopSubtree!();
        ProcMacroKind!();
        Expander!();
        ProcMacroSrvSpan!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Expander { pub (crate) fn new (temp_dir : & TempDir , lib : & Utf8Path ,) -> Result < Expander , LoadProcMacroDylibError > { let lib = lib . canonicalize_utf8 () ? ; let modified_time = fs :: metadata (& lib) . and_then (| it | it . modified ()) ? ; let path = ensure_file_with_lock_free_access (temp_dir , & lib) ? ; let library = ProcMacroLibrary :: open (path . as_ref ()) ? ; Ok (Expander { inner : library , modified_time }) } pub (crate) fn expand < S : ProcMacroSrvSpan > (& self , macro_name : & str , macro_body : TopSubtree < S > , attributes : Option < TopSubtree < S > > , def_site : S , call_site : S , mixed_site : S ,) -> Result < TopSubtree < S > , PanicMessage > where < S :: Server as bridge :: server :: Types > :: TokenStream : Default , { self . inner . proc_macros . expand (macro_name , macro_body , attributes , def_site , call_site , mixed_site) } pub (crate) fn list_macros (& self) -> impl Iterator < Item = (& str , ProcMacroKind) > { self . inner . proc_macros . list_macros () } pub (crate) fn modified_time (& self) -> SystemTime { self . modified_time } }
    };
}

impl_7!();