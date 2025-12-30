// Generated macro for eq_span_tokens (function)
macro_rules! Depcrate_hir_utilseq_span_tokens {
() => {
// Module: crate::hir_utils
// Provides: {"eq_span_tokens"}
// Dependencies: {}
fn eq_span_tokens (cx : & LateContext < '_ > , left : impl SpanRange , right : impl SpanRange , pred : impl Fn (TokenKind) -> bool ,) -> bool { fn f (cx : & LateContext < '_ > , left : Range < BytePos > , right : Range < BytePos > , pred : impl Fn (TokenKind) -> bool) -> bool { if let Some (lsrc) = left . get_source_range (cx) && let Some (lsrc) = lsrc . as_str () && let Some (rsrc) = right . get_source_range (cx) && let Some (rsrc) = rsrc . as_str () { let pred = | & (token , ..) : & (TokenKind , _ , _) | pred (token) ; let map = | (_ , source , _) | source ; let ltok = tokenize_with_text (lsrc) . filter (pred) . map (map) ; let rtok = tokenize_with_text (rsrc) . filter (pred) . map (map) ; ltok . eq (rtok) } else { false } } f (cx , left . into_range () , right . into_range () , pred) }
};
}
