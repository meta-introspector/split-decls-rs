macro_rules! NEED_TO_POLL_ALL {
    () => {
        # [doc = " Both base stream and inner streams need to be polled."] const NEED_TO_POLL_ALL : u8 = NEED_TO_POLL_INNER_STREAMS | NEED_TO_POLL_STREAM ;
    };
}

NEED_TO_POLL_ALL!();