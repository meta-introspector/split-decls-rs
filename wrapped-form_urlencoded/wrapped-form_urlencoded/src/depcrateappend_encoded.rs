// Generated macro for append_encoded (function)
macro_rules! Depcrateappend_encoded {
() => {
// Module: crate
// Provides: {"append_encoded"}
// Dependencies: {}
fn append_encoded (s : & str , string : & mut String , encoding : EncodingOverride < '_ >) { string . extend (byte_serialize (& encode (encoding , s))) }
};
}
