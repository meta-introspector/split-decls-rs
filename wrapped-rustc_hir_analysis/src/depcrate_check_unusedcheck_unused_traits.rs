// Generated macro for check_unused_traits (function)
macro_rules! Depcrate_check_unusedcheck_unused_traits {
() => {
// Module: crate::check_unused
// Provides: {"check_unused_traits"}
// Dependencies: {}
pub (super) fn check_unused_traits (tcx : TyCtxt < '_ > , () : ()) { let mut used_trait_imports = UnordSet :: < LocalDefId > :: default () ; for item_def_id in tcx . hir_body_owners () { let imports = tcx . used_trait_imports (item_def_id) ; debug ! ("GatherVisitor: item_def_id={:?} with imports {:#?}" , item_def_id , imports) ; used_trait_imports . extend_unord (imports . items () . copied ()) ; } for & id in tcx . resolutions (()) . maybe_unused_trait_imports . iter () { debug_assert_eq ! (tcx . def_kind (id) , DefKind :: Use) ; if tcx . visibility (id) . is_public () { continue ; } if used_trait_imports . contains (& id) { continue ; } let item = tcx . hir_expect_item (id) ; if item . span . is_dummy () { continue ; } let (path , _) = item . expect_use () ; tcx . node_span_lint (lint :: builtin :: UNUSED_IMPORTS , item . hir_id () , path . span , | lint | { if let Ok (snippet) = tcx . sess . source_map () . span_to_snippet (path . span) { lint . primary_message (format ! ("unused import: `{snippet}`")) ; } else { lint . primary_message ("unused import") ; } }) ; } }
};
}
