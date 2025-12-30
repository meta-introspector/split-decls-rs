// Generated macro for tests_for_get_chunks_of_tabs (module)
macro_rules! Depcrate_tabs_in_doc_commentstests_for_get_chunks_of_tabs {
() => {
// Module: crate::tabs_in_doc_comments
// Provides: {"tests_for_get_chunks_of_tabs"}
// Dependencies: {}
# [cfg (test)] mod tests_for_get_chunks_of_tabs { use super :: get_chunks_of_tabs ; # [test] fn test_unicode_han_string () { let res = get_chunks_of_tabs (" \u{4f4d}\t") ; assert_eq ! (res , vec ! [(4 , 5)]) ; } # [test] fn test_empty_string () { let res = get_chunks_of_tabs ("") ; assert_eq ! (res , vec ! []) ; } # [test] fn test_simple () { let res = get_chunks_of_tabs ("sd\t\t\taa") ; assert_eq ! (res , vec ! [(2 , 5)]) ; } # [test] fn test_only_t () { let res = get_chunks_of_tabs ("\t\t") ; assert_eq ! (res , vec ! [(0 , 2)]) ; } # [test] fn test_only_one_t () { let res = get_chunks_of_tabs ("\t") ; assert_eq ! (res , vec ! [(0 , 1)]) ; } # [test] fn test_double () { let res = get_chunks_of_tabs ("sd\tasd\t\taa") ; assert_eq ! (res , vec ! [(2 , 3) , (6 , 8)]) ; } # [test] fn test_start () { let res = get_chunks_of_tabs ("\t\taa") ; assert_eq ! (res , vec ! [(0 , 2)]) ; } # [test] fn test_end () { let res = get_chunks_of_tabs ("aa\t\t") ; assert_eq ! (res , vec ! [(2 , 4)]) ; } # [test] fn test_start_single () { let res = get_chunks_of_tabs ("\taa") ; assert_eq ! (res , vec ! [(0 , 1)]) ; } # [test] fn test_end_single () { let res = get_chunks_of_tabs ("aa\t") ; assert_eq ! (res , vec ! [(2 , 3)]) ; } # [test] fn test_no_tabs () { let res = get_chunks_of_tabs ("dsfs") ; assert_eq ! (res , vec ! []) ; } }
};
}
