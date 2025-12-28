macro_rules! deps {
    () => {
        ParseResult!();
    };
}

macro_rules! timezone_offset_2822 {
    () => {
        deps!();
        # [doc = " Same as `timezone_offset` but also allows for RFC 2822 legacy timezones."] # [doc = " May return `None` which indicates an insufficient offset data (i.e. `-0000`)."] # [doc = " See [RFC 2822 Section 4.3]."] # [doc = ""] # [doc = " [RFC 2822 Section 4.3]: https://tools.ietf.org/html/rfc2822#section-4.3"] pub (super) fn timezone_offset_2822 (s : & str) -> ParseResult < (& str , i32) > { let upto = s . as_bytes () . iter () . position (| & c | ! c . is_ascii_alphabetic ()) . unwrap_or (s . len ()) ; if upto > 0 { let name = & s . as_bytes () [.. upto] ; let s = & s [upto ..] ; let offset_hours = | o | Ok ((s , o * 3600)) ; if name . eq_ignore_ascii_case (b"gmt") || name . eq_ignore_ascii_case (b"ut") || name . eq_ignore_ascii_case (b"z") { return offset_hours (0) ; } else if name . eq_ignore_ascii_case (b"edt") { return offset_hours (- 4) ; } else if name . eq_ignore_ascii_case (b"est") || name . eq_ignore_ascii_case (b"cdt") { return offset_hours (- 5) ; } else if name . eq_ignore_ascii_case (b"cst") || name . eq_ignore_ascii_case (b"mdt") { return offset_hours (- 6) ; } else if name . eq_ignore_ascii_case (b"mst") || name . eq_ignore_ascii_case (b"pdt") { return offset_hours (- 7) ; } else if name . eq_ignore_ascii_case (b"pst") { return offset_hours (- 8) ; } else if name . len () == 1 { if let b'a' ..= b'i' | b'k' ..= b'y' | b'A' ..= b'I' | b'K' ..= b'Y' = name [0] { return Ok ((s , 0)) ; } } Err (INVALID) } else { timezone_offset (s , | s | Ok (s) , false , false , false) } }
    };
}

timezone_offset_2822!();