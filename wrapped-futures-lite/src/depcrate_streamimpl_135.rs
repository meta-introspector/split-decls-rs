// Generated macro for impl_135 (impl)
macro_rules! Depcrate_streamimpl_135 {
() => {
// Module: crate::stream
// Provides: {"impl_135"}
// Dependencies: {}
impl < S , St , F , B > Stream for Scan < S , St , F > where S : Stream , F : FnMut (& mut St , S :: Item) -> Option < B > , { type Item = B ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < B > > { let mut this = self . project () ; this . stream . as_mut () . poll_next (cx) . map (| item | { item . and_then (| item | { let (state , f) = this . state_f ; f (state , item) }) }) } }
};
}
