macro_rules! ZeroToken {
    () => {
        # [doc = " A pointer to a packet."] pub (crate) struct ZeroToken (* mut ()) ;
    };
}

ZeroToken!()