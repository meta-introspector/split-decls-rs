macro_rules! hex_encode {
    () => {
        fn hex_encode (data : & [u8]) -> String { let mut hex_string = String :: with_capacity (data . len () * 2) ; for byte in data . iter () { write ! (& mut hex_string , "{byte:02x}") . unwrap () ; } hex_string }
    };
}

hex_encode!();