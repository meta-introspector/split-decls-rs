// Generated macro for impl_214 (impl)
macro_rules! Depcrate_thenimpl_214 {
() => {
// Module: crate::then
// Provides: {"impl_214"}
// Dependencies: {}
impl < A , B , Req > Future for ThenServiceResponse < A , B , Req > where A : Service < Req > , B : Service < Result < A :: Response , A :: Error > > , { type Output = Result < B :: Response , B :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . as_mut () . project () ; match this . state . as_mut () . project () { StateProj :: A { fut , b } => { let res = ready ! (fut . poll (cx)) ; let b = b . take () . unwrap () ; let fut = b . 1 . call (res) ; this . state . set (State :: B { fut }) ; self . poll (cx) } StateProj :: B { fut } => fut . poll (cx) , } } }
};
}
