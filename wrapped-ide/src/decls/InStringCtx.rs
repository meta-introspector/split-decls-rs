macro_rules! InStringCtx {
    () => {
        struct InStringCtx { offset : u32 , marker_positions : Vec < u32 > , }
    };
}

InStringCtx!()