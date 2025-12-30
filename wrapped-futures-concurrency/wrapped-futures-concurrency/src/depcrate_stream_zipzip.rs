// Generated macro for Zip (trait)
macro_rules! Depcrate_stream_zipZip {
() => {
// Module: crate::stream::zip
// Provides: {"Zip"}
// Dependencies: {}
# [doc = " ‘Zips up’ multiple streams into a single stream of pairs."] pub trait Zip { # [doc = " What's the return type of our stream?"] type Item ; # [doc = " What stream do we return?"] type Stream : Stream < Item = Self :: Item > ; # [doc = " Combine multiple streams into a single stream."] fn zip (self) -> Self :: Stream ; }
};
}
