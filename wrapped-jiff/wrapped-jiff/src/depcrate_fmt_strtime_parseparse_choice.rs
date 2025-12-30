// Generated macro for parse_choice (function)
macro_rules! Depcrate_fmt_strtime_parseparse_choice {
() => {
// Module: crate::fmt::strtime::parse
// Provides: {"parse_choice"}
// Dependencies: {}
# [doc = " Parses the input such that, on success, the index of the first matching"] # [doc = " choice (via ASCII case insensitive comparisons) is returned, along with"] # [doc = " any remaining unparsed input."] # [doc = ""] # [doc = " If no choice given is a prefix of the input, then an error is returned."] # [doc = " The error includes the possible allowed choices."] fn parse_choice < 'i > (input : & 'i [u8] , choices : & [& 'static [u8]] ,) -> Result < (usize , & 'i [u8]) , Error > { for (i , choice) in choices . into_iter () . enumerate () { if input . len () < choice . len () { continue ; } let (candidate , input) = input . split_at (choice . len ()) ; if candidate . eq_ignore_ascii_case (choice) { return Ok ((i , input)) ; } } # [cfg (feature = "alloc")] { let mut err = alloc :: format ! ("failed to find expected choice at beginning of {input:?}, \
             available choices are: " , input = escape :: Bytes (input) ,) ; for (i , choice) in choices . iter () . enumerate () { if i > 0 { write ! (err , ", ") . unwrap () ; } write ! (err , "{}" , escape :: Bytes (choice)) . unwrap () ; } Err (Error :: adhoc (err)) } # [cfg (not (feature = "alloc"))] { Err (err ! ("failed to find expected value from a set of allowed choices")) } }
};
}
