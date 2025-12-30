// Generated macro for impl_539 (impl)
macro_rules! Depcrate_future_race_vecimpl_539 {
() => {
// Module: crate::future::race::vec
// Provides: {"impl_539"}
// Dependencies: {}
impl < Fut > fmt :: Debug for Race < Fut > where Fut : Future + fmt :: Debug , Fut :: Output : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . futures . iter ()) . finish () } }
};
}
