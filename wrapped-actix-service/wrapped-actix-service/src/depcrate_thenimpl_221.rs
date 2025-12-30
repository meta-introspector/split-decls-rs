// Generated macro for impl_221 (impl)
macro_rules! Depcrate_thenimpl_221 {
() => {
// Module: crate::then
// Provides: {"impl_221"}
// Dependencies: {}
impl < A , B , Req > Future for ThenServiceFactoryResponse < A , B , Req > where A : ServiceFactory < Req > , B : ServiceFactory < Result < A :: Response , A :: Error > , Config = A :: Config , Error = A :: Error , InitError = A :: InitError , > , { type Output = Result < ThenService < A :: Service , B :: Service , Req > , A :: InitError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if this . a . is_none () { if let Poll :: Ready (service) = this . fut_a . poll (cx) ? { * this . a = Some (service) ; } } if this . b . is_none () { if let Poll :: Ready (service) = this . fut_b . poll (cx) ? { * this . b = Some (service) ; } } if this . a . is_some () && this . b . is_some () { Poll :: Ready (Ok (ThenService :: new (this . a . take () . unwrap () , this . b . take () . unwrap () ,))) } else { Poll :: Pending } } }
};
}
