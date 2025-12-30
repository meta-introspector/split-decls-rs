// Generated macro for impl_821 (impl)
macro_rules! Depcrate_stream_merge_arrayimpl_821 {
() => {
// Module: crate::stream::merge::array
// Provides: {"impl_821"}
// Dependencies: {}
impl < S , const N : usize > Stream for Merge < S , N > where S : Stream , { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; let mut readiness = this . wakers . readiness () ; readiness . set_waker (cx . waker ()) ; for index in this . indexer . iter () { if ! readiness . any_ready () { return Poll :: Pending ; } else if ! readiness . clear_ready (index) || this . state [index] . is_none () { continue ; } # [allow (clippy :: drop_non_drop)] drop (readiness) ; let mut cx = Context :: from_waker (this . wakers . get (index) . unwrap ()) ; let stream = utils :: get_pin_mut (this . streams . as_mut () , index) . unwrap () ; match stream . poll_next (& mut cx) { Poll :: Ready (Some (item)) => { this . wakers . readiness () . set_ready (index) ; return Poll :: Ready (Some (item)) ; } Poll :: Ready (None) => { * this . complete += 1 ; this . state [index] . set_none () ; if * this . complete == this . streams . len () { return Poll :: Ready (None) ; } } Poll :: Pending => { } } readiness = this . wakers . readiness () ; } Poll :: Pending } }
};
}
