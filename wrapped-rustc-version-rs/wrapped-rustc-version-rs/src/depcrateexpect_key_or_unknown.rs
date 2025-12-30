// Generated macro for expect_key_or_unknown (function)
macro_rules! Depcrateexpect_key_or_unknown {
() => {
// Module: crate
// Provides: {"expect_key_or_unknown"}
// Dependencies: {}
fn expect_key_or_unknown (key : & str , map : & HashMap < & str , & str >) -> Result < Option < String > , Error > { match map . get (key) { Some (& "unknown") => Ok (None) , Some (& v) => Ok (Some (String :: from (v))) , None => Err (Error :: UnexpectedVersionFormat) , } }
};
}
