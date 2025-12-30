// Generated macro for tests (module)
macro_rules! Depcrate_extension_treetests {
() => {
// Module: crate::extension::tree
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use gix_testtools :: size_ok ; # [test] fn size_of_tree () { let actual = std :: mem :: size_of :: < crate :: extension :: Tree > () ; let expected = 88 ; assert ! (size_ok (actual , expected) , "the size of this structure should not change unexpectedly: {actual} <~ {expected}") ; } }
};
}
