// Generated macro for tests (module)
macro_rules! Depcrate_common_content_typetests {
() => {
// Module: crate::common::content_type
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: test_decode ; use super :: ContentType ; # [test] fn json () { assert_eq ! (test_decode ::< ContentType > (& ["application/json"]) , Some (ContentType :: json ()) ,) ; } # [test] fn from_str () { assert_eq ! ("application/json" . parse ::< ContentType > () . unwrap () , ContentType :: json () ,) ; assert ! ("invalid-mimetype" . parse ::< ContentType > () . is_err ()) ; } bench_header ! (bench_plain , ContentType , "text/plain") ; bench_header ! (bench_json , ContentType , "application/json") ; bench_header ! (bench_formdata , ContentType , "multipart/form-data; boundary=---------------abcd") ; }
};
}
