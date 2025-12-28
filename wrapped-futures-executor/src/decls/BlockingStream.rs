macro_rules! BlockingStream {
    () => {
        # [doc = " An iterator which blocks on values from a stream until they become available."] # [derive (Debug)] pub struct BlockingStream < S : Stream + Unpin > { stream : S , }
    };
}

BlockingStream!();