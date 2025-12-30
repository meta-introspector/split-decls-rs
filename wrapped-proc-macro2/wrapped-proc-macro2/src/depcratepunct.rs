// Generated macro for Punct (struct)
macro_rules! DepcratePunct {
() => {
// Module: crate
// Provides: {"Punct"}
// Dependencies: {}
# [doc = " A `Punct` is a single punctuation character like `+`, `-` or `#`."] # [doc = ""] # [doc = " Multicharacter operators like `+=` are represented as two instances of"] # [doc = " `Punct` with different forms of `Spacing` returned."] # [derive (Clone)] pub struct Punct { ch : char , spacing : Spacing , span : Span , }
};
}
