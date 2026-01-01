/* FP:diagnostic_items.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_diagnostic_items_USE_0001
/* FP:diagnostic_items.rs-0002 */ use crate :: rustc_complete :: diagnostic_items :: DiagnosticItems ;
/* FP:diagnostic_items.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_diagnostic_items_USE_0002
/* FP:diagnostic_items.rs-0004 */ use crate :: rustc_complete :: { Attribute , CRATE_OWNER_ID , OwnerId } ;
/* FP:diagnostic_items.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_diagnostic_items_USE_0003
/* FP:diagnostic_items.rs-0006 */ use crate :: rustc_complete :: query :: { LocalCrate , Providers } ;
/* FP:diagnostic_items.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_diagnostic_items_USE_0004
/* FP:diagnostic_items.rs-0008 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:diagnostic_items.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_diagnostic_items_USE_0005
/* FP:diagnostic_items.rs-0010 */ use crate :: rustc_complete :: def_id :: { DefId , LOCAL_CRATE } ;
/* FP:diagnostic_items.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_diagnostic_items_USE_0006
/* FP:diagnostic_items.rs-0012 */ use crate :: rustc_complete :: { Symbol , sym } ;
/* FP:diagnostic_items.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_diagnostic_items_USE_0007
/* FP:diagnostic_items.rs-0014 */ use crate :: errors :: DuplicateDiagnosticItemInCrate ;
/* FP:diagnostic_items.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_diagnostic_items_FN_0008
/* FP:diagnostic_items.rs-0016 */ fn observe_item < 'tcx > (tcx : TyCtxt < 'tcx > , diagnostic_items : & mut DiagnosticItems , owner : OwnerId) { let attrs = tcx . hir_attrs (owner . into ()) ; if let Some (name) = extract (attrs) { collect_item (tcx , diagnostic_items , name , owner . to_def_id ()) ; } }
/* FP:diagnostic_items.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_diagnostic_items_FN_0009
/* FP:diagnostic_items.rs-0018 */ fn collect_item (tcx : TyCtxt < '_ > , items : & mut DiagnosticItems , name : Symbol , item_def_id : DefId) { items . id_to_name . insert (item_def_id , name) ; if let Some (original_def_id) = items . name_to_id . insert (name , item_def_id) { if original_def_id != item_def_id { report_duplicate_item (tcx , name , original_def_id , item_def_id) ; } } }
/* FP:diagnostic_items.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_diagnostic_items_FN_0010
/* FP:diagnostic_items.rs-0020 */ fn report_duplicate_item (tcx : TyCtxt < '_ > , name : Symbol , original_def_id : DefId , item_def_id : DefId ,) { let orig_span = tcx . hir_span_if_local (original_def_id) ; let duplicate_span = tcx . hir_span_if_local (item_def_id) ; tcx . dcx () . emit_err (DuplicateDiagnosticItemInCrate { duplicate_span , orig_span , crate_name : tcx . crate_name (item_def_id . krate) , orig_crate_name : tcx . crate_name (original_def_id . krate) , different_crates : (item_def_id . krate != original_def_id . krate) , name , }) ; }
/* FP:diagnostic_items.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_diagnostic_items_FN_0011
/* FP:diagnostic_items.rs-0022 */ # [doc = " Extract the first `rustc_diagnostic_item = \"$name\"` out of a list of attributes."] fn extract (attrs : & [Attribute]) -> Option < Symbol > { attrs . iter () . find_map (| attr | { if attr . has_name (sym :: rustc_diagnostic_item) { attr . value_str () } else { None } }) }
/* FP:diagnostic_items.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_diagnostic_items_FN_0012
/* FP:diagnostic_items.rs-0024 */ # [doc = " Traverse and collect the diagnostic items in the current"] fn diagnostic_items (tcx : TyCtxt < '_ > , _ : LocalCrate) -> DiagnosticItems { let mut diagnostic_items = DiagnosticItems :: default () ; let crate_items = tcx . hir_crate_items (()) ; for id in crate_items . owners () . chain (std :: iter :: once (CRATE_OWNER_ID)) { observe_item (tcx , & mut diagnostic_items , id) ; } diagnostic_items }
/* FP:diagnostic_items.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_diagnostic_items_FN_0013
/* FP:diagnostic_items.rs-0026 */ # [doc = " Traverse and collect all the diagnostic items in all crates."] fn all_diagnostic_items (tcx : TyCtxt < '_ > , () : ()) -> DiagnosticItems { let mut items = DiagnosticItems :: default () ; for cnum in tcx . crates (()) . iter () . copied () . filter (| cnum | tcx . is_user_visible_dep (* cnum)) . chain (std :: iter :: once (LOCAL_CRATE)) { for (& name , & def_id) in & tcx . diagnostic_items (cnum) . name_to_id { collect_item (tcx , & mut items , name , def_id) ; } } items }
/* FP:diagnostic_items.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_passes_src_diagnostic_items_FN_0014
/* FP:diagnostic_items.rs-0028 */ pub (crate) fn provide (providers : & mut Providers) { providers . diagnostic_items = diagnostic_items ; providers . all_diagnostic_items = all_diagnostic_items ; }