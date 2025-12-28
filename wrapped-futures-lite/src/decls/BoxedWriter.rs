macro_rules! BoxedWriter {
    () => {
        # [doc = " Type alias for `Pin<Box<dyn AsyncWrite + Send + 'static>>`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::AsyncWriteExt;"] # [doc = ""] # [doc = " let writer = Vec::<u8>::new().boxed_writer();"] # [doc = " ```"] # [cfg (feature = "alloc")] pub type BoxedWriter = Pin < Box < dyn AsyncWrite + Send + 'static > > ;
    };
}

BoxedWriter!()