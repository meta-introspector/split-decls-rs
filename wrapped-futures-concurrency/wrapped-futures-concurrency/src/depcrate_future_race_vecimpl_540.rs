// Generated macro for impl_540 (impl)
macro_rules! Depcrate_future_race_vecimpl_540 {
() => {
// Module: crate::future::race::vec
// Provides: {"impl_540"}
// Dependencies: {}
impl < Fut > Future for Race < Fut > where Fut : Future , { type Output = Fut :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; assert ! (!* this . done , "Futures must not be polled after completing") ; for index in this . indexer . iter () { let fut = utils :: get_pin_mut_from_vec (this . futures . as_mut () , index) . unwrap () ; match fut . poll (cx) { Poll :: Ready (item) => { * this . done = true ; return Poll :: Ready (item) ; } Poll :: Pending => continue , } } Poll :: Pending } }
};
}
