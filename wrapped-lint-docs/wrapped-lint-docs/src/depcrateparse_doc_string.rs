// Generated macro for parse_doc_string (function)
macro_rules! Depcrateparse_doc_string {
() => {
// Module: crate
// Provides: {"parse_doc_string"}
// Dependencies: {}
# [doc = " Parses a doc string that follows `#[doc = \"`."] fn parse_doc_string (text : & str) -> String { let escaped = text . strip_suffix ("]") . unwrap_or (text) ; let escaped = escaped . strip_suffix (")") . unwrap_or (escaped) . strip_suffix ("\"") ; let Some (escaped) = escaped else { panic ! ("Cannot extract docstring content from {text}") ; } ; let mut buf = String :: new () ; unescape_str (escaped , | _ , res | match res { Ok (c) => buf . push (c) , Err (err) => { assert ! (! err . is_fatal () , "failed to unescape string literal") } }) ; buf }
};
}
