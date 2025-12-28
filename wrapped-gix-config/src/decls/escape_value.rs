macro_rules! escape_value {
    () => {
        fn escape_value (value : & BStr) -> BString { let starts_with_whitespace = value . first () . is_some_and (u8 :: is_ascii_whitespace) ; let ends_with_whitespace = value . get (value . len () . saturating_sub (1)) . is_some_and (u8 :: is_ascii_whitespace) ; let contains_comment_indicators = value . find_byteset (b";#") . is_some () ; let quote = starts_with_whitespace || ends_with_whitespace || contains_comment_indicators ; let mut buf : BString = Vec :: with_capacity (value . len ()) . into () ; if quote { buf . push (b'"') ; } for b in value . iter () . copied () { match b { b'\n' => buf . push_str (r"\n") , b'\t' => buf . push_str (r"\t") , b'"' => buf . push_str (r#"\""#) , b'\\' => buf . push_str (r"\\") , _ => buf . push (b) , } } if quote { buf . push (b'"') ; } buf }
    };
}

escape_value!();