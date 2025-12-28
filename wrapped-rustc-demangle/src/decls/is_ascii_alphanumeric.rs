macro_rules! is_ascii_alphanumeric {
    () => {
        fn is_ascii_alphanumeric (c : char) -> bool { match c { '\u{0041}' ..= '\u{005A}' | '\u{0061}' ..= '\u{007A}' | '\u{0030}' ..= '\u{0039}' => true , _ => false , } }
    };
}

is_ascii_alphanumeric!()