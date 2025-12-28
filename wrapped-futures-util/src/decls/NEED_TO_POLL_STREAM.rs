macro_rules! NEED_TO_POLL_STREAM {
    () => {
        # [doc = " The base stream needs to be polled."] const NEED_TO_POLL_STREAM : u8 = 0b10 ;
    };
}

NEED_TO_POLL_STREAM!()