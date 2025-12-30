// Generated macro for tests (module)
macro_rules! Depcrate_drivertests {
() => {
// Module: crate::driver
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use rcdom :: RcDom ; use serialize :: serialize ; use std :: iter :: repeat ; use tendril :: TendrilSink ; use super :: * ; # [test] fn from_utf8 () { assert_serialization (parse_document (RcDom :: default () , ParseOpts :: default ()) . from_utf8 () . one ("<title>Test" . as_bytes ())) ; } # [test] fn from_bytes_one () { assert_serialization (parse_document (RcDom :: default () , ParseOpts :: default ()) . from_bytes (BytesOpts :: default ()) . one ("<title>Test" . as_bytes ())) ; } # [test] fn from_bytes_iter () { assert_serialization (parse_document (RcDom :: default () , ParseOpts :: default ()) . from_bytes (BytesOpts :: default ()) . from_iter (["<title>Test" . as_bytes () , repeat (' ') . take (1200) . collect :: < String > () . as_bytes () ,] . iter () . cloned ())) ; } fn assert_serialization (dom : RcDom) { let mut serialized = Vec :: new () ; serialize (& mut serialized , & dom . document , Default :: default ()) . unwrap () ; assert_eq ! (String :: from_utf8 (serialized) . unwrap () . replace (" " , "") , "<html><head><title>Test</title></head><body></body></html>") ; } }
};
}
