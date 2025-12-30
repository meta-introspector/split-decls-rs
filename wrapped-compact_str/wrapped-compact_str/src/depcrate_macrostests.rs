// Generated macro for tests (module)
macro_rules! Depcrate_macrostests {
() => {
// Module: crate::macros
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn test_macros () { assert_eq ! (format_compact ! ("2") , "2") ; assert_eq ! (format_compact ! ("{}" , 2) , "2") ; assert ! (! format_compact ! ("2") . is_heap_allocated ()) ; assert ! (! format_compact ! ("{}" , 2) . is_heap_allocated ()) ; } }
};
}
