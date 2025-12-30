// Generated macro for impl_2690 (impl)
macro_rules! Depcrate_abortableimpl_2690 {
() => {
// Module: crate::abortable
// Provides: {"impl_2690"}
// Dependencies: {}
impl < T > Abortable < T > { fn try_poll < I > (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , poll : impl Fn (Pin < & mut T > , & mut Context < '_ >) -> Poll < I > ,) -> Poll < Result < I , Aborted > > { if self . is_aborted () { return Poll :: Ready (Err (Aborted)) ; } if let Poll :: Ready (x) = poll (self . as_mut () . project () . task , cx) { return Poll :: Ready (Ok (x)) ; } self . inner . waker . register (cx . waker ()) ; if self . is_aborted () { return Poll :: Ready (Err (Aborted)) ; } Poll :: Pending } }
};
}
