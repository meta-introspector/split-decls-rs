// Generated macro for assert_ends_with (macro)
macro_rules! Depcrateassert_ends_with {
() => {
// Module: crate
// Provides: {"assert_ends_with"}
// Dependencies: {}
# [cfg (test)] macro_rules ! assert_ends_with { ($ s : expr , $ suffix : expr) => { { let (s , suffix) = ($ s , $ suffix) ; assert ! (s . ends_with (suffix) , "{:?} should've ended in {:?}" , s , suffix) ; } } ; }
};
}
