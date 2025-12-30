// Generated macro for test (macro)
macro_rules! Depcrate_teststest {
() => {
// Module: crate::tests
// Provides: {"test"}
// Dependencies: {}
macro_rules ! test { (no_build $ test_name : ident { $ ($ i : tt) * } expands to { $ ($ o : tt) * }) => { # [test] fn $ test_name () { test_derive ! ($ crate :: derive :: impl_proptest_arbitrary { $ ($ i) * } expands to { $ ($ o) * } no_build) ; } } ; ($ test_name : ident { $ ($ i : tt) * } expands to { $ ($ o : tt) * }) => { # [test] fn $ test_name () { test_derive ! ($ crate :: derive :: impl_proptest_arbitrary { $ ($ i) * } expands to { $ ($ o) * }) ; } } ; }
};
}
