// Generated macro for impl_836 (impl)
macro_rules! Depcrate_stream_stream_peekimpl_836 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"impl_836"}
// Dependencies: {}
impl < St , T > Future for NextIfEq < '_ , St , T > where St : Stream , T : ? Sized , St :: Item : PartialEq < T > , { type Output = Option < St :: Item > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . inner . poll (cx) } }
};
}
