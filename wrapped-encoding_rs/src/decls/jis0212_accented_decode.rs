macro_rules! jis0212_accented_decode {
    () => {
        pub fn jis0212_accented_decode (pointer : usize) -> Option < u16 > { let mut i = 0 ; while i < JIS0212_ACCENTED_TRIPLES . len () { let start = JIS0212_ACCENTED_TRIPLES [i] as usize ; let length = JIS0212_ACCENTED_TRIPLES [i + 1] as usize ; let pointer_minus_start = pointer . wrapping_sub (start) ; if pointer_minus_start < length { let offset = JIS0212_ACCENTED_TRIPLES [i + 2] as usize ; let candidate = JIS0212_ACCENTED [pointer_minus_start + offset] ; if candidate == 0 { return None ; } return Some (candidate) ; } i += 3 ; } None }
    };
}

jis0212_accented_decode!();