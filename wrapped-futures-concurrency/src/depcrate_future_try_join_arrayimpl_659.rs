// Generated macro for impl_659 (impl)
macro_rules! Depcrate_future_try_join_arrayimpl_659 {
() => {
// Module: crate::future::try_join::array
// Provides: {"impl_659"}
// Dependencies: {}
impl < Fut , T , E , const N : usize > fmt :: Debug for TryJoin < Fut , T , E , N > where Fut : Future < Output = Result < T , E > > + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . state . iter ()) . finish () } }
};
}
