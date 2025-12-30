// Generated macro for assert_contains (macro)
macro_rules! Depcrateassert_contains {
() => {
// Module: crate
// Provides: {"assert_contains"}
// Dependencies: {}
# [cfg (test)] macro_rules ! assert_contains { ($ s : expr , $ needle : expr) => { { let (s , needle) = ($ s , $ needle) ; assert ! (s . contains (needle) , "{:?} should've contained {:?}" , s , needle) ; } } ; }
};
}
