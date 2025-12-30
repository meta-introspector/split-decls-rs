// Generated macro for test (module)
macro_rules! Depcrate_booltest {
() => {
// Module: crate::bool
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_sanity () { check_strategy_sanity (ANY , None) ; } # [test] fn shrinks_properly () { let mut tree = BoolValueTree :: new (true) ; assert ! (tree . simplify ()) ; assert ! (! tree . current ()) ; assert ! (! tree . clone () . simplify ()) ; assert ! (tree . complicate ()) ; assert ! (! tree . clone () . complicate ()) ; assert ! (tree . current ()) ; assert ! (! tree . simplify ()) ; assert ! (tree . current ()) ; tree = BoolValueTree :: new (false) ; assert ! (! tree . clone () . simplify ()) ; assert ! (! tree . clone () . complicate ()) ; assert ! (! tree . current ()) ; } }
};
}
