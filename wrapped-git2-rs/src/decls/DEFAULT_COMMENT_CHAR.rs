macro_rules! DEFAULT_COMMENT_CHAR {
    () => {
        # [doc = " The default comment character for `message_prettify` ('#')"] pub const DEFAULT_COMMENT_CHAR : Option < u8 > = Some (b'#') ;
    };
}

DEFAULT_COMMENT_CHAR!();