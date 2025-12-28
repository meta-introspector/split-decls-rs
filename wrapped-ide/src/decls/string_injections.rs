macro_rules! deps {
    () => {
        HighlightConfig!();
    };
}

macro_rules! string_injections {
    () => {
        deps!();
        fn string_injections (hl : & mut Highlights , sema : & Semantics < '_ , RootDatabase > , config : & HighlightConfig < '_ > , file_id : EditionedFileId , krate : Option < hir :: Crate > , token : SyntaxToken , descended_token : & SyntaxToken ,) -> ControlFlow < () > { if ! matches ! (token . kind () , STRING | BYTE_STRING | BYTE | CHAR | C_STRING) { return ControlFlow :: Continue (()) ; } if let Some (string) = ast :: String :: cast (token . clone ()) { if let Some (descended_string) = ast :: String :: cast (descended_token . clone ()) { if string . is_raw () && inject :: ra_fixture (hl , sema , config , & string , & descended_string) . is_some () { return ControlFlow :: Break (()) ; } highlight_format_string (hl , sema , krate , & string , & descended_string , file_id . edition (sema . db) ,) ; if ! string . is_raw () { highlight_escape_string (hl , & string) ; } } } else if let Some (byte_string) = ast :: ByteString :: cast (token . clone ()) { if ! byte_string . is_raw () { highlight_escape_string (hl , & byte_string) ; } } else if let Some (c_string) = ast :: CString :: cast (token . clone ()) { if ! c_string . is_raw () { highlight_escape_string (hl , & c_string) ; } } else if let Some (char) = ast :: Char :: cast (token . clone ()) { highlight_escape_char (hl , & char) } else if let Some (byte) = ast :: Byte :: cast (token) { highlight_escape_byte (hl , & byte) } ControlFlow :: Continue (()) }
    };
}

string_injections!();