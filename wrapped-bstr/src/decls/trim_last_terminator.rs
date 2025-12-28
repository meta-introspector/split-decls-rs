macro_rules! trim_last_terminator {
    () => {
        fn trim_last_terminator (mut s : & [u8]) -> & [u8] { if s . last_byte () == Some (b'\n') { s = & s [.. s . len () - 1] ; if s . last_byte () == Some (b'\r') { s = & s [.. s . len () - 1] ; } } s }
    };
}

trim_last_terminator!()