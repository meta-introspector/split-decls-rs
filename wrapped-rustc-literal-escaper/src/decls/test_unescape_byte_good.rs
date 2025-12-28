macro_rules! test_unescape_byte_good {
    () => {
        # [test] fn test_unescape_byte_good () { fn check (literal_text : & str , expected_byte : u8) { assert_eq ! (unescape_byte (literal_text) , Ok (expected_byte)) ; } check ("a" , b'a') ; check (r#"\""# , b'"') ; check (r"\n" , b'\n') ; check (r"\r" , b'\r') ; check (r"\t" , b'\t') ; check (r"\\" , b'\\') ; check (r"\'" , b'\'') ; check (r"\0" , b'\0') ; check (r"\x00" , b'\0') ; check (r"\x5a" , b'Z') ; check (r"\x5A" , b'Z') ; check (r"\x7f" , 127) ; check (r"\x80" , 128) ; check (r"\xff" , 255) ; check (r"\xFF" , 255) ; }
    };
}

test_unescape_byte_good!();