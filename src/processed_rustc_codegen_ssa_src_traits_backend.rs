/* FP:backend.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0001
/* FP:backend.rs-0002 */ use std :: any :: Any ;
/* FP:backend.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0002
/* FP:backend.rs-0004 */ use std :: hash :: Hash ;
/* FP:backend.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0003
/* FP:backend.rs-0006 */ use crate :: rustc_complete :: expand :: allocator :: AllocatorKind ;
/* FP:backend.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0004
/* FP:backend.rs-0008 */ use crate :: rustc_data_structures :: fx :: FxIndexMap ;
/* FP:backend.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0005
/* FP:backend.rs-0010 */ use crate :: rustc_data_structures :: sync :: { DynSend , DynSync } ;
/* FP:backend.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0006
/* FP:backend.rs-0012 */ use crate :: rustc_metadata :: EncodedMetadata ;
/* FP:backend.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0007
/* FP:backend.rs-0014 */ use crate :: rustc_metadata :: creader :: MetadataLoaderDyn ;
/* FP:backend.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0008
/* FP:backend.rs-0016 */ use crate :: rustc_complete :: dep_graph :: { WorkProduct , WorkProductId } ;
/* FP:backend.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0009
/* FP:backend.rs-0018 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:backend.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0010
/* FP:backend.rs-0020 */ use crate :: rustc_complete :: util :: Providers ;
/* FP:backend.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0011
/* FP:backend.rs-0022 */ use crate :: rustc_complete :: Session ;
/* FP:backend.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0012
/* FP:backend.rs-0024 */ use crate :: rustc_complete :: config :: { self , OutputFilenames , PrintRequest } ;
/* FP:backend.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0013
/* FP:backend.rs-0026 */ use crate :: rustc_complete :: Symbol ;
/* FP:backend.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0014
/* FP:backend.rs-0028 */ use super :: CodegenObject ;
/* FP:backend.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0015
/* FP:backend.rs-0030 */ use super :: write :: WriteBackendMethods ;
/* FP:backend.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0016
/* FP:backend.rs-0032 */ use crate :: back :: archive :: ArArchiveBuilderBuilder ;
/* FP:backend.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0017
/* FP:backend.rs-0034 */ use crate :: back :: link :: link_binary ;
/* FP:backend.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0018
/* FP:backend.rs-0036 */ use crate :: back :: write :: TargetMachineFactoryFn ;
/* FP:backend.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_USE_0019
/* FP:backend.rs-0038 */ use crate :: { CodegenResults , ModuleCodegen , TargetConfig } ;
/* FP:backend.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_TRAIT_0020
/* FP:backend.rs-0040 */ pub trait BackendTypes { type Value : CodegenObject + PartialEq ; type Metadata : CodegenObject ; type Function : CodegenObject ; type BasicBlock : Copy ; type Type : CodegenObject + PartialEq ; type Funclet ; type DIScope : Copy + Hash + PartialEq + Eq ; type DILocation : Copy ; type DIVariable : Copy ; }
/* FP:backend.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_TRAIT_0021
/* FP:backend.rs-0042 */ pub trait CodegenBackend { # [doc = " Locale resources for diagnostic messages - a string the content of the Fluent resource."] # [doc = " Called before `init` so that all other functions are able to emit translatable diagnostics."] fn locale_resource (& self) -> & 'static str ; fn init (& self , _sess : & Session) { } fn print (& self , _req : & PrintRequest , _out : & mut String , _sess : & Session) { } # [doc = " Collect target-specific options that should be set in `cfg(...)`, including"] # [doc = " `target_feature` and support for unstable float types."] fn target_config (& self , _sess : & Session) -> TargetConfig { TargetConfig { target_features : vec ! [] , unstable_target_features : vec ! [] , has_reliable_f16 : true , has_reliable_f16_math : true , has_reliable_f128 : true , has_reliable_f128_math : true , } } fn print_passes (& self) { } fn print_version (& self) { } # [doc = " The metadata loader used to load rlib and dylib metadata."] # [doc = ""] # [doc = " Alternative codegen backends may want to use different rlib or dylib formats than the"] # [doc = " default native static archives and dynamic libraries."] fn metadata_loader (& self) -> Box < MetadataLoaderDyn > { Box :: new (crate :: back :: metadata :: DefaultMetadataLoader) } fn provide (& self , _providers : & mut Providers) { } fn codegen_crate < 'tcx > (& self , tcx : TyCtxt < 'tcx >) -> Box < dyn Any > ; # [doc = " This is called on the returned `Box<dyn Any>` from [`codegen_crate`](Self::codegen_crate)"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics when the passed `Box<dyn Any>` was not returned by [`codegen_crate`](Self::codegen_crate)."] fn join_codegen (& self , ongoing_codegen : Box < dyn Any > , sess : & Session , outputs : & OutputFilenames ,) -> (CodegenResults , FxIndexMap < WorkProductId , WorkProduct >) ; # [doc = " This is called on the returned [`CodegenResults`] from [`join_codegen`](Self::join_codegen)."] fn link (& self , sess : & Session , codegen_results : CodegenResults , metadata : EncodedMetadata , outputs : & OutputFilenames ,) { link_binary (sess , & ArArchiveBuilderBuilder , codegen_results , metadata , outputs) ; } }
/* FP:backend.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_backend_TRAIT_0022
/* FP:backend.rs-0044 */ pub trait ExtraBackendMethods : CodegenBackend + WriteBackendMethods + Sized + Send + Sync + DynSend + DynSync { fn codegen_allocator < 'tcx > (& self , tcx : TyCtxt < 'tcx > , module_name : & str , kind : AllocatorKind , alloc_error_handler_kind : AllocatorKind ,) -> Self :: Module ; # [doc = " This generates the codegen unit and returns it along with"] # [doc = " a `u64` giving an estimate of the unit's processing cost."] fn compile_codegen_unit (& self , tcx : TyCtxt < '_ > , cgu_name : Symbol ,) -> (ModuleCodegen < Self :: Module > , u64) ; fn target_machine_factory (& self , sess : & Session , opt_level : config :: OptLevel , target_features : & [String] ,) -> TargetMachineFactoryFn < Self > ; fn spawn_named_thread < F , T > (_time_trace : bool , name : String , f : F ,) -> std :: io :: Result < std :: thread :: JoinHandle < T > > where F : FnOnce () -> T , F : Send + 'static , T : Send + 'static , { std :: thread :: Builder :: new () . name (name) . spawn (f) } # [doc = " Returns `true` if this backend can be safely called from multiple threads."] # [doc = ""] # [doc = " Defaults to `true`."] fn supports_parallel (& self) -> bool { true } }