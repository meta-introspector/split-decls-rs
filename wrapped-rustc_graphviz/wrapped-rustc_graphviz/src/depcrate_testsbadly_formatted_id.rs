// Generated macro for badly_formatted_id (function)
macro_rules! Depcrate_testsbadly_formatted_id {
() => {
// Module: crate::tests
// Provides: {"badly_formatted_id"}
// Dependencies: {}
# [test] fn badly_formatted_id () { let id2 = Id :: new ("Weird { struct : ure } !!!") ; match id2 { Ok (_) => panic ! ("graphviz id suddenly allows spaces, brackets and stuff") , Err (..) => { } } }
};
}
