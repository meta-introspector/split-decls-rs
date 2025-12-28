macro_rules! hex_encode {
    () => {
        # [inline] fn hex_encode < const UPPER : bool > (src : & [u8] , dst : & mut [u8]) { debug_assert ! (dst . len () >= (src . len () * 2)) ; # [cfg (any (miri , not (feature = "faster-hex")))] hex_encode_fallback :: < UPPER > (src , dst) ; # [cfg (all (feature = "faster-hex" , not (miri)))] match UPPER { true => unsafe { faster_hex :: hex_encode_upper (src , dst) . unwrap_unchecked () } , false => unsafe { faster_hex :: hex_encode (src , dst) . unwrap_unchecked () } , } ; }
    };
}

hex_encode!()