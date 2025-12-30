// Generated macro for test (module)
macro_rules! Depcrate_string_reftest {
() => {
// Module: crate::string_ref
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_size () { println ! ("KStringRef: {}" , std :: mem :: size_of ::< KStringRef <'static >> ()) ; } }
};
}
