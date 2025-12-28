macro_rules! trim_line_slice {
    () => {
        fn trim_line_slice (mut line : & [u8]) -> & [u8] { if line . last_byte () == Some (b'\n') { line = & line [.. line . len () - 1] ; if line . last_byte () == Some (b'\r') { line = & line [.. line . len () - 1] ; } } line }
    };
}

trim_line_slice!();