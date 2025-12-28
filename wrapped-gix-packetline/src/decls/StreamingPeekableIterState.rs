macro_rules! deps {
    () => {
        PacketLineRef!();
    };
}

macro_rules! StreamingPeekableIterState {
    () => {
        deps!();
        # [doc = " State for `StreamingPeekableIter` implementations."] pub struct StreamingPeekableIterState < T > { pub (crate) read : T , pub (crate) peek_buf : Vec < u8 > , # [cfg (any (feature = "blocking-io" , feature = "async-io"))] pub (crate) buf : Vec < u8 > , pub (crate) fail_on_err_lines : bool , pub (crate) delimiters : & 'static [PacketLineRef < 'static >] , pub (crate) is_done : bool , pub (crate) stopped_at : Option < PacketLineRef < 'static > > , # [cfg_attr (all (not (feature = "async-io") , not (feature = "blocking-io")) , allow (dead_code))] pub (crate) trace : bool , }
    };
}

StreamingPeekableIterState!();