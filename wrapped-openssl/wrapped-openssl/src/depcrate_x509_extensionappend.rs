// Generated macro for append (function)
macro_rules! Depcrate_x509_extensionappend {
() => {
// Module: crate::x509::extension
// Provides: {"append"}
// Dependencies: {}
fn append (value : & mut String , first : & mut bool , should : bool , element : & str) { if ! should { return ; } if ! * first { value . push (',') ; } * first = false ; value . push_str (element) ; }
};
}
