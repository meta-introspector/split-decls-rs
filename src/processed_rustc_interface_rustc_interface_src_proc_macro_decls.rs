/* FP:proc_macro_decls.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_proc_macro_decls_USE_0001
/* FP:proc_macro_decls.rs-0002 */ use crate :: rustc_complete :: attr ;
/* FP:proc_macro_decls.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_proc_macro_decls_USE_0002
/* FP:proc_macro_decls.rs-0004 */ use crate :: rustc_complete :: def_id :: LocalDefId ;
/* FP:proc_macro_decls.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_proc_macro_decls_USE_0003
/* FP:proc_macro_decls.rs-0006 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:proc_macro_decls.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_proc_macro_decls_USE_0004
/* FP:proc_macro_decls.rs-0008 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:proc_macro_decls.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_proc_macro_decls_USE_0005
/* FP:proc_macro_decls.rs-0010 */ use crate :: rustc_complete :: sym ;
/* FP:proc_macro_decls.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_proc_macro_decls_FN_0006
/* FP:proc_macro_decls.rs-0012 */ fn proc_macro_decls_static (tcx : TyCtxt < '_ > , () : ()) -> Option < LocalDefId > { let mut decls = None ; for id in tcx . hir_free_items () { let attrs = tcx . hir_attrs (id . hir_id ()) ; if attr :: contains_name (attrs , sym :: rustc_proc_macro_decls) { decls = Some (id . owner_id . def_id) ; } } decls }
/* FP:proc_macro_decls.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_proc_macro_decls_FN_0007
/* FP:proc_macro_decls.rs-0014 */ pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { proc_macro_decls_static , .. * providers } ; }