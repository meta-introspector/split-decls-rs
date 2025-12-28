macro_rules! is_hex {
    () => {
        # [doc = " Returns true if the given character is a hexadecimal digit."] fn is_hex (c : char) -> bool { ('0' <= c && c <= '9') || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F') }
    };
}

is_hex!();