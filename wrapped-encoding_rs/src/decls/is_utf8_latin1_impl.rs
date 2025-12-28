macro_rules! is_utf8_latin1_impl {
    () => {
        # [inline (always)] fn is_utf8_latin1_impl (buffer : & [u8]) -> Option < usize > { let mut bytes = buffer ; let mut total = 0 ; loop { if let Some ((byte , offset)) = validate_ascii (bytes) { total += offset ; if in_inclusive_range8 (byte , 0xC2 , 0xC3) { let next = offset + 1 ; if next == bytes . len () { return Some (total) ; } if bytes [next] & 0xC0 != 0x80 { return Some (total) ; } bytes = & bytes [offset + 2 ..] ; total += 2 ; } else { return Some (total) ; } } else { return None ; } } }
    };
}

is_utf8_latin1_impl!();