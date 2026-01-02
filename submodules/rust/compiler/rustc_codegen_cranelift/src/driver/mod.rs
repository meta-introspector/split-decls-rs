mkuse!{use rustc_data_structures :: profiling :: SelfProfilerRef ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: CodegenFnAttrFlags ;}
mkuse!{use rustc_middle :: mir :: mono :: { MonoItem , MonoItemData } ;}
mkuse!{use crate :: prelude :: * ;}
mkmod!{aot, { 
                getname!(aot);
                getsrc!(aot);
                getpath!(aot);
                get_deps!(aot);
                get_crates!(aot);
                mkinclude!(aot);
                 
            }}
mkmod!{jit, { 
                getname!(jit);
                getsrc!(jit);
                getpath!(jit);
                get_deps!(jit);
                get_crates!(jit);
                mkinclude!(jit);
                 
            }}

macro_rules! predefine_mono_items_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function predefine_mono_items in module {}", module_path!());
    };
}

mkfn!{
    predefine_mono_items_introspect!();
    fn predefine_mono_items < 'tcx > (tcx : TyCtxt < 'tcx > , module : & mut dyn Module , mono_items : & [(MonoItem < 'tcx > , MonoItemData)] ,) { tcx . prof . generic_activity ("predefine functions") . run (| | { let is_compiler_builtins = tcx . is_compiler_builtins (LOCAL_CRATE) ; for & (mono_item , data) in mono_items { match mono_item { MonoItem :: Fn (instance) => { let name = tcx . symbol_name (instance) . name ; let _inst_guard = crate :: PrintOnPanic (| | format ! ("{:?} {}" , instance , name)) ; let sig = get_function_sig (tcx , module . target_config () . default_call_conv , instance) ; let linkage = crate :: linkage :: get_clif_linkage (mono_item , data . linkage , data . visibility , is_compiler_builtins ,) ; let is_naked = tcx . codegen_instance_attrs (instance . def) . flags . contains (CodegenFnAttrFlags :: NAKED) ; module . declare_function (name , if is_naked { Linkage :: Import } else { linkage } , & sig ,) . unwrap () ; } MonoItem :: Static (_) | MonoItem :: GlobalAsm (_) => { } } } }) ; }
}
mkitem!{mkstruct!{struct MeasuremeProfiler (SelfProfilerRef) ;}}
mkitem!{mkstruct!{struct TimingGuard { profiler : std :: mem :: ManuallyDrop < SelfProfilerRef > , inner : Option < rustc_data_structures :: profiling :: TimingGuard < 'static > > , }}}
mkitem!{mkimpl!{impl Drop for TimingGuard { fn drop (& mut self) { self . inner . take () ; unsafe { std :: mem :: ManuallyDrop :: drop (& mut self . profiler) ; } } }}}
mkitem!{mkimpl!{impl cranelift_codegen :: timing :: Profiler for MeasuremeProfiler { fn start_pass (& self , pass : cranelift_codegen :: timing :: Pass) -> Box < dyn std :: any :: Any > { let mut timing_guard = Box :: new (TimingGuard { profiler : std :: mem :: ManuallyDrop :: new (self . 0 . clone ()) , inner : None , }) ; timing_guard . inner = Some (unsafe { & * (& * timing_guard . profiler as & SelfProfilerRef as * const SelfProfilerRef) } . generic_activity (pass . description ()) ,) ; timing_guard } }}}