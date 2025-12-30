// Generated macro for all_into (macro)
macro_rules! Depcrate_testsall_into {
() => {
// Module: crate::tests
// Provides: {"all_into"}
// Dependencies: {}
macro_rules ! all_into { ($ t : ty , $ x : ident) => { test_into ::<$ t , Utf8PathBuf > ($ x . clone ()) ; test_into ::<$ t , Box < Utf8Path >> ($ x . clone ()) ; test_into ::<$ t , Arc < Utf8Path >> ($ x . clone ()) ; test_into ::<$ t , Rc < Utf8Path >> ($ x . clone ()) ; test_into ::<$ t , Cow <'_ , Utf8Path >> ($ x . clone ()) ; test_into ::<$ t , PathBuf > ($ x . clone ()) ; test_into ::<$ t , Box < Path >> ($ x . clone ()) ; test_into ::<$ t , Arc < Path >> ($ x . clone ()) ; test_into ::<$ t , Rc < Path >> ($ x . clone ()) ; test_into ::<$ t , Cow <'_ , Path >> ($ x . clone ()) ; } ; }
};
}
