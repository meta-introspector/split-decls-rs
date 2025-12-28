macro_rules! jis0208_symbol_encode {
    () => {
        # [doc = " Prefers Shift_JIS pointers for the three symbols that are in both ranges."] # [inline (always)] pub fn jis0208_symbol_encode (bmp : u16) -> Option < usize > { let mut i = 0 ; while i < JIS0208_SYMBOL_TRIPLES . len () { let pointer_start = JIS0208_SYMBOL_TRIPLES [i] as usize ; let length = JIS0208_SYMBOL_TRIPLES [i + 1] as usize ; let symbol_start = JIS0208_SYMBOL_TRIPLES [i + 2] as usize ; let symbol_end = symbol_start + length ; let mut symbol_pos = symbol_start ; while symbol_pos < symbol_end { if JIS0208_SYMBOLS [symbol_pos] == bmp { return Some (symbol_pos - symbol_start + pointer_start) ; } symbol_pos += 1 ; } i += 3 ; } None }
    };
}

jis0208_symbol_encode!();