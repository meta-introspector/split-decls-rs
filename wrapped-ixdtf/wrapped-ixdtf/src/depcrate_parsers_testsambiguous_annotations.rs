// Generated macro for ambiguous_annotations (function)
macro_rules! Depcrate_parsers_testsambiguous_annotations {
() => {
// Module: crate::parsers::tests
// Provides: {"ambiguous_annotations"}
// Dependencies: {}
# [test] fn ambiguous_annotations () { const TESTS_TIMEZONE : & [& str] = & ["2020-01-01[Asia/Kolkata]" , "2020-01-01[asia/kolkata]" , "2020-01-01[cet]" , "2021-01-29 02:12:48+01:00:00[u][u-ca=iso8601]" ,] ; const TESTS_ANNOTATIONS : & [& str] = & ["2020-01-01[u-ca=foo]" , "2020-01-01[c-et=foo]" , "2020-01-01[cet=foo]" ,] ; for test in TESTS_TIMEZONE { let result = IxdtfParser :: from_str (test) . parse () . expect (test) ; assert ! (result . tz . is_some ()) ; } for test in TESTS_ANNOTATIONS { let result = IxdtfParser :: from_str (test) . parse () . expect (test) ; assert ! (result . tz . is_none ()) ; } }
};
}
