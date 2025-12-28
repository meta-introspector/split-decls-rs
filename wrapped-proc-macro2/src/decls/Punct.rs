macro_rules! deps {
    () => {
        Spacing!();
        Span!();
    };
}

macro_rules! Punct {
    () => {
        deps!();
        # [doc = " A `Punct` is a single punctuation character like `+`, `-` or `#`."] # [doc = ""] # [doc = " Multicharacter operators like `+=` are represented as two instances of"] # [doc = " `Punct` with different forms of `Spacing` returned."] # [derive (Clone)] pub struct Punct { ch : char , spacing : Spacing , span : Span , }
    };
}

Punct!()