// Generated macro for tests (module)
macro_rules! Depcrate_shared_util_array_strtests {
() => {
// Module: crate::shared::util::array_str
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use core :: fmt :: Write ; use super :: * ; # [test] fn fmt_write () { let mut dst = ArrayStr :: < 5 > :: new ("") . unwrap () ; assert ! (write ! (& mut dst , "abcd") . is_ok ()) ; assert ! (write ! (& mut dst , "e") . is_ok ()) ; assert ! (write ! (& mut dst , "f") . is_err ()) ; } }
};
}
