macro_rules! jis0208_range_decode {
    () => {
        # [inline (always)] pub fn jis0208_range_decode (pointer : usize) -> Option < u16 > { let mut i = 0 ; while i < JIS0208_RANGE_TRIPLES . len () { let start = JIS0208_RANGE_TRIPLES [i] as usize ; let length = JIS0208_RANGE_TRIPLES [i + 1] as usize ; let pointer_minus_start = pointer . wrapping_sub (start) ; if pointer_minus_start < length { let offset = JIS0208_RANGE_TRIPLES [i + 2] as usize ; return Some ((pointer_minus_start + offset) as u16) ; } i += 3 ; } None }
    };
}

jis0208_range_decode!();