macro_rules! deps {
    () => {
        ChangeSet!();
    };
}

macro_rules! ColorTag {
    () => {
        deps!();
        # [doc = " A parsed color/style tag."] # [derive (Debug , Default)] pub struct ColorTag < 'a > { # [doc = " Source of the tag in the format string."] pub source : Option < & 'a str > , # [doc = " Span of the tag in the format string."] pub span : Option < Span > , # [doc = " Is it a close tag like `</red>`."] pub is_close : bool , # [doc = " The changes that are implied by this tag."] pub change_set : ChangeSet , }
    };
}

ColorTag!()