macro_rules! deps {
    () => {
        MessageTrailers!();
    };
}

macro_rules! MessageTrailersBytes {
    () => {
        deps!();
        # [doc = " Collection of unencoded (bytes) trailers."] # [doc = ""] # [doc = " Use `iter()` to get access to the values."] pub struct MessageTrailersBytes (MessageTrailers) ;
    };
}

MessageTrailersBytes!();