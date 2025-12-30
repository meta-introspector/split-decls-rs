// Generated macro for assert_diffs (macro)
macro_rules! Depcrate_testsassert_diffs {
() => {
// Module: crate::tests
// Provides: {"assert_diffs"}
// Dependencies: {}
macro_rules ! assert_diffs { ([$ ($ kind : ident ($ text : literal)) ,* $ (,) ?] , $ solution : ident , $ msg : expr $ (,) ?) => { let expected = & [$ (Chunk ::$ kind ($ text)) ,*] ; assert ! (same_diffs (expected , &$ solution . diffs) , concat ! ($ msg , "\nexpected={:#?}\nactual={:#?}") , expected , $ solution . diffs ,) ; } ; }
};
}
