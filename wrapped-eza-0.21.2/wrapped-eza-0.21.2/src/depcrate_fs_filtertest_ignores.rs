// Generated macro for test_ignores (module)
macro_rules! Depcrate_fs_filtertest_ignores {
() => {
// Module: crate::fs::filter
// Provides: {"test_ignores"}
// Dependencies: {}
# [cfg (test)] mod test_ignores { use super :: * ; # [test] fn empty_matches_nothing () { let pats = IgnorePatterns :: empty () ; assert ! (! pats . is_ignored ("nothing")) ; assert ! (! pats . is_ignored ("test.mp3")) ; } # [test] fn ignores_a_glob () { let (pats , fails) = IgnorePatterns :: parse_from_iter (vec ! ["*.mp3"]) ; assert ! (fails . is_empty ()) ; assert ! (! pats . is_ignored ("nothing")) ; assert ! (pats . is_ignored ("test.mp3")) ; } # [test] fn ignores_an_exact_filename () { let (pats , fails) = IgnorePatterns :: parse_from_iter (vec ! ["nothing"]) ; assert ! (fails . is_empty ()) ; assert ! (pats . is_ignored ("nothing")) ; assert ! (! pats . is_ignored ("test.mp3")) ; } # [test] fn ignores_both () { let (pats , fails) = IgnorePatterns :: parse_from_iter (vec ! ["nothing" , "*.mp3"]) ; assert ! (fails . is_empty ()) ; assert ! (pats . is_ignored ("nothing")) ; assert ! (pats . is_ignored ("test.mp3")) ; } }
};
}
