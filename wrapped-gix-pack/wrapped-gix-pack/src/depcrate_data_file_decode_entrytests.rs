// Generated macro for tests (module)
macro_rules! Depcrate_data_file_decode_entrytests {
() => {
// Module: crate::data::file::decode::entry
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use gix_testtools :: size_ok ; use super :: * ; # [test] fn size_of_decode_entry_outcome () { let actual = std :: mem :: size_of :: < Outcome > () ; let expected = 32 ; assert ! (size_ok (actual , expected) , "this shouldn't change without use noticing as it's returned a lot: {actual} <~ {expected}") ; } }
};
}
