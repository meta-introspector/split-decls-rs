// Generated macro for malformed_explicit_test_attr (function)
macro_rules! Depcrate_errormalformed_explicit_test_attr {
() => {
// Module: crate::error
// Provides: {"malformed_explicit_test_attr"}
// Dependencies: {}
fn malformed_explicit_test_attr (test : & ItemFn) -> Errors < '_ > { let Some (explicit_test_attr) = test . attrs . iter () . find (| attr | attr_is (attr , "test_attr")) else { return Box :: new (std :: iter :: empty ()) ; } ; match explicit_test_attr . meta { syn :: Meta :: List (_) => Box :: new (std :: iter :: empty ()) , syn :: Meta :: Path (_) | syn :: Meta :: NameValue (_) => { Box :: new (std :: iter :: once (syn :: Error :: new_spanned (explicit_test_attr . path () , "invalid `test_attr` syntax; should be `#[test_attr(<test attribute>)]`" ,))) } } }
};
}
