mkitem!{extern crate smallvec ;}
mkitem!{# [macro_use] extern crate tracing ;}
mkitem!{extern crate rustc_abi ;}
mkitem!{extern crate rustc_apfloat ;}
mkitem!{extern crate rustc_ast ;}
mkitem!{extern crate rustc_codegen_ssa ;}
mkitem!{extern crate rustc_data_structures ;}
mkitem!{extern crate rustc_errors ;}
mkitem!{extern crate rustc_fluent_macro ;}
mkitem!{extern crate rustc_fs_util ;}
mkitem!{extern crate rustc_hir ;}
mkitem!{extern crate rustc_index ;}
mkitem!{# [cfg (feature = "master")] extern crate rustc_interface ;}
mkitem!{extern crate rustc_macros ;}
mkitem!{extern crate rustc_middle ;}
mkitem!{extern crate rustc_session ;}
mkitem!{extern crate rustc_span ;}
mkitem!{extern crate rustc_symbol_mangling ;}
mkitem!{extern crate rustc_target ;}
mkitem!{extern crate rustc_type_ir ;}
mkitem!{# [allow (unused_extern_crates)] extern crate rustc_driver ;}
mkmod!{abi, { 
                getname!(abi);
                getsrc!(abi);
                getpath!(abi);
                get_deps!(abi);
                get_crates!(abi);
                mkinclude!(abi);
                 
            }}
mkmod!{allocator, { 
                getname!(allocator);
                getsrc!(allocator);
                getpath!(allocator);
                get_deps!(allocator);
                get_crates!(allocator);
                mkinclude!(allocator);
                 
            }}
mkmod!{asm, { 
                getname!(asm);
                getsrc!(asm);
                getpath!(asm);
                get_deps!(asm);
                get_crates!(asm);
                mkinclude!(asm);
                 
            }}
mkmod!{attributes, { 
                getname!(attributes);
                getsrc!(attributes);
                getpath!(attributes);
                get_deps!(attributes);
                get_crates!(attributes);
                mkinclude!(attributes);
                 
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
mkmod!{builder, { 
                getname!(builder);
                getsrc!(builder);
                getpath!(builder);
                get_deps!(builder);
                get_crates!(builder);
                mkinclude!(builder);
                 
            }}
mkmod!{callee, { 
                getname!(callee);
                getsrc!(callee);
                getpath!(callee);
                get_deps!(callee);
                get_crates!(callee);
                mkinclude!(callee);
                 
            }}
mkmod!{common, { 
                getname!(common);
                getsrc!(common);
                getpath!(common);
                get_deps!(common);
                get_crates!(common);
                mkinclude!(common);
                 
            }}
mkmod!{consts, { 
                getname!(consts);
                getsrc!(consts);
                getpath!(consts);
                get_deps!(consts);
                get_crates!(consts);
                mkinclude!(consts);
                 
            }}
mkmod!{context, { 
                getname!(context);
                getsrc!(context);
                getpath!(context);
                get_deps!(context);
                get_crates!(context);
                mkinclude!(context);
                 
            }}
mkmod!{coverageinfo, { 
                getname!(coverageinfo);
                getsrc!(coverageinfo);
                getpath!(coverageinfo);
                get_deps!(coverageinfo);
                get_crates!(coverageinfo);
                mkinclude!(coverageinfo);
                 
            }}
mkmod!{debuginfo, { 
                getname!(debuginfo);
                getsrc!(debuginfo);
                getpath!(debuginfo);
                get_deps!(debuginfo);
                get_crates!(debuginfo);
                mkinclude!(debuginfo);
                 
            }}
mkmod!{declare, { 
                getname!(declare);
                getsrc!(declare);
                getpath!(declare);
                get_deps!(declare);
                get_crates!(declare);
                mkinclude!(declare);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{gcc_util, { 
                getname!(gcc_util);
                getsrc!(gcc_util);
                getpath!(gcc_util);
                get_deps!(gcc_util);
                get_crates!(gcc_util);
                mkinclude!(gcc_util);
                 
            }}
mkmod!{int, { 
                getname!(int);
                getsrc!(int);
                getpath!(int);
                get_deps!(int);
                get_crates!(int);
                mkinclude!(int);
                 
            }}
mkmod!{intrinsic, { 
                getname!(intrinsic);
                getsrc!(intrinsic);
                getpath!(intrinsic);
                get_deps!(intrinsic);
                get_crates!(intrinsic);
                mkinclude!(intrinsic);
                 
            }}
mkmod!{mono_item, { 
                getname!(mono_item);
                getsrc!(mono_item);
                getpath!(mono_item);
                get_deps!(mono_item);
                get_crates!(mono_item);
                mkinclude!(mono_item);
                 
            }}
mkmod!{type_, { 
                getname!(type_);
                getsrc!(type_);
                getpath!(type_);
                get_deps!(type_);
                get_crates!(type_);
                mkinclude!(type_);
                 
            }}
mkmod!{type_of, { 
                getname!(type_of);
                getsrc!(type_of);
                getpath!(type_of);
                get_deps!(type_of);
                get_crates!(type_of);
                mkinclude!(type_of);
                 
            }}
mkuse!{use std :: any :: Any ;}
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: ops :: Deref ;}
mkuse!{use std :: path :: PathBuf ;}
mkuse!{# [cfg (not (feature = "master"))] use std :: sync :: atomic :: AtomicBool ;}
mkuse!{# [cfg (not (feature = "master"))] use std :: sync :: atomic :: Ordering ;}
mkuse!{use std :: sync :: { Arc , Mutex } ;}
mkuse!{use back :: lto :: { ThinBuffer , ThinData } ;}
mkuse!{use gccjit :: { CType , Context , OptimizationLevel } ;}
mkuse!{# [cfg (feature = "master")] use gccjit :: { TargetInfo , Version } ;}
mkuse!{use rustc_ast :: expand :: allocator :: AllocatorKind ;}
mkuse!{use rustc_codegen_ssa :: back :: lto :: { SerializedModule , ThinModule } ;}
mkuse!{use rustc_codegen_ssa :: back :: write :: { CodegenContext , FatLtoInput , ModuleConfig , TargetMachineFactoryFn , } ;}
mkuse!{use rustc_codegen_ssa :: base :: codegen_crate ;}
mkuse!{use rustc_codegen_ssa :: target_features :: cfg_target_feature ;}
mkuse!{use rustc_codegen_ssa :: traits :: { CodegenBackend , ExtraBackendMethods , WriteBackendMethods } ;}
mkuse!{use rustc_codegen_ssa :: { CodegenResults , CompiledModule , ModuleCodegen , TargetConfig } ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_data_structures :: sync :: IntoDynSyncSend ;}
mkuse!{use rustc_errors :: DiagCtxtHandle ;}
mkuse!{use rustc_middle :: dep_graph :: { WorkProduct , WorkProductId } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_middle :: util :: Providers ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: config :: { OptLevel , OutputFilenames } ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{use rustc_target :: spec :: RelocModel ;}
mkuse!{use tempfile :: TempDir ;}
mkuse!{use crate :: back :: lto :: ModuleBuffer ;}
mkuse!{use crate :: gcc_util :: target_cpu ;}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}
mkitem!{mkstruct!{pub struct PrintOnPanic < F : Fn () -> String > (pub F) ;}}
mkitem!{mkimpl!{impl < F : Fn () -> String > Drop for PrintOnPanic < F > { fn drop (& mut self) { if :: std :: thread :: panicking () { println ! ("{}" , (self . 0) ()) ; } } }}}
mkitem!{mkstruct!{# [cfg (not (feature = "master"))] # [derive (Debug)] pub struct TargetInfo { supports_128bit_integers : AtomicBool , }}}
mkitem!{mkimpl!{# [cfg (not (feature = "master"))] impl TargetInfo { fn cpu_supports (& self , _feature : & str) -> bool { false } fn supports_target_dependent_type (& self , typ : CType) -> bool { match typ { CType :: UInt128t | CType :: Int128t => { if self . supports_128bit_integers . load (Ordering :: SeqCst) { return true ; } } _ => () , } false } }}}
mkitem!{mkstruct!{# [derive (Clone)] pub struct LockedTargetInfo { info : Arc < Mutex < IntoDynSyncSend < TargetInfo > > > , }}}
mkitem!{mkimpl!{impl Debug for LockedTargetInfo { fn fmt (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . info . lock () . expect ("lock") . fmt (formatter) } }}}
mkitem!{mkimpl!{impl LockedTargetInfo { fn cpu_supports (& self , feature : & str) -> bool { self . info . lock () . expect ("lock") . cpu_supports (feature) } fn supports_target_dependent_type (& self , typ : CType) -> bool { self . info . lock () . expect ("lock") . supports_target_dependent_type (typ) } }}}
mkitem!{mkstruct!{# [derive (Clone)] pub struct GccCodegenBackend { target_info : LockedTargetInfo , }}}
mkitem!{mkimpl!{impl CodegenBackend for GccCodegenBackend { fn locale_resource (& self) -> & 'static str { crate :: DEFAULT_LOCALE_RESOURCE } fn init (& self , _sess : & Session) { # [cfg (feature = "master")] { let target_cpu = target_cpu (_sess) ; let context = Context :: default () ; if target_cpu != "generic" { context . add_command_line_option (format ! ("-march={}" , target_cpu)) ; } * * self . target_info . info . lock () . expect ("lock") = context . get_target_info () ; } # [cfg (feature = "master")] gccjit :: set_global_personality_function_name (b"rust_eh_personality\0") ; # [cfg (not (feature = "master"))] { let temp_dir = TempDir :: new () . expect ("cannot create temporary directory") ; let temp_file = temp_dir . keep () . join ("result.asm") ; let check_context = Context :: default () ; check_context . set_print_errors_to_stderr (false) ; let _int128_ty = check_context . new_c_type (CType :: UInt128t) ; check_context . compile_to_file (gccjit :: OutputKind :: Assembler , temp_file . to_str () . expect ("path to str") ,) ; self . target_info . info . lock () . expect ("lock") . supports_128bit_integers . store (check_context . get_last_error () == Ok (None) , Ordering :: SeqCst) ; } } fn provide (& self , providers : & mut Providers) { providers . global_backend_features = | tcx , () | gcc_util :: global_gcc_features (tcx . sess , true) } fn codegen_crate (& self , tcx : TyCtxt < '_ >) -> Box < dyn Any > { let target_cpu = target_cpu (tcx . sess) ; let res = codegen_crate (self . clone () , tcx , target_cpu . to_string ()) ; Box :: new (res) } fn join_codegen (& self , ongoing_codegen : Box < dyn Any > , sess : & Session , _outputs : & OutputFilenames ,) -> (CodegenResults , FxIndexMap < WorkProductId , WorkProduct >) { ongoing_codegen . downcast :: < rustc_codegen_ssa :: back :: write :: OngoingCodegen < GccCodegenBackend > > () . expect ("Expected GccCodegenBackend's OngoingCodegen, found Box<Any>") . join (sess) } fn target_config (& self , sess : & Session) -> TargetConfig { target_config (sess , & self . target_info) } }}}

macro_rules! new_context_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_context in module {}", module_path!());
    };
}

mkfn!{
    new_context_introspect!();
    fn new_context < 'gcc , 'tcx > (tcx : TyCtxt < 'tcx >) -> Context < 'gcc > { let context = Context :: default () ; if tcx . sess . target . arch == "x86" || tcx . sess . target . arch == "x86_64" { context . add_command_line_option ("-masm=intel") ; } # [cfg (feature = "master")] { context . set_special_chars_allowed_in_func_names ("$.*") ; let version = Version :: get () ; let version = format ! ("{}.{}.{}" , version . major , version . minor , version . patch) ; context . set_output_ident (& format ! ("rustc version {} with libgccjit {}" , rustc_interface :: util :: rustc_version_str () . unwrap_or ("unknown version") , version ,)) ; } context . add_command_line_option ("-fno-asynchronous-unwind-tables") ; context }
}
mkitem!{mkimpl!{impl ExtraBackendMethods for GccCodegenBackend { fn supports_parallel (& self) -> bool { false } fn codegen_allocator (& self , tcx : TyCtxt < '_ > , module_name : & str , kind : AllocatorKind , alloc_error_handler_kind : AllocatorKind ,) -> Self :: Module { let mut mods = GccContext { context : Arc :: new (SyncContext :: new (new_context (tcx))) , relocation_model : tcx . sess . relocation_model () , should_combine_object_files : false , temp_dir : None , } ; unsafe { allocator :: codegen (tcx , & mut mods , module_name , kind , alloc_error_handler_kind) ; } mods } fn compile_codegen_unit (& self , tcx : TyCtxt < '_ > , cgu_name : Symbol ,) -> (ModuleCodegen < Self :: Module > , u64) { base :: compile_codegen_unit (tcx , cgu_name , self . target_info . clone ()) } fn target_machine_factory (& self , _sess : & Session , _opt_level : OptLevel , _features : & [String] ,) -> TargetMachineFactoryFn < Self > { Arc :: new (| _ | Ok (())) } }}}
mkitem!{mkstruct!{pub struct GccContext { context : Arc < SyncContext > , # [doc = " This field is needed in order to be able to set the flag -fPIC when necessary when doing"] # [doc = " LTO."] relocation_model : RelocModel , should_combine_object_files : bool , temp_dir : Option < TempDir > , }}}
mkitem!{mkstruct!{struct SyncContext { context : Context < 'static > , }}}
mkitem!{mkimpl!{impl SyncContext { fn new (context : Context < 'static >) -> Self { Self { context } } }}}
mkitem!{mkimpl!{impl Deref for SyncContext { type Target = Context < 'static > ; fn deref (& self) -> & Self :: Target { & self . context } }}}
mkitem!{mkimpl!{unsafe impl Send for SyncContext { }}}
mkitem!{mkimpl!{unsafe impl Sync for SyncContext { }}}
mkitem!{mkimpl!{impl WriteBackendMethods for GccCodegenBackend { type Module = GccContext ; type TargetMachine = () ; type TargetMachineError = () ; type ModuleBuffer = ModuleBuffer ; type ThinData = ThinData ; type ThinBuffer = ThinBuffer ; fn run_and_optimize_fat_lto (cgcx : & CodegenContext < Self > , _exported_symbols_for_lto : & [String] , each_linked_rlib_for_lto : & [PathBuf] , modules : Vec < FatLtoInput < Self > > ,) -> ModuleCodegen < Self :: Module > { back :: lto :: run_fat (cgcx , each_linked_rlib_for_lto , modules) } fn run_thin_lto (cgcx : & CodegenContext < Self > , _exported_symbols_for_lto : & [String] , each_linked_rlib_for_lto : & [PathBuf] , modules : Vec < (String , Self :: ThinBuffer) > , cached_modules : Vec < (SerializedModule < Self :: ModuleBuffer > , WorkProduct) > ,) -> (Vec < ThinModule < Self > > , Vec < WorkProduct >) { back :: lto :: run_thin (cgcx , each_linked_rlib_for_lto , modules , cached_modules) } fn print_pass_timings (& self) { unimplemented ! () ; } fn print_statistics (& self) { unimplemented ! () } fn optimize (_cgcx : & CodegenContext < Self > , _dcx : DiagCtxtHandle < '_ > , module : & mut ModuleCodegen < Self :: Module > , config : & ModuleConfig ,) { module . module_llvm . context . set_optimization_level (to_gcc_opt_level (config . opt_level)) ; } fn optimize_thin (cgcx : & CodegenContext < Self > , thin : ThinModule < Self > ,) -> ModuleCodegen < Self :: Module > { back :: lto :: optimize_thin_module (thin , cgcx) } fn codegen (cgcx : & CodegenContext < Self > , module : ModuleCodegen < Self :: Module > , config : & ModuleConfig ,) -> CompiledModule { back :: write :: codegen (cgcx , module , config) } fn prepare_thin (module : ModuleCodegen < Self :: Module >) -> (String , Self :: ThinBuffer) { back :: lto :: prepare_thin (module) } fn serialize_module (_module : ModuleCodegen < Self :: Module >) -> (String , Self :: ModuleBuffer) { unimplemented ! () ; } }}}

macro_rules! __rustc_codegen_backend_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rustc_codegen_backend in module {}", module_path!());
    };
}

mkfn!{
    __rustc_codegen_backend_introspect!();
    # [doc = " This is the entrypoint for a hot plugged rustc_codegen_gccjit"] # [unsafe (no_mangle)] pub fn __rustc_codegen_backend () -> Box < dyn CodegenBackend > { # [cfg (feature = "master")] let info = { let context = Context :: default () ; Arc :: new (Mutex :: new (IntoDynSyncSend (context . get_target_info ()))) } ; # [cfg (not (feature = "master"))] let info = Arc :: new (Mutex :: new (IntoDynSyncSend (TargetInfo { supports_128bit_integers : AtomicBool :: new (false) , }))) ; Box :: new (GccCodegenBackend { target_info : LockedTargetInfo { info } }) }
}

macro_rules! to_gcc_opt_level_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_gcc_opt_level in module {}", module_path!());
    };
}

mkfn!{
    to_gcc_opt_level_introspect!();
    fn to_gcc_opt_level (optlevel : Option < OptLevel >) -> OptimizationLevel { match optlevel { None => OptimizationLevel :: None , Some (level) => match level { OptLevel :: No => OptimizationLevel :: None , OptLevel :: Less => OptimizationLevel :: Limited , OptLevel :: More => OptimizationLevel :: Standard , OptLevel :: Aggressive => OptimizationLevel :: Aggressive , OptLevel :: Size | OptLevel :: SizeMin => OptimizationLevel :: Limited , } , } }
}

macro_rules! target_config_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function target_config in module {}", module_path!());
    };
}

mkfn!{
    target_config_introspect!();
    # [doc = " Returns the features that should be set in `cfg(target_feature)`."] fn target_config (sess : & Session , target_info : & LockedTargetInfo) -> TargetConfig { let (unstable_target_features , target_features) = cfg_target_feature (sess , | feature | { if feature == "neon" { return false ; } target_info . cpu_supports (feature) }) ; let has_reliable_f16 = target_info . supports_target_dependent_type (CType :: Float16) ; let has_reliable_f128 = target_info . supports_target_dependent_type (CType :: Float128) ; TargetConfig { target_features , unstable_target_features , has_reliable_f16 , has_reliable_f16_math : has_reliable_f16 , has_reliable_f128 , has_reliable_f128_math : has_reliable_f128 , } }
}