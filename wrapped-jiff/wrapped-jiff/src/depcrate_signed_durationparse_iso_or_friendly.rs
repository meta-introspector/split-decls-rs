// Generated macro for parse_iso_or_friendly (function)
macro_rules! Depcrate_signed_durationparse_iso_or_friendly {
() => {
// Module: crate::signed_duration
// Provides: {"parse_iso_or_friendly"}
// Dependencies: {}
# [doc = " A common parsing function that works in bytes."] # [doc = ""] # [doc = " Specifically, this parses either an ISO 8601 duration into a"] # [doc = " `SignedDuration` or a \"friendly\" duration into a `SignedDuration`. It also"] # [doc = " tries to give decent error messages."] # [doc = ""] # [doc = " This works because the friendly and ISO 8601 formats have non-overlapping"] # [doc = " prefixes. Both can start with a `+` or `-`, but aside from that, an ISO"] # [doc = " 8601 duration _always_ has to start with a `P` or `p`. We can utilize this"] # [doc = " property to very quickly determine how to parse the input. We just need to"] # [doc = " handle the possibly ambiguous case with a leading sign a little carefully"] # [doc = " in order to ensure good error messages."] # [doc = ""] # [doc = " (We do the same thing for `Span`.)"] # [cfg_attr (feature = "perf-inline" , inline (always))] fn parse_iso_or_friendly (bytes : & [u8]) -> Result < SignedDuration , Error > { if bytes . is_empty () { return Err (err ! ("an empty string is not a valid `SignedDuration`, \
             expected either a ISO 8601 or Jiff's 'friendly' \
             format" ,)) ; } let mut first = bytes [0] ; if first == b'+' || first == b'-' { if bytes . len () == 1 { return Err (err ! ("found nothing after sign `{sign}`, \
                 which is not a valid `SignedDuration`, \
                 expected either a ISO 8601 or Jiff's 'friendly' \
                 format" , sign = escape :: Byte (first) ,)) ; } first = bytes [1] ; } if first == b'P' || first == b'p' { temporal :: DEFAULT_SPAN_PARSER . parse_duration (bytes) } else { friendly :: DEFAULT_SPAN_PARSER . parse_duration (bytes) } }
};
}
