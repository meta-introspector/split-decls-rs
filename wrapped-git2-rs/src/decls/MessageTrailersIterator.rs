macro_rules! deps {
    () => {
        MessageTrailers!();
    };
}

macro_rules! MessageTrailersIterator {
    () => {
        deps!();
        struct MessageTrailersIterator < 'a > { trailers : & 'a MessageTrailers , range : Range < usize > , }
    };
}

MessageTrailersIterator!();