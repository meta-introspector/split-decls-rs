macro_rules! hex_encode_fallback {
    () => {
        # [inline (always)] fn hex_encode_fallback < const UPPER : bool > (src : & [u8] , dst : & mut [u8]) { if dst . len () < src . len () * 2 { unsafe { core :: hint :: unreachable_unchecked () } ; } let alphabet = match UPPER { true => b"0123456789ABCDEF" , false => b"0123456789abcdef" , } ; dst . chunks_exact_mut (2) . zip (src) . for_each (| (s , c) | { s [0] = alphabet [(c >> 4) as usize] ; s [1] = alphabet [(c & 0xF) as usize] ; }) ; }
    };
}

hex_encode_fallback!();