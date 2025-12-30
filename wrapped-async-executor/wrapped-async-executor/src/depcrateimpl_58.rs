// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
impl Drop for Ticker < '_ > { fn drop (& mut self) { if self . sleeping != 0 { let mut sleepers = self . state . sleepers . lock () . unwrap_or_else (PoisonError :: into_inner) ; let notified = sleepers . remove (self . sleeping) ; self . state . notified . store (sleepers . is_notified () , Ordering :: Release) ; if notified { drop (sleepers) ; self . state . notify () ; } } } }
};
}
