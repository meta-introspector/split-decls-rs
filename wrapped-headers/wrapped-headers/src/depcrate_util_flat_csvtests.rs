// Generated macro for tests (module)
macro_rules! Depcrate_util_flat_csvtests {
() => {
// Module: crate::util::flat_csv
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn comma () { let val = HeaderValue :: from_static ("aaa, b; bb, ccc") ; let csv = FlatCsv :: < Comma > :: from (val) ; let mut values = csv . iter () ; assert_eq ! (values . next () , Some ("aaa")) ; assert_eq ! (values . next () , Some ("b; bb")) ; assert_eq ! (values . next () , Some ("ccc")) ; assert_eq ! (values . next () , None) ; } # [test] fn semicolon () { let val = HeaderValue :: from_static ("aaa; b, bb; ccc") ; let csv = FlatCsv :: < SemiColon > :: from (val) ; let mut values = csv . iter () ; assert_eq ! (values . next () , Some ("aaa")) ; assert_eq ! (values . next () , Some ("b, bb")) ; assert_eq ! (values . next () , Some ("ccc")) ; assert_eq ! (values . next () , None) ; } # [test] fn quoted_text () { let val = HeaderValue :: from_static ("foo=\"bar,baz\", sherlock=holmes") ; let csv = FlatCsv :: < Comma > :: from (val) ; let mut values = csv . iter () ; assert_eq ! (values . next () , Some ("foo=\"bar,baz\"")) ; assert_eq ! (values . next () , Some ("sherlock=holmes")) ; assert_eq ! (values . next () , None) ; } }
};
}
