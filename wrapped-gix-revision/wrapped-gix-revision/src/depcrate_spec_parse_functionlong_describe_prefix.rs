// Generated macro for long_describe_prefix (function)
macro_rules! Depcrate_spec_parse_functionlong_describe_prefix {
() => {
// Module: crate::spec::parse::function
// Provides: {"long_describe_prefix"}
// Dependencies: {}
fn long_describe_prefix (name : & BStr) -> Option < (& BStr , delegate :: PrefixHint < '_ >) > { let mut iter = name . rsplit (| b | * b == b'-') ; let candidate = iter . by_ref () . find_map (| substr | { if substr . first () ? != & b'g' { return None ; } let rest = substr . get (1 ..) ? ; rest . iter () . all (u8 :: is_ascii_hexdigit) . then (| | rest . as_bstr ()) }) ? ; let candidate = iter . clone () . any (| token | ! token . is_empty ()) . then_some (candidate) ; let hint = iter . next () . and_then (| gen | gen . to_str () . ok () . and_then (| gen | usize :: from_str (gen) . ok ())) . and_then (| generation | { iter . next () . map (| token | { let last_token_len = token . len () ; let first_token_ptr = iter . next_back () . map_or (token . as_ptr () , < [_] > :: as_ptr) ; # [allow (unsafe_code)] let prior_tokens_len : usize = unsafe { token . as_ptr () . offset_from (first_token_ptr) } . try_into () . expect ("positive value") ; delegate :: PrefixHint :: DescribeAnchor { ref_name : name [.. prior_tokens_len + last_token_len] . as_bstr () , generation , } }) }) . unwrap_or (delegate :: PrefixHint :: MustBeCommit) ; candidate . map (| c | (c , hint)) }
};
}
