mkuse!{use std :: ffi :: CString ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use rustc_data_structures :: memmap :: Mmap ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , LOCAL_CRATE } ;}
mkuse!{use rustc_middle :: middle :: exported_symbols :: { ExportedSymbol , SymbolExportInfo , SymbolExportLevel } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: config :: { CrateType , Lto } ;}
mkuse!{use tracing :: info ;}
mkuse!{use crate :: back :: symbol_export :: { self , allocator_shim_symbols , symbol_name_for_instance_in_crate } ;}
mkuse!{use crate :: back :: write :: CodegenContext ;}
mkuse!{use crate :: base :: allocator_kind_for_codegen ;}
mkuse!{use crate :: errors :: { DynamicLinkingWithLTO , LtoDisallowed , LtoDylib , LtoProcMacro } ;}
mkuse!{use crate :: traits :: * ;}
mkitem!{mkstruct!{pub struct ThinModule < B : WriteBackendMethods > { pub shared : Arc < ThinShared < B > > , pub idx : usize , }}}
mkitem!{mkimpl!{impl < B : WriteBackendMethods > ThinModule < B > { pub fn name (& self) -> & str { self . shared . module_names [self . idx] . to_str () . unwrap () } pub fn cost (& self) -> u64 { self . data () . len () as u64 } pub fn data (& self) -> & [u8] { let a = self . shared . thin_buffers . get (self . idx) . map (| b | b . data ()) ; a . unwrap_or_else (| | { let len = self . shared . thin_buffers . len () ; self . shared . serialized_modules [self . idx - len] . data () }) } }}}
mkitem!{mkstruct!{pub struct ThinShared < B : WriteBackendMethods > { pub data : B :: ThinData , pub thin_buffers : Vec < B :: ThinBuffer > , pub serialized_modules : Vec < SerializedModule < B :: ModuleBuffer > > , pub module_names : Vec < CString > , }}}
mkitem!{mkenum!{pub enum SerializedModule < M : ModuleBufferMethods > { Local (M) , FromRlib (Vec < u8 >) , FromUncompressedFile (Mmap) , }}}
mkitem!{mkimpl!{impl < M : ModuleBufferMethods > SerializedModule < M > { pub fn data (& self) -> & [u8] { match * self { SerializedModule :: Local (ref m) => m . data () , SerializedModule :: FromRlib (ref m) => m , SerializedModule :: FromUncompressedFile (ref m) => m , } } }}}

macro_rules! crate_type_allows_lto_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crate_type_allows_lto in module {}", module_path!());
    };
}

mkfn!{
    crate_type_allows_lto_introspect!();
    fn crate_type_allows_lto (crate_type : CrateType) -> bool { match crate_type { CrateType :: Executable | CrateType :: Dylib | CrateType :: Staticlib | CrateType :: Cdylib | CrateType :: ProcMacro | CrateType :: Sdylib => true , CrateType :: Rlib => false , } }
}

macro_rules! exported_symbols_for_lto_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exported_symbols_for_lto in module {}", module_path!());
    };
}

mkfn!{
    exported_symbols_for_lto_introspect!();
    pub (super) fn exported_symbols_for_lto (tcx : TyCtxt < '_ > , each_linked_rlib_for_lto : & [CrateNum] ,) -> Vec < String > { let export_threshold = match tcx . sess . lto () { Lto :: ThinLocal => SymbolExportLevel :: Rust , Lto :: Fat | Lto :: Thin => symbol_export :: crates_export_threshold (& tcx . crate_types ()) , Lto :: No => return vec ! [] , } ; let copy_symbols = | cnum | { tcx . exported_non_generic_symbols (cnum) . iter () . chain (tcx . exported_generic_symbols (cnum)) . filter_map (| & (s , info) : & (ExportedSymbol < '_ > , SymbolExportInfo) | { if info . level . is_below_threshold (export_threshold) || info . used { Some (symbol_name_for_instance_in_crate (tcx , s , cnum)) } else { None } }) . collect :: < Vec < _ > > () } ; let mut symbols_below_threshold = { let _timer = tcx . prof . generic_activity ("lto_generate_symbols_below_threshold") ; copy_symbols (LOCAL_CRATE) } ; info ! ("{} symbols to preserve in this crate" , symbols_below_threshold . len ()) ; if tcx . sess . lto () != Lto :: ThinLocal { for & cnum in each_linked_rlib_for_lto { let _timer = tcx . prof . generic_activity ("lto_generate_symbols_below_threshold") ; symbols_below_threshold . extend (copy_symbols (cnum)) ; } } if export_threshold == SymbolExportLevel :: Rust && allocator_kind_for_codegen (tcx) . is_some () { symbols_below_threshold . extend (allocator_shim_symbols (tcx) . map (| (name , _kind) | name)) ; } symbols_below_threshold }
}

macro_rules! check_lto_allowed_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_lto_allowed in module {}", module_path!());
    };
}

mkfn!{
    check_lto_allowed_introspect!();
    pub (super) fn check_lto_allowed < B : WriteBackendMethods > (cgcx : & CodegenContext < B >) { if cgcx . lto == Lto :: ThinLocal { return ; } let dcx = cgcx . create_dcx () ; for crate_type in cgcx . crate_types . iter () { if ! crate_type_allows_lto (* crate_type) { dcx . handle () . emit_fatal (LtoDisallowed) ; } else if * crate_type == CrateType :: Dylib { if ! cgcx . opts . unstable_opts . dylib_lto { dcx . handle () . emit_fatal (LtoDylib) ; } } else if * crate_type == CrateType :: ProcMacro && ! cgcx . opts . unstable_opts . dylib_lto { dcx . handle () . emit_fatal (LtoProcMacro) ; } } if cgcx . opts . cg . prefer_dynamic && ! cgcx . opts . unstable_opts . dylib_lto { dcx . handle () . emit_fatal (DynamicLinkingWithLTO) ; } }
}