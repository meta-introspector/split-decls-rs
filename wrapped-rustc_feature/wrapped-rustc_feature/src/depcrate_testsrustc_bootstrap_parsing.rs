// Generated macro for rustc_bootstrap_parsing (function)
macro_rules! Depcrate_testsrustc_bootstrap_parsing {
() => {
// Module: crate::tests
// Provides: {"rustc_bootstrap_parsing"}
// Dependencies: {}
# [test] fn rustc_bootstrap_parsing () { let is_bootstrap = | env : & str , krate : Option < & str > | { matches ! (UnstableFeatures :: from_environment_value (krate , Ok (env . to_string ())) , UnstableFeatures :: Cheat) } ; assert ! (is_bootstrap ("1" , None)) ; assert ! (is_bootstrap ("1" , Some ("x"))) ; assert ! (is_bootstrap ("x" , Some ("x"))) ; assert ! (is_bootstrap ("x,y,z" , Some ("x"))) ; assert ! (is_bootstrap ("x,y,z" , Some ("y"))) ; assert ! (! is_bootstrap ("x" , Some ("a"))) ; assert ! (! is_bootstrap ("x,y,z" , Some ("a"))) ; assert ! (! is_bootstrap ("x,y,z" , None)) ; assert ! (! is_bootstrap ("0" , None)) ; let is_force_stable = | krate : Option < & str > | { matches ! (UnstableFeatures :: from_environment_value (krate , Ok ("-1" . to_string ())) , UnstableFeatures :: Disallow) } ; assert ! (is_force_stable (None)) ; assert ! (is_force_stable (Some ("x"))) ; assert ! (is_force_stable (Some ("x,y,z"))) ; }
};
}
