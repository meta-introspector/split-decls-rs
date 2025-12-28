macro_rules! is_tchar {
    () => {
        # [doc = " Returns if the byte is a `tchar` as defined in"] # [doc = " [RFC 7230 section 3.2.6](https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.6)."] const fn is_tchar (b : u8) -> bool { matches ! (b , b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' | b'-' | b'.' | b'^' | b'_' | b'`' | b'|' | b'~' | b'0' ..= b'9' | b'a' ..= b'z' | b'A' ..= b'Z') }
    };
}

is_tchar!();