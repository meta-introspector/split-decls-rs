// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: ptr ; # [test] fn test_cvt () { assert ! (crate :: cvt (- 1) . is_err ()) ; assert ! (crate :: cvt_p (ptr :: null_mut ::< () > ()) . is_err ()) ; } }
};
}
