// Generated macro for impl_23 (impl)
macro_rules! Depcrate_and_thenimpl_23 {
() => {
// Module: crate::and_then
// Provides: {"impl_23"}
// Dependencies: {}
impl < A , B , Req > Future for AndThenServiceFactoryResponse < A , B , Req > where A : ServiceFactory < Req > , B : ServiceFactory < A :: Response , Error = A :: Error , InitError = A :: InitError > , { type Output = Result < AndThenService < A :: Service , B :: Service , Req > , A :: InitError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if this . a . is_none () { if let Poll :: Ready (service) = this . fut_a . poll (cx) ? { * this . a = Some (service) ; } } if this . b . is_none () { if let Poll :: Ready (service) = this . fut_b . poll (cx) ? { * this . b = Some (service) ; } } if this . a . is_some () && this . b . is_some () { Poll :: Ready (Ok (AndThenService :: new (this . a . take () . unwrap () , this . b . take () . unwrap () ,))) } else { Poll :: Pending } } }
};
}
