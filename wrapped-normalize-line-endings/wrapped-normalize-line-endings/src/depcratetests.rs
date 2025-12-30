// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: iter :: FromIterator ; # [test] fn normalized () { let input = "This is a string \n with \r some \n\r\n random newlines\r\r\n\n" ; assert_eq ! (& String :: from_iter (super :: normalized (input . chars ())) , "This is a string \n with \n some \n\n random newlines\n\n\n") ; } }
};
}
