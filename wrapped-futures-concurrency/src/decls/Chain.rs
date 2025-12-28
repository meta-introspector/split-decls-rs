macro_rules! Chain {
    () => {
        # [doc = " Takes multiple streams and creates a new stream over all in sequence."] pub trait Chain { # [doc = " What's the return type of our stream?"] type Item ; # [doc = " What stream do we return?"] type Stream : Stream < Item = Self :: Item > ; # [doc = " Combine multiple streams into a single stream."] fn chain (self) -> Self :: Stream ; }
    };
}

Chain!();