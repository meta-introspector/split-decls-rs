// Generated macro for impl_664 (impl)
macro_rules! Depcrate_stream_stream_flattenimpl_664 {
() => {
// Module: crate::stream::stream::flatten
// Provides: {"impl_664"}
// Dependencies: {}
impl < St > Stream for Flatten < St , St :: Item > where St : Stream , St :: Item : Stream , { type Item = < St :: Item as Stream > :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; Poll :: Ready (loop { if let Some (s) = this . next . as_mut () . as_pin_mut () { if let Some (item) = ready ! (s . poll_next (cx)) { break Some (item) ; } else { this . next . set (None) ; } } else if let Some (s) = ready ! (this . stream . as_mut () . poll_next (cx)) { this . next . set (Some (s)) ; } else { break None ; } }) } }
};
}
