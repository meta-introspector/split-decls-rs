macro_rules! deps {
    () => {
        CodegenCx!();
        SymbolAlreadyDefined!();
        Linkage!();
    };
}

macro_rules! check_and_apply_linkage {
    () => {
        deps!();
        fn check_and_apply_linkage < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , attrs : & CodegenFnAttrs , llty : & 'll Type , sym : & str , def_id : DefId ,) -> & 'll Value { if let Some (linkage) = attrs . import_linkage { debug ! ("get_static: sym={} linkage={:?}" , sym , linkage) ; let g1 = if matches ! (attrs . import_linkage , Some (Linkage :: ExternalWeak)) { let instance = Instance :: mono (cx . tcx , def_id) ; if let ty :: Adt (struct_def , args) = instance . ty (cx . tcx , cx . typing_env ()) . kind () && cx . tcx . is_lang_item (struct_def . did () , LangItem :: Option) && let ty :: FnPtr (sig , header) = args . type_at (0) . kind () { let fn_sig = sig . with (* header) ; let fn_abi = cx . fn_abi_of_fn_ptr (fn_sig , ty :: List :: empty ()) ; cx . declare_fn (sym , & fn_abi , None) } else { cx . declare_global (sym , cx . type_i8 ()) } } else { cx . declare_global (sym , cx . type_i8 ()) } ; llvm :: set_linkage (g1 , base :: linkage_to_llvm (linkage)) ; let real_name = format ! ("_rust_extern_with_linkage_{:016x}_{sym}" , cx . tcx . stable_crate_id (LOCAL_CRATE)) ; let g2 = cx . define_global (& real_name , llty) . unwrap_or_else (| | { cx . sess () . dcx () . emit_fatal (SymbolAlreadyDefined { span : cx . tcx . def_span (def_id) , symbol_name : sym , }) }) ; llvm :: set_linkage (g2 , llvm :: Linkage :: InternalLinkage) ; llvm :: set_initializer (g2 , g1) ; g2 } else if cx . tcx . sess . target . arch == "x86" && common :: is_mingw_gnu_toolchain (& cx . tcx . sess . target) && let Some (dllimport) = crate :: common :: get_dllimport (cx . tcx , def_id , sym) { cx . declare_global (& common :: i686_decorated_name (dllimport , true , true , false) , llty) } else { cx . declare_global (sym , llty) } }
    };
}

check_and_apply_linkage!();