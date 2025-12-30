// Generated macro for BytesInner (enum)
macro_rules! DepcrateBytesInner {
() => {
// Module: crate
// Provides: {"BytesInner"}
// Dependencies: {}
# [derive (Debug , Clone)] enum BytesInner < 'a > { # [cfg (feature = "alloc")] Owned (Vec < u8 >) , Borrowed (& 'a [u8]) , }
};
}
