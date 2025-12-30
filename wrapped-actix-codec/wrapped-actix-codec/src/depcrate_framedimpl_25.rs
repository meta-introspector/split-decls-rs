// Generated macro for impl_25 (impl)
macro_rules! Depcrate_framedimpl_25 {
() => {
// Module: crate::framed
// Provides: {"impl_25"}
// Dependencies: {}
impl < T , U > Stream for Framed < T , U > where T : AsyncRead , U : Decoder , { type Item = Result < U :: Item , U :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . next_item (cx) } }
};
}
