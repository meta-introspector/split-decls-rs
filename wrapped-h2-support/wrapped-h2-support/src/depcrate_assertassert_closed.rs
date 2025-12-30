// Generated macro for assert_closed (macro)
macro_rules! Depcrate_assertassert_closed {
() => {
// Module: crate::assert
// Provides: {"assert_closed"}
// Dependencies: {}
# [macro_export] macro_rules ! assert_closed { ($ transport : expr) => { { use futures :: StreamExt ; assert ! ($ transport . next () . await . is_none ()) ; } } ; }
};
}
