macro_rules! is_attr {
    () => {
        # [doc = " Returns true if the byte is a valid `attr-char` as defined in"] # [doc = " [RFC 5987 section 3.2.1](https://datatracker.ietf.org/doc/html/rfc5987#section-3.2.1)."] # [doc = ""] # [doc = " ```text"] # [doc = "  attr-char     = ALPHA / DIGIT"] # [doc = "                / \"!\" / \"#\" / \"$\" / \"&\" / \"+\" / \"-\" / \".\""] # [doc = "                / \"^\" / \"_\" / \"`\" / \"|\" / \"~\""] # [doc = "                ; token except ( \"*\" / \"'\" / \"%\" )"] # [doc = " ```"] const fn is_attr (b : u8) -> bool { matches ! (b , b'a' ..= b'z' | b'A' ..= b'Z' | b'0' ..= b'9' | b'!' | b'#' | b'$' | b'&' | b'+' | b'-' | b'.' | b'^' | b'_' | b'`' | b'|' | b'~') }
    };
}

is_attr!();