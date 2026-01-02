mkitem!{# [macro_use] extern crate rustc_middle ;}
mkitem!{extern crate rustc_abi ;}
mkitem!{extern crate rustc_ast ;}
mkitem!{extern crate rustc_codegen_ssa ;}
mkitem!{extern crate rustc_data_structures ;}
mkitem!{extern crate rustc_errors ;}
mkitem!{extern crate rustc_fs_util ;}
mkitem!{extern crate rustc_hir ;}
mkitem!{extern crate rustc_incremental ;}
mkitem!{extern crate rustc_index ;}
mkitem!{extern crate rustc_metadata ;}
mkitem!{extern crate rustc_session ;}
mkitem!{extern crate rustc_span ;}
mkitem!{extern crate rustc_symbol_mangling ;}
mkitem!{extern crate rustc_target ;}
mkitem!{# [macro_use] extern crate tracing ;}
mkitem!{# [allow (unused_extern_crates)] extern crate rustc_driver ;}
mkuse!{use std :: any :: Any ;}
mkuse!{use std :: env ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use cranelift_codegen :: isa :: TargetIsa ;}
mkuse!{use cranelift_codegen :: settings :: { self , Configurable } ;}
mkuse!{use rustc_codegen_ssa :: traits :: CodegenBackend ;}
mkuse!{use rustc_codegen_ssa :: { CodegenResults , TargetConfig } ;}
mkuse!{use rustc_middle :: dep_graph :: { WorkProduct , WorkProductId } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: config :: OutputFilenames ;}
mkuse!{use rustc_span :: { Symbol , sym } ;}
mkuse!{pub use crate :: config :: * ;}
mkuse!{use crate :: prelude :: * ;}
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
mkmod!{analyze, { 
                getname!(analyze);
                getsrc!(analyze);
                getpath!(analyze);
                get_deps!(analyze);
                get_crates!(analyze);
                mkinclude!(analyze);
                 
            }}
mkmod!{base, { 
                getname!(base);
                getsrc!(base);
                getpath!(base);
                get_deps!(base);
                get_crates!(base);
                mkinclude!(base);
                 
            }}
mkmod!{cast, { 
                getname!(cast);
                getsrc!(cast);
                getpath!(cast);
                get_deps!(cast);
                get_crates!(cast);
                mkinclude!(cast);
                 
            }}
mkmod!{codegen_f16_f128, { 
                getname!(codegen_f16_f128);
                getsrc!(codegen_f16_f128);
                getpath!(codegen_f16_f128);
                get_deps!(codegen_f16_f128);
                get_crates!(codegen_f16_f128);
                mkinclude!(codegen_f16_f128);
                 
            }}
mkmod!{codegen_i128, { 
                getname!(codegen_i128);
                getsrc!(codegen_i128);
                getpath!(codegen_i128);
                get_deps!(codegen_i128);
                get_crates!(codegen_i128);
                mkinclude!(codegen_i128);
                 
            }}
mkmod!{common, { 
                getname!(common);
                getsrc!(common);
                getpath!(common);
                get_deps!(common);
                get_crates!(common);
                mkinclude!(common);
                 
            }}
mkmod!{compiler_builtins, { 
                getname!(compiler_builtins);
                getsrc!(compiler_builtins);
                getpath!(compiler_builtins);
                get_deps!(compiler_builtins);
                get_crates!(compiler_builtins);
                mkinclude!(compiler_builtins);
                 
            }}
mkmod!{concurrency_limiter, { 
                getname!(concurrency_limiter);
                getsrc!(concurrency_limiter);
                getpath!(concurrency_limiter);
                get_deps!(concurrency_limiter);
                get_crates!(concurrency_limiter);
                mkinclude!(concurrency_limiter);
                 
            }}
mkmod!{config, { 
                getname!(config);
                getsrc!(config);
                getpath!(config);
                get_deps!(config);
                get_crates!(config);
                mkinclude!(config);
                 
            }}
mkmod!{constant, { 
                getname!(constant);
                getsrc!(constant);
                getpath!(constant);
                get_deps!(constant);
                get_crates!(constant);
                mkinclude!(constant);
                 
            }}
mkmod!{debuginfo, { 
                getname!(debuginfo);
                getsrc!(debuginfo);
                getpath!(debuginfo);
                get_deps!(debuginfo);
                get_crates!(debuginfo);
                mkinclude!(debuginfo);
                 
            }}
mkmod!{discriminant, { 
                getname!(discriminant);
                getsrc!(discriminant);
                getpath!(discriminant);
                get_deps!(discriminant);
                get_crates!(discriminant);
                mkinclude!(discriminant);
                 
            }}
mkmod!{driver, { 
                getname!(driver);
                getsrc!(driver);
                getpath!(driver);
                get_deps!(driver);
                get_crates!(driver);
                mkinclude!(driver);
                 
            }}
mkmod!{global_asm, { 
                getname!(global_asm);
                getsrc!(global_asm);
                getpath!(global_asm);
                get_deps!(global_asm);
                get_crates!(global_asm);
                mkinclude!(global_asm);
                 
            }}
mkmod!{inline_asm, { 
                getname!(inline_asm);
                getsrc!(inline_asm);
                getpath!(inline_asm);
                get_deps!(inline_asm);
                get_crates!(inline_asm);
                mkinclude!(inline_asm);
                 
            }}
mkmod!{intrinsics, { 
                getname!(intrinsics);
                getsrc!(intrinsics);
                getpath!(intrinsics);
                get_deps!(intrinsics);
                get_crates!(intrinsics);
                mkinclude!(intrinsics);
                 
            }}
mkmod!{linkage, { 
                getname!(linkage);
                getsrc!(linkage);
                getpath!(linkage);
                get_deps!(linkage);
                get_crates!(linkage);
                mkinclude!(linkage);
                 
            }}
mkmod!{main_shim, { 
                getname!(main_shim);
                getsrc!(main_shim);
                getpath!(main_shim);
                get_deps!(main_shim);
                get_crates!(main_shim);
                mkinclude!(main_shim);
                 
            }}
mkmod!{num, { 
                getname!(num);
                getsrc!(num);
                getpath!(num);
                get_deps!(num);
                get_crates!(num);
                mkinclude!(num);
                 
            }}
mkmod!{optimize, { 
                getname!(optimize);
                getsrc!(optimize);
                getpath!(optimize);
                get_deps!(optimize);
                get_crates!(optimize);
                mkinclude!(optimize);
                 
            }}
mkmod!{pointer, { 
                getname!(pointer);
                getsrc!(pointer);
                getpath!(pointer);
                get_deps!(pointer);
                get_crates!(pointer);
                mkinclude!(pointer);
                 
            }}
mkmod!{pretty_clif, { 
                getname!(pretty_clif);
                getsrc!(pretty_clif);
                getpath!(pretty_clif);
                get_deps!(pretty_clif);
                get_crates!(pretty_clif);
                mkinclude!(pretty_clif);
                 
            }}
mkmod!{toolchain, { 
                getname!(toolchain);
                getsrc!(toolchain);
                getpath!(toolchain);
                get_deps!(toolchain);
                get_crates!(toolchain);
                mkinclude!(toolchain);
                 
            }}
mkmod!{unsize, { 
                getname!(unsize);
                getsrc!(unsize);
                getpath!(unsize);
                get_deps!(unsize);
                get_crates!(unsize);
                mkinclude!(unsize);
                 
            }}
mkmod!{unwind_module, { 
                getname!(unwind_module);
                getsrc!(unwind_module);
                getpath!(unwind_module);
                get_deps!(unwind_module);
                get_crates!(unwind_module);
                mkinclude!(unwind_module);
                 
            }}
mkmod!{value_and_place, { 
                getname!(value_and_place);
                getsrc!(value_and_place);
                getpath!(value_and_place);
                get_deps!(value_and_place);
                get_crates!(value_and_place);
                mkinclude!(value_and_place);
                 
            }}
mkmod!{vtable, { 
                getname!(vtable);
                getsrc!(vtable);
                getpath!(vtable);
                get_deps!(vtable);
                get_crates!(vtable);
                mkinclude!(vtable);
                 
            }}
mkmod!{prelude, { 
                getname!(prelude);
                getsrc!(prelude);
                getpath!(prelude);
                get_deps!(prelude);
                get_crates!(prelude);
                mkinclude!(prelude);
                mkuse!{pub (crate) use cranelift_codegen :: Context ;}
mkuse!{pub (crate) use cranelift_codegen :: ir :: condcodes :: { FloatCC , IntCC } ;}
mkuse!{pub (crate) use cranelift_codegen :: ir :: function :: Function ;}
mkuse!{pub (crate) use cranelift_codegen :: ir :: { AbiParam , Block , FuncRef , Inst , InstBuilder , MemFlags , Signature , SourceLoc , StackSlot , StackSlotData , StackSlotKind , TrapCode , Type , Value , types , } ;}
mkuse!{pub (crate) use cranelift_module :: { self , DataDescription , FuncId , Linkage , Module } ;}
mkuse!{pub (crate) use rustc_abi :: { BackendRepr , FIRST_VARIANT , FieldIdx , Scalar , Size , VariantIdx } ;}
mkuse!{pub (crate) use rustc_data_structures :: fx :: { FxHashMap , FxIndexMap } ;}
mkuse!{pub (crate) use rustc_hir :: def_id :: { DefId , LOCAL_CRATE } ;}
mkuse!{pub (crate) use rustc_index :: Idx ;}
mkuse!{pub (crate) use rustc_middle :: mir :: { self , * } ;}
mkuse!{pub (crate) use rustc_middle :: ty :: layout :: { LayoutOf , TyAndLayout } ;}
mkuse!{pub (crate) use rustc_middle :: ty :: { self , FloatTy , Instance , InstanceKind , IntTy , Ty , TyCtxt , UintTy , } ;}
mkuse!{pub (crate) use rustc_span :: Span ;}
mkuse!{pub (crate) use crate :: abi :: * ;}
mkuse!{pub (crate) use crate :: base :: { codegen_operand , codegen_place } ;}
mkuse!{pub (crate) use crate :: cast :: * ;}
mkuse!{pub (crate) use crate :: common :: * ;}
mkuse!{pub (crate) use crate :: debuginfo :: { DebugContext , UnwindContext } ;}
mkuse!{pub (crate) use crate :: pointer :: Pointer ;}
mkuse!{pub (crate) use crate :: value_and_place :: { CPlace , CValue } ;} 
            }}
mkitem!{mkstruct!{struct PrintOnPanic < F : Fn () -> String > (F) ;}}
mkitem!{mkimpl!{impl < F : Fn () -> String > Drop for PrintOnPanic < F > { fn drop (& mut self) { if :: std :: thread :: panicking () { println ! ("{}" , (self . 0) ()) ; } } }}}
mkitem!{mkstruct!{# [doc = " The codegen context holds any information shared between the codegen of individual functions"] # [doc = " inside a single codegen unit with the exception of the Cranelift [`Module`](cranelift_module::Module)."] struct CodegenCx { output_filenames : Arc < OutputFilenames > , invocation_temp : Option < String > , should_write_ir : bool , global_asm : String , inline_asm_index : usize , debug_context : Option < DebugContext > , cgu_name : Symbol , }}}
mkitem!{mkimpl!{impl CodegenCx { fn new (tcx : TyCtxt < '_ > , isa : & dyn TargetIsa , debug_info : bool , cgu_name : Symbol) -> Self { assert_eq ! (pointer_ty (tcx) , isa . pointer_type ()) ; let debug_context = if debug_info && ! tcx . sess . target . options . is_like_windows { Some (DebugContext :: new (tcx , isa , cgu_name . as_str ())) } else { None } ; CodegenCx { output_filenames : tcx . output_filenames (()) . clone () , invocation_temp : tcx . sess . invocation_temp . clone () , should_write_ir : crate :: pretty_clif :: should_write_ir (tcx) , global_asm : String :: new () , inline_asm_index : 0 , debug_context , cgu_name , } } }}}
mkitem!{mkstruct!{pub struct CraneliftCodegenBackend { pub config : Option < BackendConfig > , }}}
mkitem!{mkimpl!{impl CodegenBackend for CraneliftCodegenBackend { fn locale_resource (& self) -> & 'static str { "" } fn init (& self , sess : & Session) { use rustc_session :: config :: { InstrumentCoverage , Lto } ; match sess . lto () { Lto :: No | Lto :: ThinLocal => { } Lto :: Thin | Lto :: Fat => { sess . dcx () . warn ("LTO is not supported. You may get a linker error.") } } if sess . opts . cg . instrument_coverage () != InstrumentCoverage :: No { sess . dcx () . fatal ("`-Cinstrument-coverage` is LLVM specific and not supported by Cranelift") ; } } fn target_config (& self , sess : & Session) -> TargetConfig { let target_features = if sess . target . arch == "x86_64" && sess . target . os != "none" { vec ! [sym :: fxsr , sym :: sse , sym :: sse2 , Symbol :: intern ("x87")] } else if sess . target . arch == "aarch64" { match & * sess . target . os { "none" => vec ! [] , "macos" => vec ! [sym :: neon , sym :: aes , sym :: sha2 , sym :: sha3] , _ => vec ! [sym :: neon] , } } else { vec ! [] } ; let unstable_target_features = target_features . clone () ; let has_reliable_f128 = ! sess . target . is_like_windows ; let has_reliable_f16 = match & * sess . target . arch { "s390x" => false , "x86_64" if sess . target . os == "windows" && sess . target . env == "gnu" && sess . target . abi != "llvm" => { false } _ => true , } ; TargetConfig { target_features , unstable_target_features , has_reliable_f16 , has_reliable_f16_math : has_reliable_f16 , has_reliable_f128 , has_reliable_f128_math : has_reliable_f128 , } } fn print_version (& self) { println ! ("Cranelift version: {}" , cranelift_codegen :: VERSION) ; } fn codegen_crate (& self , tcx : TyCtxt < '_ >) -> Box < dyn Any > { info ! ("codegen crate {}" , tcx . crate_name (LOCAL_CRATE)) ; let config = self . config . clone () . unwrap_or_else (| | { BackendConfig :: from_opts (& tcx . sess . opts . cg . llvm_args) . unwrap_or_else (| err | tcx . sess . dcx () . fatal (err)) }) ; if config . jit_mode { # [cfg (feature = "jit")] driver :: jit :: run_jit (tcx , config . jit_args) ; # [cfg (not (feature = "jit"))] tcx . dcx () . fatal ("jit support was disabled when compiling rustc_codegen_cranelift") ; } else { driver :: aot :: run_aot (tcx) } } fn join_codegen (& self , ongoing_codegen : Box < dyn Any > , sess : & Session , outputs : & OutputFilenames ,) -> (CodegenResults , FxIndexMap < WorkProductId , WorkProduct >) { ongoing_codegen . downcast :: < driver :: aot :: OngoingCodegen > () . unwrap () . join (sess , outputs) } }}}

macro_rules! enable_verifier_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function enable_verifier in module {}", module_path!());
    };
}

mkfn!{
    enable_verifier_introspect!();
    # [doc = " Determine if the Cranelift ir verifier should run."] # [doc = ""] # [doc = " Returns true when `-Zverify-llvm-ir` is passed, the `CG_CLIF_ENABLE_VERIFIER` env var is set to"] # [doc = " 1 or when cg_clif is compiled with debug assertions enabled or false otherwise."] fn enable_verifier (sess : & Session) -> bool { sess . verify_llvm_ir () || cfg ! (debug_assertions) || env :: var ("CG_CLIF_ENABLE_VERIFIER") . as_deref () == Ok ("1") }
}

macro_rules! target_triple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function target_triple in module {}", module_path!());
    };
}

mkfn!{
    target_triple_introspect!();
    fn target_triple (sess : & Session) -> target_lexicon :: Triple { match sess . target . llvm_target . parse () { Ok (triple) => triple , Err (err) => sess . dcx () . fatal (format ! ("target not recognized: {}" , err)) , } }
}

macro_rules! build_isa_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_isa in module {}", module_path!());
    };
}

mkfn!{
    build_isa_introspect!();
    fn build_isa (sess : & Session , jit : bool) -> Arc < dyn TargetIsa + 'static > { use target_lexicon :: BinaryFormat ; let target_triple = crate :: target_triple (sess) ; let mut flags_builder = settings :: builder () ; flags_builder . set ("is_pic" , if jit { "false" } else { "true" }) . unwrap () ; let enable_verifier = if enable_verifier (sess) { "true" } else { "false" } ; flags_builder . set ("enable_verifier" , enable_verifier) . unwrap () ; flags_builder . set ("regalloc_checker" , enable_verifier) . unwrap () ; let mut frame_ptr = sess . target . options . frame_pointer . clone () ; frame_ptr . ratchet (sess . opts . cg . force_frame_pointers) ; let preserve_frame_pointer = frame_ptr != rustc_target :: spec :: FramePointer :: MayOmit ; flags_builder . set ("preserve_frame_pointers" , if preserve_frame_pointer { "true" } else { "false" }) . unwrap () ; let tls_model = match target_triple . binary_format { BinaryFormat :: Elf => "elf_gd" , BinaryFormat :: Macho => "macho" , BinaryFormat :: Coff => "coff" , _ => "none" , } ; flags_builder . set ("tls_model" , tls_model) . unwrap () ; flags_builder . set ("enable_llvm_abi_extensions" , "true") . unwrap () ; if let Some (align) = sess . opts . unstable_opts . min_function_alignment { flags_builder . set ("log2_min_function_alignment" , & align . bytes () . ilog2 () . to_string ()) . unwrap () ; } use rustc_session :: config :: OptLevel ; match sess . opts . optimize { OptLevel :: No => { flags_builder . set ("opt_level" , "none") . unwrap () ; } OptLevel :: Less | OptLevel :: More | OptLevel :: Size | OptLevel :: SizeMin | OptLevel :: Aggressive => { flags_builder . set ("opt_level" , "speed_and_size") . unwrap () ; } } if let target_lexicon :: OperatingSystem :: Windows = target_triple . operating_system { flags_builder . enable ("enable_multi_ret_implicit_sret") . unwrap () ; } if let target_lexicon :: Architecture :: S390x = target_triple . architecture { flags_builder . enable ("enable_multi_ret_implicit_sret") . unwrap () ; } if let target_lexicon :: Architecture :: Aarch64 (_) | target_lexicon :: Architecture :: Riscv64 (_) | target_lexicon :: Architecture :: X86_64 = target_triple . architecture { flags_builder . enable ("enable_probestack") . unwrap () ; flags_builder . set ("probestack_strategy" , "inline") . unwrap () ; } else { flags_builder . set ("enable_probestack" , "false") . unwrap () ; } let flags = settings :: Flags :: new (flags_builder) ; let isa_builder = match sess . opts . cg . target_cpu . as_deref () { Some ("native") => cranelift_native :: builder_with_options (true) . unwrap () , Some (value) => { let mut builder = cranelift_codegen :: isa :: lookup (target_triple . clone ()) . unwrap_or_else (| err | { sess . dcx () . fatal (format ! ("can't compile for {}: {}" , target_triple , err)) ; }) ; if builder . enable (value) . is_err () { sess . dcx () . fatal ("the specified target cpu isn't currently supported by Cranelift.") ; } builder } None => { let mut builder = cranelift_codegen :: isa :: lookup (target_triple . clone ()) . unwrap_or_else (| err | { sess . dcx () . fatal (format ! ("can't compile for {}: {}" , target_triple , err)) ; }) ; if target_triple . architecture == target_lexicon :: Architecture :: X86_64 { builder . enable (sess . target . cpu . as_ref ()) . unwrap () ; } builder } } ; match isa_builder . finish (flags) { Ok (target_isa) => target_isa , Err (err) => sess . dcx () . fatal (format ! ("failed to build TargetIsa: {}" , err)) , } }
}

macro_rules! __rustc_codegen_backend_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __rustc_codegen_backend in module {}", module_path!());
    };
}

mkfn!{
    __rustc_codegen_backend_introspect!();
    # [doc = " This is the entrypoint for a hot plugged rustc_codegen_cranelift"] # [no_mangle] pub fn __rustc_codegen_backend () -> Box < dyn CodegenBackend > { Box :: new (CraneliftCodegenBackend { config : None }) }
}