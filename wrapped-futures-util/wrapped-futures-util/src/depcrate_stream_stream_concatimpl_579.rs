// Generated macro for impl_579 (impl)
macro_rules! Depcrate_stream_stream_concatimpl_579 {
() => {
// Module: crate::stream::stream::concat
// Provides: {"impl_579"}
// Dependencies: {}
impl < St > Future for Concat < St > where St : Stream , St :: Item : Extend < < St :: Item as IntoIterator > :: Item > + IntoIterator + Default , { type Output = St :: Item ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { None => return Poll :: Ready (this . accum . take () . unwrap_or_default ()) , Some (e) => { if let Some (a) = this . accum { a . extend (e) } else { * this . accum = Some (e) } } } } } }
};
}
