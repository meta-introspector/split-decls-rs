// Generated macro for impl_750 (impl)
macro_rules! Depcrate_stream_stream_groupimpl_750 {
() => {
// Module: crate::stream::stream_group
// Provides: {"impl_750"}
// Dependencies: {}
impl < S : Stream > Deref for Keyed < S > { type Target = StreamGroup < S > ; fn deref (& self) -> & Self :: Target { & self . group } }
};
}
