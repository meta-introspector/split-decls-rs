// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
# [cfg (feature = "std")] impl AsyncWrite for Writer { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { self . poll_fill_bytes (cx , buf) . map (Ok) } fn poll_flush (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } fn poll_close (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . inner . closed . store (true , Ordering :: Release) ; self . inner . reader . wake () ; self . inner . writer . wake () ; Poll :: Ready (Ok (())) } }
};
}
