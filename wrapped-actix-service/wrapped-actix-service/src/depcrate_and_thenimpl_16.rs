// Generated macro for impl_16 (impl)
macro_rules! Depcrate_and_thenimpl_16 {
() => {
// Module: crate::and_then
// Provides: {"impl_16"}
// Dependencies: {}
impl < A , B , Req > Future for AndThenServiceResponse < A , B , Req > where A : Service < Req > , B : Service < A :: Response , Error = A :: Error > , { type Output = Result < B :: Response , A :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . as_mut () . project () ; match this . state . as_mut () . project () { StateProj :: A { fut , b } => { let res = ready ! (fut . poll (cx)) ? ; let b = b . take () . unwrap () ; let fut = b . 1 . call (res) ; this . state . set (State :: B { fut }) ; self . poll (cx) } StateProj :: B { fut } => fut . poll (cx) , } } }
};
}
