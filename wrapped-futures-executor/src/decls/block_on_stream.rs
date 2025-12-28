macro_rules! deps {
    () => {
        BlockingStream!();
    };
}

macro_rules! block_on_stream {
    () => {
        deps!();
        # [doc = " Turn a stream into a blocking iterator."] # [doc = ""] # [doc = " When `next` is called on the resulting `BlockingStream`, the caller"] # [doc = " will be blocked until the next element of the `Stream` becomes available."] pub fn block_on_stream < S : Stream + Unpin > (stream : S) -> BlockingStream < S > { BlockingStream { stream } }
    };
}

block_on_stream!();