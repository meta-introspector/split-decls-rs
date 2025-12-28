macro_rules! deps {
    () => {
        Read!();
        Result!();
        ErrorCode!();
    };
}

macro_rules! ignore_escape {
    () => {
        deps!();
        # [doc = " Parses a JSON escape sequence and discards the value. Assumes the previous"] # [doc = " byte read was a backslash."] fn ignore_escape < 'de , R > (read : & mut R) -> Result < () > where R : ? Sized + Read < 'de > , { let ch = tri ! (next_or_eof (read)) ; match ch { b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => { } b'u' => { tri ! (read . decode_hex_escape ()) ; } _ => { return error (read , ErrorCode :: InvalidEscape) ; } } Ok (()) }
    };
}

ignore_escape!();