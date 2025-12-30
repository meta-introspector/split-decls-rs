// Generated macro for impl_1161 (impl)
macro_rules! Depcrate_stream_stream_catch_unwindimpl_1161 {
() => {
// Module: crate::stream::stream::catch_unwind
// Provides: {"impl_1161"}
// Dependencies: {}
impl < St : Stream + UnwindSafe > Stream for CatchUnwind < St > { type Item = Result < St :: Item , Box < dyn Any + Send > > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; if * this . caught_unwind { Poll :: Ready (None) } else { let res = catch_unwind (AssertUnwindSafe (| | this . stream . as_mut () . poll_next (cx))) ; match res { Ok (poll) => poll . map (| opt | opt . map (Ok)) , Err (e) => { * this . caught_unwind = true ; Poll :: Ready (Some (Err (e))) } } } } fn size_hint (& self) -> (usize , Option < usize >) { if self . caught_unwind { (0 , Some (0)) } else { self . stream . size_hint () } } }
};
}
