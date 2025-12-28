macro_rules! as_substr {
    () => {
        # [doc = " Given an original string like `AACC`, and a suggestion like `AABBCC`, try to detect"] # [doc = " the case where a substring of the suggestion is \"sandwiched\" in the original, like"] # [doc = " `BB` is. Return the length of the prefix, the \"trimmed\" suggestion, and the length"] # [doc = " of the suffix."] pub (crate) fn as_substr < 'a > (original : & 'a str , suggestion : & 'a str ,) -> Option < (usize , & 'a str , usize) > { let common_prefix = original . chars () . zip (suggestion . chars ()) . take_while (| (c1 , c2) | c1 == c2) . map (| (c , _) | c . len_utf8 ()) . sum () ; let original = & original [common_prefix ..] ; let suggestion = & suggestion [common_prefix ..] ; if let Some (stripped) = suggestion . strip_suffix (original) { let common_suffix = original . len () ; Some ((common_prefix , stripped , common_suffix)) } else { None } }
    };
}

as_substr!();