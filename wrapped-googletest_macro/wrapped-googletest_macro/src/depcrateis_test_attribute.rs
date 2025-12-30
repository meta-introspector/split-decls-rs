// Generated macro for is_test_attribute (function)
macro_rules! Depcrateis_test_attribute {
() => {
// Module: crate
// Provides: {"is_test_attribute"}
// Dependencies: {}
fn is_test_attribute (attr : & Attribute) -> bool { match attr . path () . segments . last () { Some (last_segment) => last_segment . ident == "test" , None => false , } }
};
}
