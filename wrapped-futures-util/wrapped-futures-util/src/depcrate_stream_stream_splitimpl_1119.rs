// Generated macro for impl_1119 (impl)
macro_rules! Depcrate_stream_stream_splitimpl_1119 {
() => {
// Module: crate::stream::stream::split
// Provides: {"impl_1119"}
// Dependencies: {}
impl < S > SplitStream < S > { # [doc = " Returns `true` if the `SplitStream<S>` and `SplitSink<S>` originate from the same call to `StreamExt::split`."] pub fn is_pair_of < Item > (& self , other : & SplitSink < S , Item >) -> bool { other . is_pair_of (& self) } }
};
}
