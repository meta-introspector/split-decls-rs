macro_rules! trim_line {
    () => {
        fn trim_line (line : & mut Vec < u8 >) { if line . last_byte () == Some (b'\n') { line . pop_byte () ; if line . last_byte () == Some (b'\r') { line . pop_byte () ; } } }
    };
}

trim_line!();