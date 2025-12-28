macro_rules! deps {
    () => {
        MessageTrailersIterator!();
    };
}

macro_rules! MessageTrailersBytesIterator {
    () => {
        deps!();
        # [doc = " Borrowed iterator over the raw (bytes) trailers."] pub struct MessageTrailersBytesIterator < 'a > (MessageTrailersIterator < 'a >) ;
    };
}

MessageTrailersBytesIterator!()