// Generated macro for impl_63 (impl)
macro_rules! Depcrate_reactorimpl_63 {
() => {
// Module: crate::reactor
// Provides: {"impl_63"}
// Dependencies: {}
impl Direction { # [doc = " Returns `true` if there are no wakers interested in this direction."] fn is_empty (& self) -> bool { self . waker . is_none () && self . wakers . iter () . all (| (_ , opt) | opt . is_none ()) } # [doc = " Moves all wakers into a `Vec`."] fn drain_into (& mut self , dst : & mut Vec < Waker >) { if let Some (w) = self . waker . take () { dst . push (w) ; } for (_ , opt) in self . wakers . iter_mut () { if let Some (w) = opt . take () { dst . push (w) ; } } } }
};
}
