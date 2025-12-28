macro_rules! deps {
    () => {
        StripStr!();
        StrippedStr!();
    };
}

macro_rules! strip_str {
    () => {
        deps!();
        # [doc = " Strip ANSI escapes from a `&str`, returning the printable content"] # [doc = ""] # [doc = " This can be used to take output from a program that includes escape sequences and write it"] # [doc = " somewhere that does not easily support them, such as a log file."] # [doc = ""] # [doc = " For non-contiguous data, see [`StripStr`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::io::Write as _;"] # [doc = ""] # [doc = " let styled_text = \"\\x1b[32mfoo\\x1b[m bar\";"] # [doc = " let plain_str = anstream::adapter::strip_str(&styled_text).to_string();"] # [doc = " assert_eq!(plain_str, \"foo bar\");"] # [doc = " ```"] # [inline] pub fn strip_str (data : & str) -> StrippedStr < '_ > { StrippedStr :: new (data) }
    };
}

strip_str!()