/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0001
/* FP:lib.rs-0002 */ # [allow (internal_features)] # [allow (rustc :: diagnostic_outside_of_impl)] # [allow (rustc :: untranslatable_diagnostic)] # [doc (html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")] # [doc (rust_logo)] # [feature (assert_matches)] # [feature (box_patterns)] # [feature (file_buffered)] # [feature (if_let_guard)] # [feature (negative_impls)] # [feature (rustdoc_internals)] # [feature (string_from_utf8_lossy_owned)] # [feature (trait_alias)] # [feature (try_blocks)] # [recursion_limit = "256"] use std :: collections :: BTreeSet ;
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0002
/* FP:lib.rs-0004 */ use std :: io ;
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0003
/* FP:lib.rs-0006 */ use std :: path :: { Path , PathBuf } ;
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0004
/* FP:lib.rs-0008 */ use std :: sync :: Arc ;
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0005
/* FP:lib.rs-0010 */ use crate :: rustc_data_structures :: fx :: { FxHashSet , FxIndexMap } ;
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0006
/* FP:lib.rs-0012 */ use crate :: rustc_data_structures :: unord :: UnordMap ;
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0007
/* FP:lib.rs-0014 */ use crate :: rustc_complete :: CRATE_HIR_ID ;
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0008
/* FP:lib.rs-0016 */ use crate :: rustc_complete :: attrs :: { CfgEntry , NativeLibKind } ;
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0009
/* FP:lib.rs-0018 */ use crate :: rustc_complete :: def_id :: CrateNum ;
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0010
/* FP:lib.rs-0020 */ use rustc_macros :: { Decodable , Encodable , HashStable } ;
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0011
/* FP:lib.rs-0022 */ use crate :: rustc_metadata :: EncodedMetadata ;
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0012
/* FP:lib.rs-0024 */ use crate :: rustc_complete :: dep_graph :: WorkProduct ;
/* FP:lib.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0013
/* FP:lib.rs-0026 */ use crate :: rustc_complete :: lint :: LevelAndSource ;
/* FP:lib.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0014
/* FP:lib.rs-0028 */ use crate :: rustc_complete :: middle :: debugger_visualizer :: DebuggerVisualizerFile ;
/* FP:lib.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0015
/* FP:lib.rs-0030 */ use crate :: rustc_complete :: middle :: dependency_format :: Dependencies ;
/* FP:lib.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0016
/* FP:lib.rs-0032 */ use crate :: rustc_complete :: middle :: exported_symbols :: SymbolExportKind ;
/* FP:lib.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0017
/* FP:lib.rs-0034 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:lib.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0018
/* FP:lib.rs-0036 */ use crate :: rustc_complete :: util :: Providers ;
/* FP:lib.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0019
/* FP:lib.rs-0038 */ use crate :: rustc_serialize :: opaque :: { FileEncoder , MemDecoder } ;
/* FP:lib.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0020
/* FP:lib.rs-0040 */ use crate :: rustc_serialize :: { Decodable , Decoder , Encodable , Encoder } ;
/* FP:lib.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0021
/* FP:lib.rs-0042 */ use crate :: rustc_complete :: Session ;
/* FP:lib.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0022
/* FP:lib.rs-0044 */ use crate :: rustc_complete :: config :: { CrateType , OutputFilenames , OutputType , RUST_CGU_EXT } ;
/* FP:lib.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0023
/* FP:lib.rs-0046 */ use crate :: rustc_complete :: cstore :: { self , CrateSource } ;
/* FP:lib.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0024
/* FP:lib.rs-0048 */ use crate :: rustc_complete :: lint :: builtin :: LINKER_MESSAGES ;
/* FP:lib.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_USE_0025
/* FP:lib.rs-0050 */ use crate :: rustc_complete :: Symbol ;
/* FP:lib.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MOD_0026
/* FP:lib.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MOD_0027
/* FP:lib.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MOD_0028
/* FP:lib.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MOD_0029
/* FP:lib.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MOD_0030
/* FP:lib.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MOD_0031
/* FP:lib.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MOD_0032
/* FP:lib.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MOD_0033
/* FP:lib.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MOD_0034
/* FP:lib.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MOD_0035
/* FP:lib.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MOD_0036
/* FP:lib.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MOD_0037
/* FP:lib.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MOD_0038
/* FP:lib.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MACRO_0039
/* FP:lib.rs-0078 */ rustc_fluent_macro :: fluent_messages ! { "../messages.ftl" }
/* FP:lib.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_STRUCT_0040
/* FP:lib.rs-0080 */ pub struct ModuleCodegen < M > { # [doc = " The name of the module. When the crate may be saved between"] # [doc = " compilations, incremental compilation requires that name be"] # [doc = " unique amongst **all** crates. Therefore, it should contain"] # [doc = " something unique to this crate (e.g., a module path) as well"] # [doc = " as the crate name and disambiguator."] # [doc = " We currently generate these names via CodegenUnit::build_cgu_name()."] pub name : String , pub module_llvm : M , pub kind : ModuleKind , # [doc = " Saving the ThinLTO buffer for embedding in the object file."] pub thin_lto_buffer : Option < Vec < u8 > > , }
/* FP:lib.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_IMPL_0041
/* FP:lib.rs-0082 */ impl < M > ModuleCodegen < M > { pub fn new_regular (name : impl Into < String > , module : M) -> Self { Self { name : name . into () , module_llvm : module , kind : ModuleKind :: Regular , thin_lto_buffer : None , } } pub fn new_allocator (name : impl Into < String > , module : M) -> Self { Self { name : name . into () , module_llvm : module , kind : ModuleKind :: Allocator , thin_lto_buffer : None , } } pub fn into_compiled_module (self , emit_obj : bool , emit_dwarf_obj : bool , emit_bc : bool , emit_asm : bool , emit_ir : bool , outputs : & OutputFilenames , invocation_temp : Option < & str > ,) -> CompiledModule { let object = emit_obj . then (| | outputs . temp_path_for_cgu (OutputType :: Object , & self . name , invocation_temp)) ; let dwarf_object = emit_dwarf_obj . then (| | outputs . temp_path_dwo_for_cgu (& self . name , invocation_temp)) ; let bytecode = emit_bc . then (| | outputs . temp_path_for_cgu (OutputType :: Bitcode , & self . name , invocation_temp)) ; let assembly = emit_asm . then (| | outputs . temp_path_for_cgu (OutputType :: Assembly , & self . name , invocation_temp)) ; let llvm_ir = emit_ir . then (| | { outputs . temp_path_for_cgu (OutputType :: LlvmAssembly , & self . name , invocation_temp) }) ; CompiledModule { name : self . name , kind : self . kind , object , dwarf_object , bytecode , assembly , llvm_ir , links_from_incr_cache : Vec :: new () , } } }
/* FP:lib.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_STRUCT_0042
/* FP:lib.rs-0084 */ # [derive (Debug , Encodable , Decodable)] pub struct CompiledModule { pub name : String , pub kind : ModuleKind , pub object : Option < PathBuf > , pub dwarf_object : Option < PathBuf > , pub bytecode : Option < PathBuf > , pub assembly : Option < PathBuf > , pub llvm_ir : Option < PathBuf > , pub links_from_incr_cache : Vec < PathBuf > , }
/* FP:lib.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_IMPL_0043
/* FP:lib.rs-0086 */ impl CompiledModule { # [doc = " Call `emit` function with every artifact type currently compiled"] pub fn for_each_output (& self , mut emit : impl FnMut (& Path , OutputType)) { if let Some (path) = self . object . as_deref () { emit (path , OutputType :: Object) ; } if let Some (path) = self . bytecode . as_deref () { emit (path , OutputType :: Bitcode) ; } if let Some (path) = self . llvm_ir . as_deref () { emit (path , OutputType :: LlvmAssembly) ; } if let Some (path) = self . assembly . as_deref () { emit (path , OutputType :: Assembly) ; } } }
/* FP:lib.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_STRUCT_0044
/* FP:lib.rs-0088 */ pub (crate) struct CachedModuleCodegen { pub name : String , pub source : WorkProduct , }
/* FP:lib.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_ENUM_0045
/* FP:lib.rs-0090 */ # [derive (Copy , Clone , Debug , PartialEq , Encodable , Decodable)] pub enum ModuleKind { Regular , Allocator , }
/* FP:lib.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_MACRO_0046
/* FP:lib.rs-0092 */ bitflags :: bitflags ! { # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct MemFlags : u8 { const VOLATILE = 1 << 0 ; const NONTEMPORAL = 1 << 1 ; const UNALIGNED = 1 << 2 ; } }
/* FP:lib.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_STRUCT_0047
/* FP:lib.rs-0094 */ # [derive (Clone , Debug , Encodable , Decodable , HashStable)] pub struct NativeLib { pub kind : NativeLibKind , pub name : Symbol , pub filename : Option < Symbol > , pub cfg : Option < CfgEntry > , pub verbatim : bool , pub dll_imports : Vec < cstore :: DllImport > , }
/* FP:lib.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_IMPL_0048
/* FP:lib.rs-0096 */ impl From < & cstore :: NativeLib > for NativeLib { fn from (lib : & cstore :: NativeLib) -> Self { NativeLib { kind : lib . kind , filename : lib . filename , name : lib . name , cfg : lib . cfg . clone () , verbatim : lib . verbatim . unwrap_or (false) , dll_imports : lib . dll_imports . clone () , } } }
/* FP:lib.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_STRUCT_0049
/* FP:lib.rs-0098 */ # [doc = " Misc info we load from metadata to persist beyond the tcx."] # [doc = ""] # [doc = " Note: though `CrateNum` is only meaningful within the same tcx, information within `CrateInfo`"] # [doc = " is self-contained. `CrateNum` can be viewed as a unique identifier within a `CrateInfo`, where"] # [doc = " `used_crate_source` contains all `CrateSource` of the dependents, and maintains a mapping from"] # [doc = " identifiers (`CrateNum`) to `CrateSource`. The other fields map `CrateNum` to the crate's own"] # [doc = " additional properties, so that effectively we can retrieve each dependent crate's `CrateSource`"] # [doc = " and the corresponding properties without referencing information outside of a `CrateInfo`."] # [derive (Debug , Encodable , Decodable)] pub struct CrateInfo { pub target_cpu : String , pub target_features : Vec < String > , pub crate_types : Vec < CrateType > , pub exported_symbols : UnordMap < CrateType , Vec < (String , SymbolExportKind) > > , pub linked_symbols : FxIndexMap < CrateType , Vec < (String , SymbolExportKind) > > , pub local_crate_name : Symbol , pub compiler_builtins : Option < CrateNum > , pub profiler_runtime : Option < CrateNum > , pub is_no_builtins : FxHashSet < CrateNum > , pub native_libraries : FxIndexMap < CrateNum , Vec < NativeLib > > , pub crate_name : UnordMap < CrateNum , Symbol > , pub used_libraries : Vec < NativeLib > , pub used_crate_source : UnordMap < CrateNum , Arc < CrateSource > > , pub used_crates : Vec < CrateNum > , pub dependency_formats : Arc < Dependencies > , pub windows_subsystem : Option < String > , pub natvis_debugger_visualizers : BTreeSet < DebuggerVisualizerFile > , pub lint_levels : CodegenLintLevels , pub metadata_symbol : String , }
/* FP:lib.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_STRUCT_0050
/* FP:lib.rs-0100 */ # [doc = " Target-specific options that get set in `cfg(...)`."] # [doc = ""] # [doc = " RUSTC_SPECIFIC_FEATURES should be skipped here, those are handled outside codegen."] pub struct TargetConfig { # [doc = " Options to be set in `cfg(target_features)`."] pub target_features : Vec < Symbol > , # [doc = " Options to be set in `cfg(target_features)`, but including unstable features."] pub unstable_target_features : Vec < Symbol > , # [doc = " Option for `cfg(target_has_reliable_f16)`, true if `f16` basic arithmetic works."] pub has_reliable_f16 : bool , # [doc = " Option for `cfg(target_has_reliable_f16_math)`, true if `f16` math calls work."] pub has_reliable_f16_math : bool , # [doc = " Option for `cfg(target_has_reliable_f128)`, true if `f128` basic arithmetic works."] pub has_reliable_f128 : bool , # [doc = " Option for `cfg(target_has_reliable_f128_math)`, true if `f128` math calls work."] pub has_reliable_f128_math : bool , }
/* FP:lib.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_STRUCT_0051
/* FP:lib.rs-0102 */ # [derive (Encodable , Decodable)] pub struct CodegenResults { pub modules : Vec < CompiledModule > , pub allocator_module : Option < CompiledModule > , pub crate_info : CrateInfo , }
/* FP:lib.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_ENUM_0052
/* FP:lib.rs-0104 */ pub enum CodegenErrors { WrongFileType , EmptyVersionNumber , EncodingVersionMismatch { version_array : String , rlink_version : u32 } , RustcVersionMismatch { rustc_version : String } , CorruptFile , }
/* FP:lib.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_FN_0053
/* FP:lib.rs-0106 */ pub fn provide (providers : & mut Providers) { crate :: back :: symbol_export :: provide (providers) ; crate :: base :: provide (providers) ; crate :: target_features :: provide (providers) ; crate :: codegen_attrs :: provide (providers) ; providers . queries . global_backend_features = | _tcx : TyCtxt < '_ > , () | vec ! [] ; }
/* FP:lib.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_FN_0054
/* FP:lib.rs-0108 */ # [doc = " Checks if the given filename ends with the `.rcgu.o` extension that `rustc`"] # [doc = " uses for the object files it generates."] pub fn looks_like_rust_object_file (filename : & str) -> bool { let path = Path :: new (filename) ; let ext = path . extension () . and_then (| s | s . to_str ()) ; if ext != Some (OutputType :: Object . extension ()) { return false ; } let ext2 = path . file_stem () . and_then (| s | Path :: new (s) . extension ()) . and_then (| s | s . to_str ()) ; ext2 == Some (RUST_CGU_EXT) }
/* FP:lib.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_CONST_0055
/* FP:lib.rs-0110 */ const RLINK_VERSION : u32 = 1 ;
/* FP:lib.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_CONST_0056
/* FP:lib.rs-0112 */ const RLINK_MAGIC : & [u8] = b"rustlink" ;
/* FP:lib.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_IMPL_0057
/* FP:lib.rs-0114 */ impl CodegenResults { pub fn serialize_rlink (sess : & Session , rlink_file : & Path , codegen_results : & CodegenResults , metadata : & EncodedMetadata , outputs : & OutputFilenames ,) -> Result < usize , io :: Error > { let mut encoder = FileEncoder :: new (rlink_file) ? ; encoder . emit_raw_bytes (RLINK_MAGIC) ; encoder . emit_raw_bytes (& RLINK_VERSION . to_be_bytes ()) ; encoder . emit_str (sess . cfg_version) ; Encodable :: encode (codegen_results , & mut encoder) ; Encodable :: encode (metadata , & mut encoder) ; Encodable :: encode (outputs , & mut encoder) ; encoder . finish () . map_err (| (_path , err) | err) } pub fn deserialize_rlink (sess : & Session , data : Vec < u8 > ,) -> Result < (Self , EncodedMetadata , OutputFilenames) , CodegenErrors > { if ! data . starts_with (RLINK_MAGIC) { return Err (CodegenErrors :: WrongFileType) ; } let data = & data [RLINK_MAGIC . len () ..] ; if data . len () < 4 { return Err (CodegenErrors :: EmptyVersionNumber) ; } let mut version_array : [u8 ; 4] = Default :: default () ; version_array . copy_from_slice (& data [.. 4]) ; if u32 :: from_be_bytes (version_array) != RLINK_VERSION { return Err (CodegenErrors :: EncodingVersionMismatch { version_array : String :: from_utf8_lossy (& version_array) . to_string () , rlink_version : RLINK_VERSION , }) ; } let Ok (mut decoder) = MemDecoder :: new (& data [4 ..] , 0) else { return Err (CodegenErrors :: CorruptFile) ; } ; let rustc_version = decoder . read_str () ; if rustc_version != sess . cfg_version { return Err (CodegenErrors :: RustcVersionMismatch { rustc_version : rustc_version . to_string () , }) ; } let codegen_results = CodegenResults :: decode (& mut decoder) ; let metadata = EncodedMetadata :: decode (& mut decoder) ; let outputs = OutputFilenames :: decode (& mut decoder) ; Ok ((codegen_results , metadata , outputs)) } }
/* FP:lib.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_STRUCT_0058
/* FP:lib.rs-0116 */ # [doc = " A list of lint levels used in codegen."] # [doc = ""] # [doc = " When using `-Z link-only`, we don't have access to the tcx and must work"] # [doc = " solely from the `.rlink` file. `Lint`s are defined too early to be encodeable."] # [doc = " Instead, encode exactly the information we need."] # [derive (Copy , Clone , Debug , Encodable , Decodable)] pub struct CodegenLintLevels { linker_messages : LevelAndSource , }
/* FP:lib.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_lib_IMPL_0059
/* FP:lib.rs-0118 */ impl CodegenLintLevels { pub fn from_tcx (tcx : TyCtxt < '_ >) -> Self { Self { linker_messages : tcx . lint_level_at_node (LINKER_MESSAGES , CRATE_HIR_ID) } } }