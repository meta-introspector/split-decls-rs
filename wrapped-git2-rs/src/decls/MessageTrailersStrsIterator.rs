macro_rules! deps {
    () => {
        MessageTrailersIterator!();
    };
}

macro_rules! MessageTrailersStrsIterator {
    () => {
        deps!();
        # [doc = " Borrowed iterator over the UTF-8-encoded trailers."] pub struct MessageTrailersStrsIterator < 'a > (MessageTrailersIterator < 'a >) ;
    };
}

MessageTrailersStrsIterator!()