macro_rules! find_invalid_byte {
    () => {
        const fn find_invalid_byte (bytes : & [u8]) -> Option < u8 > { let mut i = 0 ; while i < bytes . len () { let b = bytes [i] ; if ! is_valid_byte (b) { return Some (b) ; } i += 1 ; } None }
    };
}

find_invalid_byte!();