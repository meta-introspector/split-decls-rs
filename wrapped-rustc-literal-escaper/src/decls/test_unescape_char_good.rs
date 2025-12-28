macro_rules! test_unescape_char_good {
    () => {
        # [test] fn test_unescape_char_good () { fn check (literal_text : & str , expected_char : char) { assert_eq ! (unescape_char (literal_text) , Ok (expected_char)) ; } check ("a" , 'a') ; check ("ы" , 'ы') ; check ("🦀" , '🦀') ; check (r#"\""# , '"') ; check (r"\n" , '\n') ; check (r"\r" , '\r') ; check (r"\t" , '\t') ; check (r"\\" , '\\') ; check (r"\'" , '\'') ; check (r"\0" , '\0') ; check (r"\x00" , '\0') ; check (r"\x5a" , 'Z') ; check (r"\x5A" , 'Z') ; check (r"\x7f" , 127 as char) ; check (r"\u{0}" , '\0') ; check (r"\u{000000}" , '\0') ; check (r"\u{41}" , 'A') ; check (r"\u{0041}" , 'A') ; check (r"\u{00_41}" , 'A') ; check (r"\u{4__1__}" , 'A') ; check (r"\u{1F63b}" , '😻') ; }
    };
}

test_unescape_char_good!();