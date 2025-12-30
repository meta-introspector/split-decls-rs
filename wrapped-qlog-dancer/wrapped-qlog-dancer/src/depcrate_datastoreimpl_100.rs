// Generated macro for impl_100 (impl)
macro_rules! Depcrate_datastoreimpl_100 {
() => {
// Module: crate::datastore
// Provides: {"impl_100"}
// Dependencies: {}
impl Display for ApplicationProto { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let v = match self { ApplicationProto :: Http2 => "HTTP/2" , ApplicationProto :: Http3 => "HTTP/3" , } ; write ! (f , "{}" , v) } }
};
}
