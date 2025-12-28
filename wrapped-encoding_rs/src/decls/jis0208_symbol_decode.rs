macro_rules! jis0208_symbol_decode {
    () => {
        pub fn jis0208_symbol_decode (pointer : usize) -> Option < u16 > { let mut i = 0 ; while i < JIS0208_SYMBOL_TRIPLES . len () { let start = JIS0208_SYMBOL_TRIPLES [i] as usize ; let length = JIS0208_SYMBOL_TRIPLES [i + 1] as usize ; let pointer_minus_start = pointer . wrapping_sub (start) ; if pointer_minus_start < length { let offset = JIS0208_SYMBOL_TRIPLES [i + 2] as usize ; return Some (JIS0208_SYMBOLS [pointer_minus_start + offset]) ; } i += 3 ; } None }
    };
}

jis0208_symbol_decode!();