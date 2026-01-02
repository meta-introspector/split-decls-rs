mkitem!{mkstruct!{# [derive (Copy , Clone)] pub (crate) struct EscapeOptions { # [doc = " Produce \\'."] pub escape_single_quote : bool , # [doc = " Produce \\\"."] pub escape_double_quote : bool , # [doc = " Produce \\x escapes for non-ASCII, and use \\x rather than \\u for ASCII"] # [doc = " control characters."] pub escape_nonascii : bool , }}}

macro_rules! escape_bytes_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function escape_bytes in module {}", module_path!());
    };
}

mkfn!{
    escape_bytes_introspect!();
    pub (crate) fn escape_bytes (bytes : & [u8] , opt : EscapeOptions) -> String { let mut repr = String :: new () ; if opt . escape_nonascii { for & byte in bytes { escape_single_byte (byte , opt , & mut repr) ; } } else { let mut chunks = bytes . utf8_chunks () ; while let Some (chunk) = chunks . next () { for ch in chunk . valid () . chars () { escape_single_char (ch , opt , & mut repr) ; } for & byte in chunk . invalid () { escape_single_byte (byte , opt , & mut repr) ; } } } repr }
}

macro_rules! escape_single_byte_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function escape_single_byte in module {}", module_path!());
    };
}

mkfn!{
    escape_single_byte_introspect!();
    fn escape_single_byte (byte : u8 , opt : EscapeOptions , repr : & mut String) { if byte == b'\0' { repr . push_str ("\\0") ; } else if (byte == b'\'' && ! opt . escape_single_quote) || (byte == b'"' && ! opt . escape_double_quote) { repr . push (byte as char) ; } else { repr . extend (byte . escape_ascii () . map (char :: from)) ; } }
}

macro_rules! escape_single_char_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function escape_single_char in module {}", module_path!());
    };
}

mkfn!{
    escape_single_char_introspect!();
    fn escape_single_char (ch : char , opt : EscapeOptions , repr : & mut String) { if (ch == '\'' && ! opt . escape_single_quote) || (ch == '"' && ! opt . escape_double_quote) { repr . push (ch) ; } else { repr . extend (ch . escape_debug ()) ; } }
}