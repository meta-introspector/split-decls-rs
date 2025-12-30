// Generated macro for test (module)
macro_rules! Depcrate_mdtest {
() => {
// Module: crate::md
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { # [cfg (ossl300)] use super :: Md ; # [test] # [cfg (ossl300)] fn test_md_fetch_properties () { assert ! (Md :: fetch (None , "SHA-256" , Some ("provider=gibberish")) . is_err ()) ; } }
};
}
