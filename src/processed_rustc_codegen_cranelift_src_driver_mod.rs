/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_driver_mod_USE_0001
/* FP:mod.rs-0002 */ use crate :: rustc_data_structures :: profiling :: SelfProfilerRef ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_driver_mod_USE_0002
/* FP:mod.rs-0004 */ use crate :: rustc_complete :: middle :: codegen_fn_attrs :: CodegenFnAttrFlags ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_driver_mod_USE_0003
/* FP:mod.rs-0006 */ use crate :: rustc_complete :: mir :: mono :: { MonoItem , MonoItemData } ;
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_driver_mod_USE_0004
/* FP:mod.rs-0008 */ use crate :: prelude :: * ;
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_driver_mod_MOD_0005
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_driver_mod_MOD_0006
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_driver_mod_FN_0007
/* FP:mod.rs-0014 */ fn predefine_mono_items < 'tcx > (tcx : TyCtxt < 'tcx > , module : & mut dyn Module , mono_items : & [(MonoItem < 'tcx > , MonoItemData)] ,) { tcx . prof . generic_activity ("predefine functions") . run (| | { let is_compiler_builtins = tcx . is_compiler_builtins (LOCAL_CRATE) ; for & (mono_item , data) in mono_items { match mono_item { MonoItem :: Fn (instance) => { let name = tcx . symbol_name (instance) . name ; let _inst_guard = crate :: PrintOnPanic (| | format ! ("{:?} {}" , instance , name)) ; let sig = get_function_sig (tcx , module . target_config () . default_call_conv , instance) ; let linkage = crate :: linkage :: get_clif_linkage (mono_item , data . linkage , data . visibility , is_compiler_builtins ,) ; let is_naked = tcx . codegen_instance_attrs (instance . def) . flags . contains (CodegenFnAttrFlags :: NAKED) ; module . declare_function (name , if is_naked { Linkage :: Import } else { linkage } , & sig ,) . unwrap () ; } MonoItem :: Static (_) | MonoItem :: GlobalAsm (_) => { } } } }) ; }
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_driver_mod_STRUCT_0008
/* FP:mod.rs-0016 */ struct MeasuremeProfiler (SelfProfilerRef) ;
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_driver_mod_STRUCT_0009
/* FP:mod.rs-0018 */ struct TimingGuard { profiler : std :: mem :: ManuallyDrop < SelfProfilerRef > , inner : Option < crate :: rustc_data_structures :: profiling :: TimingGuard < 'static > > , }
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_driver_mod_IMPL_0010
/* FP:mod.rs-0020 */ impl Drop for TimingGuard { fn drop (& mut self) { self . inner . take () ; unsafe { std :: mem :: ManuallyDrop :: drop (& mut self . profiler) ; } } }
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_driver_mod_IMPL_0011
/* FP:mod.rs-0022 */ impl cranelift_codegen :: timing :: Profiler for MeasuremeProfiler { fn start_pass (& self , pass : cranelift_codegen :: timing :: Pass) -> Box < dyn std :: any :: Any > { let mut timing_guard = Box :: new (TimingGuard { profiler : std :: mem :: ManuallyDrop :: new (self . 0 . clone ()) , inner : None , }) ; timing_guard . inner = Some (unsafe { & * (& * timing_guard . profiler as & SelfProfilerRef as * const SelfProfilerRef) } . generic_activity (pass . description ()) ,) ; timing_guard } }