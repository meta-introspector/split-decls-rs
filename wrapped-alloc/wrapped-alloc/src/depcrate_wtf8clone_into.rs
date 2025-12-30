// Generated macro for clone_into (function)
macro_rules! Depcrate_wtf8clone_into {
() => {
// Module: crate::wtf8
// Provides: {"clone_into"}
// Dependencies: {}
# [inline] pub (super) fn clone_into (slice : & Wtf8 , buf : & mut Wtf8Buf) { buf . is_known_utf8 = false ; slice . as_bytes () . clone_into (& mut buf . bytes) ; }
};
}
