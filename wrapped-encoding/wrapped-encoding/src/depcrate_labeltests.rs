// Generated macro for tests (module)
macro_rules! Depcrate_labeltests {
() => {
// Module: crate::label
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { extern crate test ; use all ; use super :: encoding_from_whatwg_label ; # [test] fn test_encoding_from_whatwg_label () { assert ! (encoding_from_whatwg_label ("utf-8") . is_some ()) ; assert ! (encoding_from_whatwg_label ("UTF-8") . is_some ()) ; assert ! (encoding_from_whatwg_label ("\t\n\x0C\r utf-8\t\n\x0C\r ") . is_some ()) ; assert ! (encoding_from_whatwg_label ("\u{A0}utf-8") . is_none () , "Non-ASCII whitespace should not be trimmed") ; assert ! (encoding_from_whatwg_label ("greek") . is_some ()) ; assert ! (encoding_from_whatwg_label ("gree\u{212A}") . is_none () , "Case-insensitive matching should be ASCII only. Kelvin sign does not match k.") ; for encoding in all :: encodings () { if let Some (whatwg_name) = encoding . whatwg_name () { if whatwg_name == "replacement" { continue ; } assert_eq ! (encoding_from_whatwg_label (whatwg_name) . and_then (| e | e . whatwg_name ()) , Some (whatwg_name)) ; } } } # [bench] fn bench_encoding_from_whatwg_label (bencher : & mut test :: Bencher) { bencher . iter (| | test :: black_box ({ encoding_from_whatwg_label ("iso-8859-bazinga") })) } }
};
}
