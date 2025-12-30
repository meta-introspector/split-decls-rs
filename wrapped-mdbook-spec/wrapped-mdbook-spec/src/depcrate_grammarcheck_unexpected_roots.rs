// Generated macro for check_unexpected_roots (function)
macro_rules! Depcrate_grammarcheck_unexpected_roots {
() => {
// Module: crate::grammar
// Provides: {"check_unexpected_roots"}
// Dependencies: {}
# [doc = " This checks that all the grammar roots are what we expect."] # [doc = ""] # [doc = " This is intended to help catch any unexpected misspellings, orphaned"] # [doc = " productions, or general mistakes."] fn check_unexpected_roots (grammar : & Grammar , diag : & mut Diagnostics) { let mut set : HashSet < _ > = grammar . name_order . iter () . map (| s | s . as_str ()) . collect () ; fn remove (set : & mut HashSet < & str > , grammar : & Grammar , prod : & Production , root_name : & str) { prod . expression . visit_nt (& mut | nt | { if nt == root_name { return ; } if ! set . remove (nt) { return ; } if let Some (nt_prod) = grammar . productions . get (nt) { remove (set , grammar , nt_prod , root_name) ; } }) ; } grammar . productions . values () . filter (| prod | prod . is_root) . for_each (| root | { remove (& mut set , grammar , root , & root . name) ; }) ; let expected : HashSet < _ > = grammar . productions . values () . filter_map (| p | p . is_root . then (| | p . name . as_str ())) . collect () ; if set != expected { let new : Vec < _ > = set . difference (& expected) . collect () ; let removed : Vec < _ > = expected . difference (& set) . collect () ; if ! new . is_empty () { warn_or_err ! (diag , "New grammar production detected that is not used in any root-accessible\n\
                 production. If this is expected, mark the production with\n\
                 `@root`. If not, make sure it is spelled correctly and used in\n\
                 another root-accessible production.\n\
                 \n\
                 The new names are: {new:?}\n") ; } else if ! removed . is_empty () { warn_or_err ! (diag , "Old grammar production root seems to have been removed\n\
                 (it is used in some other production that is root-accessible).\n\
                 If this is expected, remove `@root` from the production.\n\
                 \n\
                 The removed names are: {removed:?}\n") ; } else { unreachable ! ("unexpected") ; } } }
};
}
