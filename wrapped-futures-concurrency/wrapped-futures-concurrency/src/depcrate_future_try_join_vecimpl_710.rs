// Generated macro for impl_710 (impl)
macro_rules! Depcrate_future_try_join_vecimpl_710 {
() => {
// Module: crate::future::try_join::vec
// Provides: {"impl_710"}
// Dependencies: {}
impl < Fut , T , E > fmt :: Debug for TryJoin < Fut , T , E > where Fut : Future < Output = Result < T , E > > + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . state . iter ()) . finish () } }
};
}
