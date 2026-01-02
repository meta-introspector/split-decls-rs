mkuse!{use std :: collections :: BTreeSet ;}
mkuse!{use std :: io ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashSet , FxIndexMap } ;}
mkuse!{use rustc_data_structures :: unord :: UnordMap ;}
mkuse!{use rustc_hir :: CRATE_HIR_ID ;}
mkuse!{use rustc_hir :: attrs :: { CfgEntry , NativeLibKind } ;}
mkuse!{use rustc_hir :: def_id :: CrateNum ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable } ;}
mkuse!{use rustc_metadata :: EncodedMetadata ;}
mkuse!{use rustc_middle :: dep_graph :: WorkProduct ;}
mkuse!{use rustc_middle :: lint :: LevelAndSource ;}
mkuse!{use rustc_middle :: middle :: debugger_visualizer :: DebuggerVisualizerFile ;}
mkuse!{use rustc_middle :: middle :: dependency_format :: Dependencies ;}
mkuse!{use rustc_middle :: middle :: exported_symbols :: SymbolExportKind ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_middle :: util :: Providers ;}
mkuse!{use rustc_serialize :: opaque :: { FileEncoder , MemDecoder } ;}
mkuse!{use rustc_serialize :: { Decodable , Decoder , Encodable , Encoder } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: config :: { CrateType , OutputFilenames , OutputType , RUST_CGU_EXT } ;}
mkuse!{use rustc_session :: cstore :: { self , CrateSource } ;}
mkuse!{use rustc_session :: lint :: builtin :: LINKER_MESSAGES ;}
mkuse!{use rustc_span :: Symbol ;}
mkmod!{assert_module_sources, { 
                getname!(assert_module_sources);
                getsrc!(assert_module_sources);
                getpath!(assert_module_sources);
                get_deps!(assert_module_sources);
                get_crates!(assert_module_sources);
                mkinclude!(assert_module_sources);
                 
            }}
mkmod!{back, { 
                getname!(back);
                getsrc!(back);
                getpath!(back);
                get_deps!(back);
                get_crates!(back);
                mkinclude!(back);
                 
            }}
mkmod!{base, { 
                getname!(base);
                getsrc!(base);
                getpath!(base);
                get_deps!(base);
                get_crates!(base);
                mkinclude!(base);
                 
            }}
mkmod!{codegen_attrs, { 
                getname!(codegen_attrs);
                getsrc!(codegen_attrs);
                getpath!(codegen_attrs);
                get_deps!(codegen_attrs);
                get_crates!(codegen_attrs);
                mkinclude!(codegen_attrs);
                 
            }}
mkmod!{common, { 
                getname!(common);
                getsrc!(common);
                getpath!(common);
                get_deps!(common);
                get_crates!(common);
                mkinclude!(common);
                 
            }}
