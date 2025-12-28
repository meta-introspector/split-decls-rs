macro_rules! is_ascii_punctuation {
    () => {
        fn is_ascii_punctuation (c : char) -> bool { match c { '\u{0021}' ..= '\u{002F}' | '\u{003A}' ..= '\u{0040}' | '\u{005B}' ..= '\u{0060}' | '\u{007B}' ..= '\u{007E}' => true , _ => false , } }
    };
}

is_ascii_punctuation!()