// Generated macro for test_derive (macro)
macro_rules! Depcrate_teststest_derive {
() => {
// Module: crate::tests
// Provides: {"test_derive"}
// Dependencies: {}
macro_rules ! test_derive { ($ name : path { $ ($ i : tt) * } expands to { $ ($ o : tt) * }) => { { # [allow (dead_code)] fn ensure_compiles () { $ ($ i) * $ ($ o) * } test_derive ! ($ name { $ ($ i) * } expands to { $ ($ o) * } no_build) ; } } ; ($ name : path { $ ($ i : tt) * } expands to { $ ($ o : tt) * } no_build) => { { let expected = stringify ! ($ ($ o) *) . parse ::< proc_macro2 :: TokenStream > () . expect ("output should be a valid TokenStream") ; let i = stringify ! ($ ($ i) *) ; let parsed = $ crate :: syn :: parse_str ::<$ crate :: syn :: DeriveInput > (i) . expect (concat ! ("Failed to parse input to `#[derive(" , stringify ! ($ name) , ")]`")) ; let res = $ name (parsed) ; assert_eq ! (format ! ("{}" , res) , format ! ("{}" , expected)) ; } } ; }
};
}
