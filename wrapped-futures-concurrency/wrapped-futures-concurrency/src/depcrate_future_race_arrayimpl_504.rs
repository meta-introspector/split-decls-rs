// Generated macro for impl_504 (impl)
macro_rules! Depcrate_future_race_arrayimpl_504 {
() => {
// Module: crate::future::race::array
// Provides: {"impl_504"}
// Dependencies: {}
impl < Fut , const N : usize > Future for Race < Fut , N > where Fut : Future , { type Output = Fut :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; assert ! (!* this . done , "Futures must not be polled after completing") ; for index in this . indexer . iter () { let fut = utils :: get_pin_mut (this . futures . as_mut () , index) . unwrap () ; match fut . poll (cx) { Poll :: Ready (item) => { * this . done = true ; return Poll :: Ready (item) ; } Poll :: Pending => continue , } } Poll :: Pending } }
};
}
