// Generated macro for impl_1325 (impl)
macro_rules! Depcrate_stream_try_stream_try_flatten_unorderedimpl_1325 {
() => {
// Module: crate::stream::try_stream::try_flatten_unordered
// Provides: {"impl_1325"}
// Dependencies: {}
impl < St > Stream for NestedTryStreamIntoEitherTryStream < St > where St : TryStream , St :: Ok : TryStream + Unpin , < St :: Ok as TryStream > :: Error : From < St :: Error > , { type Item = Either < IntoStream < St :: Ok > , SingleStreamResult < St :: Ok > > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let item = ready ! (self . project () . stream . try_poll_next (cx)) ; let out = match item { Some (res) => match res { Ok (stream) => Either :: Left (stream . into_stream ()) , err @ Err (_) => { let res = err . map (| _ : St :: Ok | unreachable ! ()) . map_err (Into :: into) ; Either :: Right (Single :: new (res)) } } , None => return Poll :: Ready (None) , } ; Poll :: Ready (Some (out)) } }
};
}
