macro_rules! deps {
    () => {
        ParseResult!();
    };
}

macro_rules! timezone_offset {
    () => {
        deps!();
        # [doc = " Parse a timezone from `s` and return the offset in seconds."] # [doc = ""] # [doc = " The `consume_colon` function is used to parse a mandatory or optional `:`"] # [doc = " separator between hours offset and minutes offset."] # [doc = ""] # [doc = " The `allow_missing_minutes` flag allows the timezone minutes offset to be"] # [doc = " missing from `s`."] # [doc = ""] # [doc = " The `allow_tz_minus_sign` flag allows the timezone offset negative character"] # [doc = " to also be `−` MINUS SIGN (U+2212) in addition to the typical"] # [doc = " ASCII-compatible `-` HYPHEN-MINUS (U+2D)."] # [doc = " This is part of [RFC 3339 & ISO 8601]."] # [doc = ""] # [doc = " [RFC 3339 & ISO 8601]: https://en.wikipedia.org/w/index.php?title=ISO_8601&oldid=1114309368#Time_offsets_from_UTC"] pub (crate) fn timezone_offset < F > (mut s : & str , mut consume_colon : F , allow_zulu : bool , allow_missing_minutes : bool , allow_tz_minus_sign : bool ,) -> ParseResult < (& str , i32) > where F : FnMut (& str) -> ParseResult < & str > , { if allow_zulu { if let Some (& b'Z' | & b'z') = s . as_bytes () . first () { return Ok ((& s [1 ..] , 0)) ; } } const fn digits (s : & str) -> ParseResult < (u8 , u8) > { let b = s . as_bytes () ; if b . len () < 2 { Err (TOO_SHORT) } else { Ok ((b [0] , b [1])) } } let negative = match s . chars () . next () { Some ('+') => { s = & s ['+' . len_utf8 () ..] ; false } Some ('-') => { s = & s ['-' . len_utf8 () ..] ; true } Some ('−') => { if ! allow_tz_minus_sign { return Err (INVALID) ; } s = & s ['−' . len_utf8 () ..] ; true } Some (_) => return Err (INVALID) , None => return Err (TOO_SHORT) , } ; let hours = match digits (s) ? { (h1 @ b'0' ..= b'9' , h2 @ b'0' ..= b'9') => i32 :: from ((h1 - b'0') * 10 + (h2 - b'0')) , _ => return Err (INVALID) , } ; s = & s [2 ..] ; s = consume_colon (s) ? ; let minutes = if let Ok (ds) = digits (s) { match ds { (m1 @ b'0' ..= b'5' , m2 @ b'0' ..= b'9') => i32 :: from ((m1 - b'0') * 10 + (m2 - b'0')) , (b'6' ..= b'9' , b'0' ..= b'9') => return Err (OUT_OF_RANGE) , _ => return Err (INVALID) , } } else if allow_missing_minutes { 0 } else { return Err (TOO_SHORT) ; } ; s = match s . len () { len if len >= 2 => & s [2 ..] , 0 => s , _ => return Err (TOO_SHORT) , } ; let seconds = hours * 3600 + minutes * 60 ; Ok ((s , if negative { - seconds } else { seconds })) }
    };
}

timezone_offset!();