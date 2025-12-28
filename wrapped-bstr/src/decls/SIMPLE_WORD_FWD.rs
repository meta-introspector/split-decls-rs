macro_rules! SIMPLE_WORD_FWD {
    () => {
        pub static SIMPLE_WORD_FWD : Lazy < DFA < & 'static [u8] > > = Lazy :: new (| | { # [cfg (target_endian = "big")] static BYTES : & 'static [u8] = include_bytes ! ("simple_word_fwd.bigendian.dfa") ; # [cfg (target_endian = "little")] static BYTES : & 'static [u8] = include_bytes ! ("simple_word_fwd.littleendian.dfa") ; let (dfa , _) = DFA :: from_bytes (BYTES) . expect ("serialized DFA should be valid") ; dfa }) ;
    };
}

SIMPLE_WORD_FWD!();