macro_rules! GRAPHEME_BREAK_REV {
    () => {
        pub static GRAPHEME_BREAK_REV : Lazy < DFA < & 'static [u8] > > = Lazy :: new (| | { # [cfg (target_endian = "big")] static BYTES : & 'static [u8] = include_bytes ! ("grapheme_break_rev.bigendian.dfa") ; # [cfg (target_endian = "little")] static BYTES : & 'static [u8] = include_bytes ! ("grapheme_break_rev.littleendian.dfa") ; let (dfa , _) = DFA :: from_bytes (BYTES) . expect ("serialized DFA should be valid") ; dfa }) ;
    };
}

GRAPHEME_BREAK_REV!()