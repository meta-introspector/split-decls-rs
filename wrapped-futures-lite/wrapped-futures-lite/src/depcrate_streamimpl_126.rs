// Generated macro for impl_126 (impl)
macro_rules! Depcrate_streamimpl_126 {
() => {
// Module: crate::stream
// Provides: {"impl_126"}
// Dependencies: {}
impl < T , E , S , C > Future for TryCollectFuture < S , C > where S : Stream < Item = Result < T , E > > , C : Default + Extend < T > , { type Output = Result < C , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; Poll :: Ready (Ok (loop { match ready ! (this . stream . as_mut () . poll_next (cx) ?) { Some (x) => this . items . extend (Some (x)) , None => break mem :: take (this . items) , } })) } }
};
}
