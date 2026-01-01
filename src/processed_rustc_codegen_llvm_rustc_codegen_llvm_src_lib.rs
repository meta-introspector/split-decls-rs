/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0001
/* FP:lib.rs-0002 */ # [allow (internal_features)] # [doc (html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")] # [doc (rust_logo)] # [feature (assert_matches)] # [feature (extern_types)] # [feature (file_buffered)] # [feature (if_let_guard)] # [feature (impl_trait_in_assoc_type)] # [feature (iter_intersperse)] # [feature (rustdoc_internals)] # [feature (slice_as_array)] # [feature (try_blocks)] use std :: any :: Any ;
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0002
/* FP:lib.rs-0004 */ use std :: ffi :: CStr ;
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0003
/* FP:lib.rs-0006 */ use std :: mem :: ManuallyDrop ;
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0004
/* FP:lib.rs-0008 */ use std :: path :: PathBuf ;
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0005
/* FP:lib.rs-0010 */ use back :: owned_target_machine :: OwnedTargetMachine ;
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0006
/* FP:lib.rs-0012 */ use back :: write :: { create_informational_target_machine , create_target_machine } ;
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0007
/* FP:lib.rs-0014 */ use context :: SimpleCx ;
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0008
/* FP:lib.rs-0016 */ use errors :: ParseTargetMachineConfig ;
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0009
/* FP:lib.rs-0018 */ use llvm_util :: target_config ;
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0010
/* FP:lib.rs-0020 */ use crate :: rustc_complete :: expand :: allocator :: AllocatorKind ;
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0011
/* FP:lib.rs-0022 */ use crate :: rustc_codegen_ssa :: back :: lto :: { SerializedModule , ThinModule } ;
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0012
/* FP:lib.rs-0024 */ use crate :: rustc_codegen_ssa :: back :: write :: { CodegenContext , FatLtoInput , ModuleConfig , TargetMachineFactoryConfig , TargetMachineFactoryFn , } ;
/* FP:lib.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0013
/* FP:lib.rs-0026 */ use crate :: rustc_codegen_ssa :: traits :: * ;
/* FP:lib.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0014
/* FP:lib.rs-0028 */ use crate :: rustc_codegen_ssa :: { CodegenResults , CompiledModule , ModuleCodegen , TargetConfig } ;
/* FP:lib.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0015
/* FP:lib.rs-0030 */ use crate :: rustc_data_structures :: fx :: FxIndexMap ;
/* FP:lib.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0016
/* FP:lib.rs-0032 */ use crate :: rustc_complete :: DiagCtxtHandle ;
/* FP:lib.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0017
/* FP:lib.rs-0034 */ use crate :: rustc_metadata :: EncodedMetadata ;
/* FP:lib.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0018
/* FP:lib.rs-0036 */ use crate :: rustc_complete :: dep_graph :: { WorkProduct , WorkProductId } ;
/* FP:lib.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0019
/* FP:lib.rs-0038 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:lib.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0020
/* FP:lib.rs-0040 */ use crate :: rustc_complete :: util :: Providers ;
/* FP:lib.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0021
/* FP:lib.rs-0042 */ use crate :: rustc_complete :: Session ;
/* FP:lib.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0022
/* FP:lib.rs-0044 */ use crate :: rustc_complete :: config :: { OptLevel , OutputFilenames , PrintKind , PrintRequest } ;
/* FP:lib.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_USE_0023
/* FP:lib.rs-0046 */ use crate :: rustc_complete :: Symbol ;
/* FP:lib.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0024
/* FP:lib.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0025
/* FP:lib.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0026
/* FP:lib.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0027
/* FP:lib.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0028
/* FP:lib.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0029
/* FP:lib.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0030
/* FP:lib.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0031
/* FP:lib.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0032
/* FP:lib.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0033
/* FP:lib.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0034
/* FP:lib.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0035
/* FP:lib.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0036
/* FP:lib.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0037
/* FP:lib.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0038
/* FP:lib.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0039
/* FP:lib.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0040
/* FP:lib.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0041
/* FP:lib.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0042
/* FP:lib.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0043
/* FP:lib.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0044
/* FP:lib.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0045
/* FP:lib.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MOD_0046
/* FP:lib.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_MACRO_0047
/* FP:lib.rs-0094 */ rustc_fluent_macro :: fluent_messages ! { "../messages.ftl" }
/* FP:lib.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_STRUCT_0048
/* FP:lib.rs-0096 */ # [derive (Clone)] pub struct LlvmCodegenBackend (()) ;
/* FP:lib.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_STRUCT_0049
/* FP:lib.rs-0098 */ struct TimeTraceProfiler { enabled : bool , }
/* FP:lib.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_IMPL_0050
/* FP:lib.rs-0100 */ impl TimeTraceProfiler { fn new (enabled : bool) -> Self { if enabled { unsafe { llvm :: LLVMRustTimeTraceProfilerInitialize () } } TimeTraceProfiler { enabled } } }
/* FP:lib.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_IMPL_0051
/* FP:lib.rs-0102 */ impl Drop for TimeTraceProfiler { fn drop (& mut self) { if self . enabled { unsafe { llvm :: LLVMRustTimeTraceProfilerFinishThread () } } } }
/* FP:lib.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_IMPL_0052
/* FP:lib.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_IMPL_0053
/* FP:lib.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_IMPL_0054
/* FP:lib.rs-0108 */ impl LlvmCodegenBackend { pub fn new () -> Box < dyn CodegenBackend > { Box :: new (LlvmCodegenBackend (())) } }
/* FP:lib.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_IMPL_0055
/* FP:lib.rs-0110 */ impl CodegenBackend for LlvmCodegenBackend { fn locale_resource (& self) -> & 'static str { crate :: DEFAULT_LOCALE_RESOURCE } fn init (& self , sess : & Session) { llvm_util :: init (sess) ; } fn provide (& self , providers : & mut Providers) { providers . global_backend_features = | tcx , () | llvm_util :: global_llvm_features (tcx . sess , true , false) } fn print (& self , req : & PrintRequest , out : & mut String , sess : & Session) { use std :: fmt :: Write ; match req . kind { PrintKind :: RelocationModels => { writeln ! (out , "Available relocation models:") . unwrap () ; for name in & ["static" , "pic" , "pie" , "dynamic-no-pic" , "ropi" , "rwpi" , "ropi-rwpi" , "default" ,] { writeln ! (out , "    {name}") . unwrap () ; } writeln ! (out) . unwrap () ; } PrintKind :: CodeModels => { writeln ! (out , "Available code models:") . unwrap () ; for name in & ["tiny" , "small" , "kernel" , "medium" , "large"] { writeln ! (out , "    {name}") . unwrap () ; } writeln ! (out) . unwrap () ; } PrintKind :: TlsModels => { writeln ! (out , "Available TLS models:") . unwrap () ; for name in & ["global-dynamic" , "local-dynamic" , "initial-exec" , "local-exec" , "emulated"] { writeln ! (out , "    {name}") . unwrap () ; } writeln ! (out) . unwrap () ; } PrintKind :: StackProtectorStrategies => { writeln ! (out , r#"Available stack protector strategies:
/* FP:lib.rs-0111 */     all
/* FP:lib.rs-0112 */         Generate stack canaries in all functions.
/* FP:lib.rs-0113 */ 
/* FP:lib.rs-0114 */     strong
/* FP:lib.rs-0115 */         Generate stack canaries in a function if it either:
/* FP:lib.rs-0116 */         - has a local variable of `[T; N]` type, regardless of `T` and `N`
/* FP:lib.rs-0117 */         - takes the address of a local variable.
/* FP:lib.rs-0118 */ 
/* FP:lib.rs-0119 */           (Note that a local variable being borrowed is not equivalent to its
/* FP:lib.rs-0120 */           address being taken: e.g. some borrows may be removed by optimization,
/* FP:lib.rs-0121 */           while by-value argument passing may be implemented with reference to a
/* FP:lib.rs-0122 */           local stack variable in the ABI.)
/* FP:lib.rs-0123 */ 
/* FP:lib.rs-0124 */     basic
/* FP:lib.rs-0125 */         Generate stack canaries in functions with local variables of `[T; N]`
/* FP:lib.rs-0126 */         type, where `T` is byte-sized and `N` >= 8.
/* FP:lib.rs-0127 */ 
/* FP:lib.rs-0128 */     none
/* FP:lib.rs-0129 */         Do not generate stack canaries.
/* FP:lib.rs-0130 */ "#) . unwrap () ; } _other => llvm_util :: print (req , out , sess) , } } fn print_passes (& self) { llvm_util :: print_passes () ; } fn print_version (& self) { llvm_util :: print_version () ; } fn target_config (& self , sess : & Session) -> TargetConfig { target_config (sess) } fn codegen_crate < 'tcx > (& self , tcx : TyCtxt < 'tcx >) -> Box < dyn Any > { Box :: new (crate :: rustc_codegen_ssa :: base :: codegen_crate (LlvmCodegenBackend (()) , tcx , crate :: llvm_util :: target_cpu (tcx . sess) . to_string () ,)) } fn join_codegen (& self , ongoing_codegen : Box < dyn Any > , sess : & Session , outputs : & OutputFilenames ,) -> (CodegenResults , FxIndexMap < WorkProductId , WorkProduct >) { let (codegen_results , work_products) = ongoing_codegen . downcast :: < crate :: rustc_codegen_ssa :: back :: write :: OngoingCodegen < LlvmCodegenBackend > > () . expect ("Expected LlvmCodegenBackend's OngoingCodegen, found Box<Any>") . join (sess) ; if sess . opts . unstable_opts . llvm_time_trace { sess . time ("llvm_dump_timing_file" , | | { let file_name = outputs . with_extension ("llvm_timings.json") ; llvm_util :: time_trace_profiler_finish (& file_name) ; }) ; } (codegen_results , work_products) } fn link (& self , sess : & Session , codegen_results : CodegenResults , metadata : EncodedMetadata , outputs : & OutputFilenames ,) { use crate :: rustc_codegen_ssa :: back :: link :: link_binary ; use crate :: back :: archive :: LlvmArchiveBuilderBuilder ; link_binary (sess , & LlvmArchiveBuilderBuilder , codegen_results , metadata , outputs) ; } }
/* FP:lib.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_STRUCT_0056
/* FP:lib.rs-0132 */ pub struct ModuleLlvm { llcx : & 'static mut llvm :: Context , llmod_raw : * const llvm :: Module , tm : ManuallyDrop < OwnedTargetMachine > , }
/* FP:lib.rs-0133 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_IMPL_0057
/* FP:lib.rs-0134 */ unsafe impl Send for ModuleLlvm { }
/* FP:lib.rs-0135 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_IMPL_0058
/* FP:lib.rs-0136 */ unsafe impl Sync for ModuleLlvm { }
/* FP:lib.rs-0137 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_IMPL_0059
/* FP:lib.rs-0139 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_lib_IMPL_0060
/* FP:lib.rs-0140 */ impl Drop for ModuleLlvm { fn drop (& mut self) { unsafe { ManuallyDrop :: drop (& mut self . tm) ; llvm :: LLVMContextDispose (& mut * (self . llcx as * mut _)) ; } } }