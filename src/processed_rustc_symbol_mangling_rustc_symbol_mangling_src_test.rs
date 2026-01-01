/* FP:test.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_test_USE_0001
/* FP:test.rs-0002 */ use crate :: rustc_complete :: def_id :: LocalDefId ;
/* FP:test.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_test_USE_0002
/* FP:test.rs-0004 */ use crate :: rustc_complete :: ty :: print :: with_no_trimmed_paths ;
/* FP:test.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_test_USE_0003
/* FP:test.rs-0006 */ use crate :: rustc_complete :: ty :: { GenericArgs , Instance , TyCtxt } ;
/* FP:test.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_test_USE_0004
/* FP:test.rs-0008 */ use crate :: rustc_complete :: { Symbol , sym } ;
/* FP:test.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_test_USE_0005
/* FP:test.rs-0010 */ use crate :: errors :: { Kind , TestOutput } ;
/* FP:test.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_test_CONST_0006
/* FP:test.rs-0012 */ const SYMBOL_NAME : Symbol = sym :: rustc_symbol_name ;
/* FP:test.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_test_CONST_0007
/* FP:test.rs-0014 */ const DEF_PATH : Symbol = sym :: rustc_def_path ;
/* FP:test.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_test_FN_0008
/* FP:test.rs-0016 */ pub fn report_symbol_names (tcx : TyCtxt < '_ >) { if ! tcx . features () . rustc_attrs () { return ; } tcx . dep_graph . with_ignore (| | { let mut symbol_names = SymbolNamesTest { tcx } ; let crate_items = tcx . hir_crate_items (()) ; for id in crate_items . free_items () { symbol_names . process_attrs (id . owner_id . def_id) ; } for id in crate_items . trait_items () { symbol_names . process_attrs (id . owner_id . def_id) ; } for id in crate_items . impl_items () { symbol_names . process_attrs (id . owner_id . def_id) ; } for id in crate_items . foreign_items () { symbol_names . process_attrs (id . owner_id . def_id) ; } }) }
/* FP:test.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_test_STRUCT_0009
/* FP:test.rs-0018 */ struct SymbolNamesTest < 'tcx > { tcx : TyCtxt < 'tcx > , }
/* FP:test.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_symbol_mangling_src_test_IMPL_0010
/* FP:test.rs-0020 */ impl SymbolNamesTest < '_ > { fn process_attrs (& mut self , def_id : LocalDefId) { let tcx = self . tcx ; for attr in tcx . get_attrs (def_id , SYMBOL_NAME) { let def_id = def_id . to_def_id () ; let instance = Instance :: new_raw (def_id , tcx . erase_and_anonymize_regions (GenericArgs :: identity_for_item (tcx , def_id)) ,) ; let mangled = tcx . symbol_name (instance) ; tcx . dcx () . emit_err (TestOutput { span : attr . span () , kind : Kind :: SymbolName , content : format ! ("{mangled}") , }) ; if let Ok (demangling) = rustc_demangle :: try_demangle (mangled . name) { tcx . dcx () . emit_err (TestOutput { span : attr . span () , kind : Kind :: Demangling , content : format ! ("{demangling}") , }) ; tcx . dcx () . emit_err (TestOutput { span : attr . span () , kind : Kind :: DemanglingAlt , content : format ! ("{demangling:#}") , }) ; } } for attr in tcx . get_attrs (def_id , DEF_PATH) { tcx . dcx () . emit_err (TestOutput { span : attr . span () , kind : Kind :: DefPath , content : with_no_trimmed_paths ! (tcx . def_path_str (def_id)) , }) ; } } }