// Generated macro for impl_239 (impl)
macro_rules! Depcrate_transformimpl_239 {
() => {
// Module: crate::transform
// Provides: {"impl_239"}
// Dependencies: {}
impl < T , S , Req > Future for ApplyTransformFuture < T , S , Req > where S : ServiceFactory < Req > , T : Transform < S :: Service , Req , InitError = S :: InitError > , { type Output = Result < T :: Transform , T :: InitError > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . as_mut () . project () ; match this . state . as_mut () . project () { ApplyTransformFutureStateProj :: A { fut } => { let srv = ready ! (fut . poll (cx)) ? ; let fut = this . store . 0 . new_transform (srv) ; this . state . set (ApplyTransformFutureState :: B { fut }) ; self . poll (cx) } ApplyTransformFutureStateProj :: B { fut } => fut . poll (cx) , } } }
};
}
