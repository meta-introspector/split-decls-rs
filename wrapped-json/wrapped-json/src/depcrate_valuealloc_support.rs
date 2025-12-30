// Generated macro for alloc_support (module)
macro_rules! Depcrate_valuealloc_support {
() => {
// Module: crate::value
// Provides: {"alloc_support"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod alloc_support { use super :: * ; use alloc :: boxed :: Box ; impl JsonStr { # [doc = "\n        Treat a string as native JSON.\n        "] pub fn boxed (json : impl Into < Box < str > >) -> Box < Self > { let json = json . into () ; unsafe { Box :: from_raw (Box :: into_raw (json) as * mut str as * mut JsonStr) } } } impl From < Box < str > > for Box < JsonStr > { fn from (value : Box < str >) -> Self { unsafe { Box :: from_raw (Box :: into_raw (value) as * mut JsonStr) } } } }
};
}
