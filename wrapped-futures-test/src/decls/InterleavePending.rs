macro_rules! InterleavePending {
    () => {
        # [doc = " Wrapper that interleaves [`Poll::Pending`] in calls to poll."] # [doc = ""] # [doc = " See the `interleave_pending` methods on:"] # [doc = " * [`FutureTestExt`](crate::future::FutureTestExt::interleave_pending)"] # [doc = " * [`StreamTestExt`](crate::stream::StreamTestExt::interleave_pending)"] # [doc = " * [`SinkTestExt`](crate::sink::SinkTestExt::interleave_pending_sink)"] # [doc = " * [`AsyncReadTestExt`](crate::io::AsyncReadTestExt::interleave_pending)"] # [doc = " * [`AsyncWriteTestExt`](crate::io::AsyncWriteTestExt::interleave_pending_write)"] # [pin_project] # [derive (Debug)] pub struct InterleavePending < T > { # [pin] inner : T , pended : bool , }
    };
}

InterleavePending!();