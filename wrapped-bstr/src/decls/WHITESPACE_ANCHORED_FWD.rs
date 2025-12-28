macro_rules! WHITESPACE_ANCHORED_FWD {
    () => {
        pub static WHITESPACE_ANCHORED_FWD : Lazy < DFA < & 'static [u32] > > = Lazy :: new (| | { static ALIGNED : & AlignAs < [u8] , u32 > = & AlignAs { _align : [] , # [cfg (target_endian = "big")] bytes : * include_bytes ! ("whitespace_anchored_fwd.bigendian.dfa") , # [cfg (target_endian = "little")] bytes : * include_bytes ! ("whitespace_anchored_fwd.littleendian.dfa") , } ; let (dfa , _) = DFA :: from_bytes (& ALIGNED . bytes) . expect ("serialized DFA should be valid") ; dfa }) ;
    };
}

WHITESPACE_ANCHORED_FWD!();