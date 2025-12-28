macro_rules! deps {
    () => {
        ParseErrorKind!();
    };
}

macro_rules! check_suffix {
    () => {
        deps!();
        # [doc = " Makes sure that `s` is a valid literal suffix."] pub (crate) fn check_suffix (s : & str) -> Result < () , ParseErrorKind > { if s . is_empty () { return Ok (()) ; } let mut chars = s . chars () ; let first = chars . next () . unwrap () ; let rest = chars . as_str () ; if first == '_' && rest . is_empty () { return Err (InvalidSuffix) ; } if first . is_ascii () && ! (first . is_ascii_alphabetic () || first == '_') { return Err (UnexpectedChar) ; } # [cfg (feature = "check_suffix")] fn is_valid_suffix (first : char , rest : & str) -> bool { use unicode_xid :: UnicodeXID ; (first == '_' || first . is_xid_start ()) && rest . chars () . all (| c | c . is_xid_continue ()) } # [cfg (not (feature = "check_suffix"))] fn is_valid_suffix (first : char , rest : & str) -> bool { if first . is_ascii () && ! (first . is_ascii_alphabetic () || first == '_') { return false ; } for c in rest . chars () { if c . is_ascii () && ! (c . is_ascii_alphanumeric () || c == '_') { return false ; } } true } if is_valid_suffix (first , rest) { Ok (()) } else { Err (InvalidSuffix) } }
    };
}

check_suffix!()