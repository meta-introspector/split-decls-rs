macro_rules! POLLING {
    () => {
        # [doc = " The current stream is being polled at the moment."] const POLLING : u8 = 0b100 ;
    };
}

POLLING!();