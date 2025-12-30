// Generated macro for impl_487 (impl)
macro_rules! Depcrate_future_join_vecimpl_487 {
() => {
// Module: crate::future::join::vec
// Provides: {"impl_487"}
// Dependencies: {}
impl < Fut > fmt :: Debug for Join < Fut > where Fut : Future + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . state . iter ()) . finish () } }
};
}
