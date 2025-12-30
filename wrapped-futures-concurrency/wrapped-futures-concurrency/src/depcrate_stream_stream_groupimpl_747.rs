// Generated macro for impl_747 (impl)
macro_rules! Depcrate_stream_stream_groupimpl_747 {
() => {
// Module: crate::stream::stream_group
// Provides: {"impl_747"}
// Dependencies: {}
impl < S : Stream > FromIterator < S > for StreamGroup < S > { fn from_iter < T : IntoIterator < Item = S > > (iter : T) -> Self { let iter = iter . into_iter () ; let len = iter . size_hint () . 1 . unwrap_or_default () ; let mut this = Self :: with_capacity (len) ; for stream in iter { this . insert (stream) ; } this } }
};
}
