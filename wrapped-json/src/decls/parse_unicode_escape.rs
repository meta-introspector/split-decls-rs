macro_rules! deps {
    () => {
        Read!();
        ErrorCode!();
        Result!();
    };
}

macro_rules! parse_unicode_escape {
    () => {
        deps!();
        # [doc = " Parses a JSON \\u escape and appends it into the scratch space. Assumes `\\u`"] # [doc = " has just been read."] # [cold] fn parse_unicode_escape < 'de , R : Read < 'de > > (read : & mut R , validate : bool , scratch : & mut Vec < u8 > ,) -> Result < () > { let mut n = tri ! (read . decode_hex_escape ()) ; if validate && n >= 0xDC00 && n <= 0xDFFF { return error (read , ErrorCode :: LoneLeadingSurrogateInHexEscape) ; } loop { if n < 0xD800 || n > 0xDBFF { push_wtf8_codepoint (n as u32 , scratch) ; return Ok (()) ; } let n1 = n ; if tri ! (peek_or_eof (read)) == b'\\' { read . discard () ; } else { return if validate { read . discard () ; error (read , ErrorCode :: UnexpectedEndOfHexEscape) } else { push_wtf8_codepoint (n1 as u32 , scratch) ; Ok (()) } ; } if tri ! (peek_or_eof (read)) == b'u' { read . discard () ; } else { return if validate { read . discard () ; error (read , ErrorCode :: UnexpectedEndOfHexEscape) } else { push_wtf8_codepoint (n1 as u32 , scratch) ; parse_escape (read , validate , scratch) } ; } let n2 = tri ! (read . decode_hex_escape ()) ; if n2 < 0xDC00 || n2 > 0xDFFF { if validate { return error (read , ErrorCode :: LoneLeadingSurrogateInHexEscape) ; } push_wtf8_codepoint (n1 as u32 , scratch) ; n = n2 ; continue ; } let n = ((((n1 - 0xD800) as u32) << 10) | (n2 - 0xDC00) as u32) + 0x1_0000 ; push_wtf8_codepoint (n , scratch) ; return Ok (()) ; } }
    };
}

parse_unicode_escape!()