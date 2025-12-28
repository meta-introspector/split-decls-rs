macro_rules! AssertUnmoved {
    () => {
        # [doc = " Combinator that asserts that the underlying type is not moved after being polled."] # [doc = ""] # [doc = " See the `assert_unmoved` methods on:"] # [doc = " * [`FutureTestExt`](crate::future::FutureTestExt::assert_unmoved)"] # [doc = " * [`StreamTestExt`](crate::stream::StreamTestExt::assert_unmoved)"] # [doc = " * [`SinkTestExt`](crate::sink::SinkTestExt::assert_unmoved_sink)"] # [doc = " * [`AsyncReadTestExt`](crate::io::AsyncReadTestExt::assert_unmoved)"] # [doc = " * [`AsyncWriteTestExt`](crate::io::AsyncWriteTestExt::assert_unmoved_write)"] # [pin_project (PinnedDrop , ! Unpin)] # [derive (Debug , Clone)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct AssertUnmoved < T > { # [pin] inner : T , this_addr : usize , }
    };
}

AssertUnmoved!()