macro_rules! escape_into {
    () => {
        # [doc = " Escapes all meta characters in `text` and writes the result into `buf`."] # [doc = ""] # [doc = " This will append escape characters into the given buffer. The characters"] # [doc = " that are appended are safe to use as a literal in a regular expression."] pub fn escape_into (text : & str , buf : & mut String) { buf . reserve (text . len ()) ; for c in text . chars () { if is_meta_character (c) { buf . push ('\\') ; } buf . push (c) ; } }
    };
}

escape_into!()