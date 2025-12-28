macro_rules! TrackClosed {
    () => {
        # [doc = " Async wrapper that tracks whether it has been closed."] # [doc = ""] # [doc = " See the `track_closed` methods on:"] # [doc = " * [`SinkTestExt`](crate::sink::SinkTestExt::track_closed)"] # [doc = " * [`AsyncWriteTestExt`](crate::io::AsyncWriteTestExt::track_closed)"] # [pin_project :: pin_project] # [derive (Debug)] pub struct TrackClosed < T > { # [pin] inner : T , closed : bool , }
    };
}

TrackClosed!();