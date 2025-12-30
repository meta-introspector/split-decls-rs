// Generated macro for impl_2692 (impl)
macro_rules! Depcrate_abortableimpl_2692 {
() => {
// Module: crate::abortable
// Provides: {"impl_2692"}
// Dependencies: {}
impl < St > Stream for Abortable < St > where St : Stream , { type Item = St :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . try_poll (cx , | stream , cx | stream . poll_next (cx)) . map (Result :: ok) . map (Option :: flatten) } }
};
}
