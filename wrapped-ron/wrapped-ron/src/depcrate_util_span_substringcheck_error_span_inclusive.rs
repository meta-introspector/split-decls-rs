// Generated macro for check_error_span_inclusive (function)
macro_rules! Depcrate_util_span_substringcheck_error_span_inclusive {
() => {
// Module: crate::util::span_substring
// Provides: {"check_error_span_inclusive"}
// Dependencies: {}
# [doc = " Given a string `ron`, a [`SpannedResult`], and a substring, verify that trying to parse `ron` results in an error"] # [doc = " equal to the [`SpannedResult`] with a Span that inclusively (as in `[start..=end`]) selects that substring."] # [doc = " See [`check_error_span_exclusive`] for the rationale behind both versions of this helper."] # [allow (clippy :: unwrap_used)] # [allow (clippy :: missing_panics_doc)] pub fn check_error_span_inclusive < T : serde :: de :: DeserializeOwned + PartialEq + core :: fmt :: Debug > (ron : & str , check : SpannedResult < T > , substr : & str ,) { let res_str = crate :: de :: from_str :: < T > (ron) ; assert_eq ! (res_str , check) ; let res_bytes = crate :: de :: from_bytes :: < T > (ron . as_bytes ()) ; assert_eq ! (res_bytes , check) ; # [cfg (feature = "std")] { let res_reader = crate :: de :: from_reader :: < & [u8] , T > (ron . as_bytes ()) ; assert_eq ! (res_reader , check) ; } assert_eq ! (check . unwrap_err () . span . substring_inclusive (ron) . unwrap () , substr) ; }
};
}
