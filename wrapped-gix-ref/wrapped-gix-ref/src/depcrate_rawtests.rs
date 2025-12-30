// Generated macro for tests (module)
macro_rules! Depcrate_rawtests {
() => {
// Module: crate::raw
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use gix_testtools :: size_ok ; use super :: * ; # [test] fn size_of_reference () { let actual = std :: mem :: size_of :: < Reference > () ; let expected = 80 ; assert ! (size_ok (actual , expected) , "let's not let it change size undetected: {actual} <~ {expected}") ; } }
};
}
