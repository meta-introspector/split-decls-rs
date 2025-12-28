macro_rules! Ident {
    () => {
        struct Ident < 's > { # [doc = " ASCII part of the identifier."] ascii : & 's str , # [doc = " Punycode insertion codes for Unicode codepoints, if any."] punycode : & 's str , }
    };
}

Ident!()