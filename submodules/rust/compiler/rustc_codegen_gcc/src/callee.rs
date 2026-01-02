mkuse!{# [cfg (feature = "master")] use gccjit :: { FnAttribute , Visibility } ;}
mkuse!{use gccjit :: { Function , FunctionType } ;}
mkuse!{use rustc_middle :: ty :: layout :: { FnAbiOf , HasTyCtxt } ;}
mkuse!{use rustc_middle :: ty :: { self , Instance , TypeVisitableExt } ;}
mkuse!{use crate :: attributes ;}
mkuse!{use crate :: context :: CodegenCx ;}

macro_rules! get_fn_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_fn in module {}", module_path!());
    };
}

mkfn!{
    get_fn_introspect!();
    # [doc = " Codegens a reference to a fn/method item, monomorphizing and"] # [doc = " inlining as it goes."] # [doc = ""] # [doc = " # Parameters"] # [doc = ""] # [doc = " - `cx`: the crate context"] # [doc = " - `instance`: the instance to be instantiated"] pub fn get_fn < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , instance : Instance < 'tcx >) -> Function < 'gcc > { let tcx = cx . tcx () ; assert ! (! instance . args . has_infer ()) ; assert ! (! instance . args . has_escaping_bound_vars ()) ; let sym = tcx . symbol_name (instance) . name ; if let Some (& func) = cx . function_instances . borrow () . get (& instance) { return func ; } let fn_abi = cx . fn_abi_of_instance (instance , ty :: List :: empty ()) ; let func = if let Some (_func) = cx . get_declared_value (sym) { unreachable ! () ; } else { cx . linkage . set (FunctionType :: Extern) ; let func = cx . declare_fn (sym , fn_abi) ; attributes :: from_fn_attrs (cx , func , instance) ; # [cfg (feature = "master")] { let instance_def_id = instance . def_id () ; let is_generic = instance . args . non_erasable_generics () . next () . is_some () ; let is_hidden = if is_generic { if ! (cx . tcx . sess . opts . share_generics () || tcx . codegen_instance_attrs (instance . def) . inline == rustc_hir :: attrs :: InlineAttr :: Never) { true } else if let Some (instance_def_id) = instance_def_id . as_local () { cx . tcx . is_unreachable_local_definition (instance_def_id) || ! cx . tcx . local_crate_exports_generics () } else { instance . upstream_monomorphization (tcx) . is_none () && ! cx . tcx . local_crate_exports_generics () } } else { cx . tcx . is_codegened_item (instance_def_id) && (! instance_def_id . is_local () || ! cx . tcx . is_reachable_non_generic (instance_def_id)) } ; if is_hidden { func . add_attribute (FnAttribute :: Visibility (Visibility :: Hidden)) ; } } func } ; cx . function_instances . borrow_mut () . insert (instance , func) ; func }
}