// Generated macro for impl_88 (impl)
macro_rules! Depcrate_errorimpl_88 {
() => {
// Module: crate::error
// Provides: {"impl_88"}
// Dependencies: {}
# [doc = " Manual implementation because [`Span`] is not [`PartialEq`] and can be ignored when comparing."] impl PartialEq for SpanError { fn eq (& self , other : & SpanError) -> bool { self . err == other . err } }
};
}
