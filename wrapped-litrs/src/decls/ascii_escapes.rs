macro_rules! ascii_escapes {
    () => {
        # [test] fn ascii_escapes () { check ! ('\n') ; check ! ('\r') ; check ! ('\t') ; check ! ('\\') ; check ! ('\0') ; check ! ('\x00') ; check ! ('\x01') ; check ! ('\x0c') ; check ! ('\x0D') ; check ! ('\x13') ; check ! ('\x30') ; check ! ('\x30') ; check ! ('\x4B') ; check ! ('\x6b') ; check ! ('\x7F') ; check ! ('\x7f') ; }
    };
}

ascii_escapes!();