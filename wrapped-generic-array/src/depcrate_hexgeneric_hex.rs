// Generated macro for generic_hex (function)
macro_rules! Depcrate_hexgeneric_hex {
() => {
// Module: crate::hex
// Provides: {"generic_hex"}
// Dependencies: {}
fn generic_hex < N , const UPPER : bool > (arr : & GenericArray < u8 , N > , f : & mut fmt :: Formatter < '_ > ,) -> fmt :: Result where N : ArrayLength + Add < N > , Sum < N , N > : ArrayLength , { let max_digits = N :: USIZE * 2 ; let max_digits = match f . precision () { Some (precision) if precision < max_digits => precision , _ => max_digits , } ; let max_bytes = (max_digits >> 1) + (max_digits & 1) ; let input = { if max_bytes > N :: USIZE { unsafe { core :: hint :: unreachable_unchecked () } ; } & arr [.. max_bytes] } ; if N :: USIZE <= 1024 { let mut buf = GenericArray :: < u8 , Sum < N , N > > :: default () ; if N :: USIZE < 16 { hex_encode_fallback :: < UPPER > (arr , & mut buf) ; } else { hex_encode :: < UPPER > (input , & mut buf) ; } f . write_str (unsafe { str :: from_utf8_unchecked (buf . get_unchecked (.. max_digits)) }) ? ; } else { let mut buf = [0u8 ; 2048] ; let mut digits_left = max_digits ; for chunk in input . chunks (1024) { hex_encode :: < UPPER > (chunk , & mut buf) ; let n = min (chunk . len () * 2 , digits_left) ; f . write_str (unsafe { str :: from_utf8_unchecked (buf . get_unchecked (.. n)) }) ? ; digits_left -= n ; } } Ok (()) }
};
}
