macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! NEED_TO_POLL_INNER_STREAMS {
    () => {
        deps!();
        # [doc = " Inner streams need to be polled."] const NEED_TO_POLL_INNER_STREAMS : u8 = 1 ;
    };
}

NEED_TO_POLL_INNER_STREAMS!()