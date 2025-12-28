macro_rules! strip_newline {
    () => {
        fn strip_newline (text : & str) -> & str { text . strip_suffix ("\r\n") . or_else (| | text . strip_suffix ('\n')) . unwrap_or (text) }
    };
}

strip_newline!();