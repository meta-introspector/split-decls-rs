macro_rules! BoxedReader {
    () => {
        # [doc = " Type alias for `Pin<Box<dyn AsyncRead + Send + 'static>>`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::AsyncReadExt;"] # [doc = ""] # [doc = " let reader = [1, 2, 3].boxed_reader();"] # [doc = " ```"] # [cfg (feature = "alloc")] pub type BoxedReader = Pin < Box < dyn AsyncRead + Send + 'static > > ;
    };
}

BoxedReader!();