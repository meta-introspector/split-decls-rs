macro_rules! deps {
    () => {
        MessageTrailers!();
    };
}

macro_rules! MessageTrailersStrs {
    () => {
        deps!();
        # [doc = " Collection of UTF-8-encoded trailers."] # [doc = ""] # [doc = " Use `iter()` to get access to the values."] pub struct MessageTrailersStrs (MessageTrailers) ;
    };
}

MessageTrailersStrs!();