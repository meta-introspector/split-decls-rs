// Generated macro for impl_1126 (impl)
macro_rules! Depcrate_stream_stream_splitimpl_1126 {
() => {
// Module: crate::stream::stream::split
// Provides: {"impl_1126"}
// Dependencies: {}
impl < S , Item > SplitSink < S , Item > { # [doc = " Returns `true` if the `SplitStream<S>` and `SplitSink<S>` originate from the same call to `StreamExt::split`."] pub fn is_pair_of (& self , other : & SplitStream < S >) -> bool { self . lock . is_pair_of (& other . 0) } }
};
}
