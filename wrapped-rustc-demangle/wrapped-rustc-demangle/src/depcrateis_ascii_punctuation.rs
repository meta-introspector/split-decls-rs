// Generated macro for is_ascii_punctuation (function)
macro_rules! Depcrateis_ascii_punctuation {
() => {
// Module: crate
// Provides: {"is_ascii_punctuation"}
// Dependencies: {}
fn is_ascii_punctuation (c : char) -> bool { match c { '\u{0021}' ..= '\u{002F}' | '\u{003A}' ..= '\u{0040}' | '\u{005B}' ..= '\u{0060}' | '\u{007B}' ..= '\u{007E}' => true , _ => false , } }
};
}
