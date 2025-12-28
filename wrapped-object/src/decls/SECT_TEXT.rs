macro_rules! SECT_TEXT {
    () => {
        # [doc = " the real text part of the text section no headers, and no padding"] pub const SECT_TEXT : & str = "__text" ;
    };
}

SECT_TEXT!();