/* FP:check_unused.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_check_unused_USE_0001
/* FP:check_unused.rs-0002 */ use crate :: rustc_data_structures :: unord :: { ExtendUnord , UnordSet } ;
/* FP:check_unused.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_check_unused_USE_0002
/* FP:check_unused.rs-0004 */ use crate :: rustc_complete :: def :: DefKind ;
/* FP:check_unused.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_check_unused_USE_0003
/* FP:check_unused.rs-0006 */ use crate :: rustc_complete :: def_id :: LocalDefId ;
/* FP:check_unused.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_check_unused_USE_0004
/* FP:check_unused.rs-0008 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:check_unused.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_check_unused_USE_0005
/* FP:check_unused.rs-0010 */ use crate :: rustc_complete :: lint ;
/* FP:check_unused.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_check_unused_USE_0006
/* FP:check_unused.rs-0012 */ use tracing :: debug ;
/* FP:check_unused.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_check_unused_FN_0007
/* FP:check_unused.rs-0014 */ pub (super) fn check_unused_traits (tcx : TyCtxt < '_ > , () : ()) { let mut used_trait_imports = UnordSet :: < LocalDefId > :: default () ; for item_def_id in tcx . hir_body_owners () { let imports = tcx . used_trait_imports (item_def_id) ; debug ! ("GatherVisitor: item_def_id={:?} with imports {:#?}" , item_def_id , imports) ; used_trait_imports . extend_unord (imports . items () . copied ()) ; } for & id in tcx . resolutions (()) . maybe_unused_trait_imports . iter () { debug_assert_eq ! (tcx . def_kind (id) , DefKind :: Use) ; if tcx . visibility (id) . is_public () { continue ; } if used_trait_imports . contains (& id) { continue ; } let item = tcx . hir_expect_item (id) ; if item . span . is_dummy () { continue ; } let (path , _) = item . expect_use () ; tcx . node_span_lint (lint :: builtin :: UNUSED_IMPORTS , item . hir_id () , path . span , | lint | { if let Ok (snippet) = tcx . sess . source_map () . span_to_snippet (path . span) { lint . primary_message (format ! ("unused import: `{snippet}`")) ; } else { lint . primary_message ("unused import") ; } }) ; } }