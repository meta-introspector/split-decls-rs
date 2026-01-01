/* FP:lto.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_USE_0001
/* FP:lto.rs-0002 */ use std :: ffi :: CString ;
/* FP:lto.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_USE_0002
/* FP:lto.rs-0004 */ use std :: sync :: Arc ;
/* FP:lto.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_USE_0003
/* FP:lto.rs-0006 */ use crate :: rustc_data_structures :: memmap :: Mmap ;
/* FP:lto.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_USE_0004
/* FP:lto.rs-0008 */ use crate :: rustc_complete :: def_id :: { CrateNum , LOCAL_CRATE } ;
/* FP:lto.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_USE_0005
/* FP:lto.rs-0010 */ use crate :: rustc_complete :: middle :: exported_symbols :: { ExportedSymbol , SymbolExportInfo , SymbolExportLevel } ;
/* FP:lto.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_USE_0006
/* FP:lto.rs-0012 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:lto.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_USE_0007
/* FP:lto.rs-0014 */ use crate :: rustc_complete :: config :: { CrateType , Lto } ;
/* FP:lto.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_USE_0008
/* FP:lto.rs-0016 */ use tracing :: info ;
/* FP:lto.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_USE_0009
/* FP:lto.rs-0018 */ use crate :: back :: symbol_export :: { self , allocator_shim_symbols , symbol_name_for_instance_in_crate } ;
/* FP:lto.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_USE_0010
/* FP:lto.rs-0020 */ use crate :: back :: write :: CodegenContext ;
/* FP:lto.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_USE_0011
/* FP:lto.rs-0022 */ use crate :: base :: allocator_kind_for_codegen ;
/* FP:lto.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_USE_0012
/* FP:lto.rs-0024 */ use crate :: errors :: { DynamicLinkingWithLTO , LtoDisallowed , LtoDylib , LtoProcMacro } ;
/* FP:lto.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_USE_0013
/* FP:lto.rs-0026 */ use crate :: traits :: * ;
/* FP:lto.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_STRUCT_0014
/* FP:lto.rs-0028 */ pub struct ThinModule < B : WriteBackendMethods > { pub shared : Arc < ThinShared < B > > , pub idx : usize , }
/* FP:lto.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_IMPL_0015
/* FP:lto.rs-0030 */ impl < B : WriteBackendMethods > ThinModule < B > { pub fn name (& self) -> & str { self . shared . module_names [self . idx] . to_str () . unwrap () } pub fn cost (& self) -> u64 { self . data () . len () as u64 } pub fn data (& self) -> & [u8] { let a = self . shared . thin_buffers . get (self . idx) . map (| b | b . data ()) ; a . unwrap_or_else (| | { let len = self . shared . thin_buffers . len () ; self . shared . serialized_modules [self . idx - len] . data () }) } }
/* FP:lto.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_STRUCT_0016
/* FP:lto.rs-0032 */ pub struct ThinShared < B : WriteBackendMethods > { pub data : B :: ThinData , pub thin_buffers : Vec < B :: ThinBuffer > , pub serialized_modules : Vec < SerializedModule < B :: ModuleBuffer > > , pub module_names : Vec < CString > , }
/* FP:lto.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_ENUM_0017
/* FP:lto.rs-0034 */ pub enum SerializedModule < M : ModuleBufferMethods > { Local (M) , FromRlib (Vec < u8 >) , FromUncompressedFile (Mmap) , }
/* FP:lto.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_IMPL_0018
/* FP:lto.rs-0036 */ impl < M : ModuleBufferMethods > SerializedModule < M > { pub fn data (& self) -> & [u8] { match * self { SerializedModule :: Local (ref m) => m . data () , SerializedModule :: FromRlib (ref m) => m , SerializedModule :: FromUncompressedFile (ref m) => m , } } }
/* FP:lto.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_FN_0019
/* FP:lto.rs-0038 */ fn crate_type_allows_lto (crate_type : CrateType) -> bool { match crate_type { CrateType :: Executable | CrateType :: Dylib | CrateType :: Staticlib | CrateType :: Cdylib | CrateType :: ProcMacro | CrateType :: Sdylib => true , CrateType :: Rlib => false , } }
/* FP:lto.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_FN_0020
/* FP:lto.rs-0040 */ pub (super) fn exported_symbols_for_lto (tcx : TyCtxt < '_ > , each_linked_rlib_for_lto : & [CrateNum] ,) -> Vec < String > { let export_threshold = match tcx . sess . lto () { Lto :: ThinLocal => SymbolExportLevel :: Rust , Lto :: Fat | Lto :: Thin => symbol_export :: crates_export_threshold (& tcx . crate_types ()) , Lto :: No => return vec ! [] , } ; let copy_symbols = | cnum | { tcx . exported_non_generic_symbols (cnum) . iter () . chain (tcx . exported_generic_symbols (cnum)) . filter_map (| & (s , info) : & (ExportedSymbol < '_ > , SymbolExportInfo) | { if info . level . is_below_threshold (export_threshold) || info . used { Some (symbol_name_for_instance_in_crate (tcx , s , cnum)) } else { None } }) . collect :: < Vec < _ > > () } ; let mut symbols_below_threshold = { let _timer = tcx . prof . generic_activity ("lto_generate_symbols_below_threshold") ; copy_symbols (LOCAL_CRATE) } ; info ! ("{} symbols to preserve in this crate" , symbols_below_threshold . len ()) ; if tcx . sess . lto () != Lto :: ThinLocal { for & cnum in each_linked_rlib_for_lto { let _timer = tcx . prof . generic_activity ("lto_generate_symbols_below_threshold") ; symbols_below_threshold . extend (copy_symbols (cnum)) ; } } if export_threshold == SymbolExportLevel :: Rust && allocator_kind_for_codegen (tcx) . is_some () { symbols_below_threshold . extend (allocator_shim_symbols (tcx) . map (| (name , _kind) | name)) ; } symbols_below_threshold }
/* FP:lto.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_back_lto_FN_0021
/* FP:lto.rs-0042 */ pub (super) fn check_lto_allowed < B : WriteBackendMethods > (cgcx : & CodegenContext < B >) { if cgcx . lto == Lto :: ThinLocal { return ; } let dcx = cgcx . create_dcx () ; for crate_type in cgcx . crate_types . iter () { if ! crate_type_allows_lto (* crate_type) { dcx . handle () . emit_fatal (LtoDisallowed) ; } else if * crate_type == CrateType :: Dylib { if ! cgcx . opts . unstable_opts . dylib_lto { dcx . handle () . emit_fatal (LtoDylib) ; } } else if * crate_type == CrateType :: ProcMacro && ! cgcx . opts . unstable_opts . dylib_lto { dcx . handle () . emit_fatal (LtoProcMacro) ; } } if cgcx . opts . cg . prefer_dynamic && ! cgcx . opts . unstable_opts . dylib_lto { dcx . handle () . emit_fatal (DynamicLinkingWithLTO) ; } }