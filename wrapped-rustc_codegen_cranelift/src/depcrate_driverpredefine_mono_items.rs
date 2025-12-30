// Generated macro for predefine_mono_items (function)
macro_rules! Depcrate_driverpredefine_mono_items {
() => {
// Module: crate::driver
// Provides: {"predefine_mono_items"}
// Dependencies: {}
fn predefine_mono_items < 'tcx > (tcx : TyCtxt < 'tcx > , module : & mut dyn Module , mono_items : & [(MonoItem < 'tcx > , MonoItemData)] ,) { tcx . prof . generic_activity ("predefine functions") . run (| | { let is_compiler_builtins = tcx . is_compiler_builtins (LOCAL_CRATE) ; for & (mono_item , data) in mono_items { match mono_item { MonoItem :: Fn (instance) => { let name = tcx . symbol_name (instance) . name ; let _inst_guard = crate :: PrintOnPanic (| | format ! ("{:?} {}" , instance , name)) ; let sig = get_function_sig (tcx , module . target_config () . default_call_conv , instance) ; let linkage = crate :: linkage :: get_clif_linkage (mono_item , data . linkage , data . visibility , is_compiler_builtins ,) ; let is_naked = tcx . codegen_instance_attrs (instance . def) . flags . contains (CodegenFnAttrFlags :: NAKED) ; module . declare_function (name , if is_naked { Linkage :: Import } else { linkage } , & sig ,) . unwrap () ; } MonoItem :: Static (_) | MonoItem :: GlobalAsm (_) => { } } } }) ; }
};
}
