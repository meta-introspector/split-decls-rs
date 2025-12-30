// Generated macro for impl_537 (impl)
macro_rules! Depcrate_utilsimpl_537 {
() => {
// Module: crate::utils
// Provides: {"impl_537"}
// Dependencies: {}
impl < A : Actor > ActorStream < A > for IntervalFunc < A > { type Item = () ; fn poll_next (self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; loop { ready ! (this . timer . as_mut () . poll (task)) ; let now = this . timer . deadline () ; this . timer . as_mut () . reset (now + * this . interval) ; (this . f) (act , ctx) ; } } }
};
}