mkmod!{debuginfo, { 
                getname!(debuginfo);
                getsrc!(debuginfo);
                getpath!(debuginfo);
                get_deps!(debuginfo);
                get_crates!(debuginfo);
                mkinclude!(debuginfo);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{meth, { 
                getname!(meth);
                getsrc!(meth);
                getpath!(meth);
                get_deps!(meth);
                get_crates!(meth);
                mkinclude!(meth);
                 
            }}
mkmod!{mir, { 
                getname!(mir);
                getsrc!(mir);
                getpath!(mir);
                get_deps!(mir);
                get_crates!(mir);
                mkinclude!(mir);
                 
            }}
mkmod!{mono_item, { 
                getname!(mono_item);
                getsrc!(mono_item);
                getpath!(mono_item);
                get_deps!(mono_item);
                get_crates!(mono_item);
                mkinclude!(mono_item);
                 
            }}
mkmod!{size_of_val, { 
                getname!(size_of_val);
                getsrc!(size_of_val);
                getpath!(size_of_val);
                get_deps!(size_of_val);
                get_crates!(size_of_val);
                mkinclude!(size_of_val);
                 
            }}
mkmod!{target_features, { 
                getname!(target_features);
                getsrc!(target_features);
                getpath!(target_features);
                get_deps!(target_features);
                get_crates!(target_features);
                mkinclude!(target_features);
                 
            }}
mkmod!{traits, { 
                getname!(traits);
                getsrc!(traits);
                getpath!(traits);
                get_deps!(traits);
                get_crates!(traits);
                mkinclude!(traits);
                 
            }}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}
mkitem!{mkstruct!{pub struct ModuleCodegen < M > { # [doc = " The name of the module. When the crate may be saved between"] # [doc = " compilations, incremental compilation requires that name be"] # [doc = " unique amongst **all** crates. Therefore, it should contain"] # [doc = " something unique to this crate (e.g., a module path) as well"] # [doc = " as the crate name and disambiguator."] # [doc = " We currently generate these names via CodegenUnit::build_cgu_name()."] pub name : String , pub module_llvm : M , pub kind : ModuleKind , # [doc = " Saving the ThinLTO buffer for embedding in the object file."] pub thin_lto_buffer : Option < Vec < u8 > > , }}}
mkitem!{mkimpl!{impl < M > ModuleCodegen < M > { pub fn new_regular (name : impl Into < String > , module : M) -> Self { Self { name : name . into () , module_llvm : module , kind : ModuleKind :: Regular , thin_lto_buffer : None , } } pub fn new_allocator (name : impl Into < String > , module : M) -> Self { Self { name : name . into () , module_llvm : module , kind : ModuleKind :: Allocator , thin_lto_buffer : None , } } pub fn into_compiled_module (self , emit_obj : bool , emit_dwarf_obj : bool , emit_bc : bool , emit_asm : bool , emit_ir : bool , outputs : & OutputFilenames , invocation_temp : Option < & str > ,) -> CompiledModule { let object = emit_obj . then (| | outputs . temp_path_for_cgu (OutputType :: Object , & self . name , invocation_temp)) ; let dwarf_object = emit_dwarf_obj . then (| | outputs . temp_path_dwo_for_cgu (& self . name , invocation_temp)) ; let bytecode = emit_bc . then (| | outputs . temp_path_for_cgu (OutputType :: Bitcode , & self . name , invocation_temp)) ; let assembly = emit_asm . then (| | outputs . temp_path_for_cgu (OutputType :: Assembly , & self . name , invocation_temp)) ; let llvm_ir = emit_ir . then (| | { outputs . temp_path_for_cgu (OutputType :: LlvmAssembly , & self . name , invocation_temp) }) ; CompiledModule { name : self . name , kind : self . kind , object , dwarf_object , bytecode , assembly , llvm_ir , links_from_incr_cache : Vec :: new () , } } }}}
mkitem!{mkstruct!{# [derive (Debug , Encodable , Decodable)] pub struct CompiledModule { pub name : String , pub kind : ModuleKind , pub object : Option < PathBuf > , pub dwarf_object : Option < PathBuf > , pub bytecode : Option < PathBuf > , pub assembly : Option < PathBuf > , pub llvm_ir : Option < PathBuf > , pub links_from_incr_cache : Vec < PathBuf > , }}}
mkitem!{mkimpl!{impl CompiledModule { # [doc = " Call `emit` function with every artifact type currently compiled"] pub fn for_each_output (& self , mut emit : impl FnMut (& Path , OutputType)) { if let Some (path) = self . object . as_deref () { emit (path , OutputType :: Object) ; } if let Some (path) = self . bytecode . as_deref () { emit (path , OutputType :: Bitcode) ; } if let Some (path) = self . llvm_ir . as_deref () { emit (path , OutputType :: LlvmAssembly) ; } if let Some (path) = self . assembly . as_deref () { emit (path , OutputType :: Assembly) ; } } }}}
mkitem!{mkstruct!{pub (crate) struct CachedModuleCodegen { pub name : String , pub source : WorkProduct , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug , PartialEq , Encodable , Decodable)] pub enum ModuleKind { Regular , Allocator , }}}
mkitem!{bitflags :: bitflags ! { # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct MemFlags : u8 { const VOLATILE = 1 << 0 ; const NONTEMPORAL = 1 << 1 ; const UNALIGNED = 1 << 2 ; } }}
mkitem!{mkstruct!{# [derive (Clone , Debug , Encodable , Decodable , HashStable)] pub struct NativeLib { pub kind : NativeLibKind , pub name : Symbol , pub filename : Option < Symbol > , pub cfg : Option < CfgEntry > , pub verbatim : bool , pub dll_imports : Vec < cstore :: DllImport > , }}}
mkitem!{mkimpl!{impl From < & cstore :: NativeLib > for NativeLib { fn from (lib : & cstore :: NativeLib) -> Self { NativeLib { kind : lib . kind , filename : lib . filename , name : lib . name , cfg : lib . cfg . clone () , verbatim : lib . verbatim . unwrap_or (false) , dll_imports : lib . dll_imports . clone () , } } }}}
mkitem!{mkstruct!{# [doc = " Misc info we load from metadata to persist beyond the tcx."] # [doc = ""] # [doc = " Note: though `CrateNum` is only meaningful within the same tcx, information within `CrateInfo`"] # [doc = " is self-contained. `CrateNum` can be viewed as a unique identifier within a `CrateInfo`, where"] # [doc = " `used_crate_source` contains all `CrateSource` of the dependents, and maintains a mapping from"] # [doc = " identifiers (`CrateNum`) to `CrateSource`. The other fields map `CrateNum` to the crate's own"] # [doc = " additional properties, so that effectively we can retrieve each dependent crate's `CrateSource`"] # [doc = " and the corresponding properties without referencing information outside of a `CrateInfo`."] # [derive (Debug , Encodable , Decodable)] pub struct CrateInfo { pub target_cpu : String , pub target_features : Vec < String > , pub crate_types : Vec < CrateType > , pub exported_symbols : UnordMap < CrateType , Vec < (String , SymbolExportKind) > > , pub linked_symbols : FxIndexMap < CrateType , Vec < (String , SymbolExportKind) > > , pub local_crate_name : Symbol , pub compiler_builtins : Option < CrateNum > , pub profiler_runtime : Option < CrateNum > , pub is_no_builtins : FxHashSet < CrateNum > , pub native_libraries : FxIndexMap < CrateNum , Vec < NativeLib > > , pub crate_name : UnordMap < CrateNum , Symbol > , pub used_libraries : Vec < NativeLib > , pub used_crate_source : UnordMap < CrateNum , Arc < CrateSource > > , pub used_crates : Vec < CrateNum > , pub dependency_formats : Arc < Dependencies > , pub windows_subsystem : Option < String > , pub natvis_debugger_visualizers : BTreeSet < DebuggerVisualizerFile > , pub lint_levels : CodegenLintLevels , pub metadata_symbol : String , }}}
mkitem!{mkstruct!{# [doc = " Target-specific options that get set in `cfg(...)`."] # [doc = ""] # [doc = " RUSTC_SPECIFIC_FEATURES should be skipped here, those are handled outside codegen."] pub struct TargetConfig { # [doc = " Options to be set in `cfg(target_features)`."] pub target_features : Vec < Symbol > , # [doc = " Options to be set in `cfg(target_features)`, but including unstable features."] pub unstable_target_features : Vec < Symbol > , # [doc = " Option for `cfg(target_has_reliable_f16)`, true if `f16` basic arithmetic works."] pub has_reliable_f16 : bool , # [doc = " Option for `cfg(target_has_reliable_f16_math)`, true if `f16` math calls work."] pub has_reliable_f16_math : bool , # [doc = " Option for `cfg(target_has_reliable_f128)`, true if `f128` basic arithmetic works."] pub has_reliable_f128 : bool , # [doc = " Option for `cfg(target_has_reliable_f128_math)`, true if `f128` math calls work."] pub has_reliable_f128_math : bool , }}}
mkitem!{mkstruct!{# [derive (Encodable , Decodable)] pub struct CodegenResults { pub modules : Vec < CompiledModule > , pub allocator_module : Option < CompiledModule > , pub crate_info : CrateInfo , }}}
mkitem!{mkenum!{pub enum CodegenErrors { WrongFileType , EmptyVersionNumber , EncodingVersionMismatch { version_array : String , rlink_version : u32 } , RustcVersionMismatch { rustc_version : String } , CorruptFile , }}}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (providers : & mut Providers) { crate :: back :: symbol_export :: provide (providers) ; crate :: base :: provide (providers) ; crate :: target_features :: provide (providers) ; crate :: codegen_attrs :: provide (providers) ; providers . queries . global_backend_features = | _tcx : TyCtxt < '_ > , () | vec ! [] ; }
}

macro_rules! looks_like_rust_object_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function looks_like_rust_object_file in module {}", module_path!());
    };
}

mkfn!{
    looks_like_rust_object_file_introspect!();
    # [doc = " Checks if the given filename ends with the `.rcgu.o` extension that `rustc`"] # [doc = " uses for the object files it generates."] pub fn looks_like_rust_object_file (filename : & str) -> bool { let path = Path :: new (filename) ; let ext = path . extension () . and_then (| s | s . to_str ()) ; if ext != Some (OutputType :: Object . extension ()) { return false ; } let ext2 = path . file_stem () . and_then (| s | Path :: new (s) . extension ()) . and_then (| s | s . to_str ()) ; ext2 == Some (RUST_CGU_EXT) }
}
mkitem!{const RLINK_VERSION : u32 = 1 ;}
mkitem!{const RLINK_MAGIC : & [u8] = b"rustlink" ;}
mkitem!{mkimpl!{impl CodegenResults { pub fn serialize_rlink (sess : & Session , rlink_file : & Path , codegen_results : & CodegenResults , metadata : & EncodedMetadata , outputs : & OutputFilenames ,) -> Result < usize , io :: Error > { let mut encoder = FileEncoder :: new (rlink_file) ? ; encoder . emit_raw_bytes (RLINK_MAGIC) ; encoder . emit_raw_bytes (& RLINK_VERSION . to_be_bytes ()) ; encoder . emit_str (sess . cfg_version) ; Encodable :: encode (codegen_results , & mut encoder) ; Encodable :: encode (metadata , & mut encoder) ; Encodable :: encode (outputs , & mut encoder) ; encoder . finish () . map_err (| (_path , err) | err) } pub fn deserialize_rlink (sess : & Session , data : Vec < u8 > ,) -> Result < (Self , EncodedMetadata , OutputFilenames) , CodegenErrors > { if ! data . starts_with (RLINK_MAGIC) { return Err (CodegenErrors :: WrongFileType) ; } let data = & data [RLINK_MAGIC . len () ..] ; if data . len () < 4 { return Err (CodegenErrors :: EmptyVersionNumber) ; } let mut version_array : [u8 ; 4] = Default :: default () ; version_array . copy_from_slice (& data [.. 4]) ; if u32 :: from_be_bytes (version_array) != RLINK_VERSION { return Err (CodegenErrors :: EncodingVersionMismatch { version_array : String :: from_utf8_lossy (& version_array) . to_string () , rlink_version : RLINK_VERSION , }) ; } let Ok (mut decoder) = MemDecoder :: new (& data [4 ..] , 0) else { return Err (CodegenErrors :: CorruptFile) ; } ; let rustc_version = decoder . read_str () ; if rustc_version != sess . cfg_version { return Err (CodegenErrors :: RustcVersionMismatch { rustc_version : rustc_version . to_string () , }) ; } let codegen_results = CodegenResults :: decode (& mut decoder) ; let metadata = EncodedMetadata :: decode (& mut decoder) ; let outputs = OutputFilenames :: decode (& mut decoder) ; Ok ((codegen_results , metadata , outputs)) } }}}
mkitem!{mkstruct!{# [doc = " A list of lint levels used in codegen."] # [doc = ""] # [doc = " When using `-Z link-only`, we don't have access to the tcx and must work"] # [doc = " solely from the `.rlink` file. `Lint`s are defined too early to be encodeable."] # [doc = " Instead, encode exactly the information we need."] # [derive (Copy , Clone , Debug , Encodable , Decodable)] pub struct CodegenLintLevels { linker_messages : LevelAndSource , }}}
mkitem!{mkimpl!{impl CodegenLintLevels { pub fn from_tcx (tcx : TyCtxt < '_ >) -> Self { Self { linker_messages : tcx . lint_level_at_node (LINKER_MESSAGES , CRATE_HIR_ID) } } }}}