macro_rules! Zip {
    () => {
        # [doc = " ‘Zips up’ multiple streams into a single stream of pairs."] pub trait Zip { # [doc = " What's the return type of our stream?"] type Item ; # [doc = " What stream do we return?"] type Stream : Stream < Item = Self :: Item > ; # [doc = " Combine multiple streams into a single stream."] fn zip (self) -> Self :: Stream ; }
    };
}

Zip!()