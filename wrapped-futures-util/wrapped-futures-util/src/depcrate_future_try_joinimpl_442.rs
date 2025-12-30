// Generated macro for impl_442 (impl)
macro_rules! Depcrate_future_try_joinimpl_442 {
() => {
// Module: crate::future::try_join
// Provides: {"impl_442"}
// Dependencies: {}
impl < Fut1 , Fut2 > fmt :: Debug for TryJoin < Fut1 , Fut2 > where Fut1 : TryFuture + fmt :: Debug , Fut1 :: Ok : fmt :: Debug , Fut1 :: Error : fmt :: Debug , Fut2 : TryFuture + fmt :: Debug , Fut2 :: Ok : fmt :: Debug , Fut2 :: Error : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryJoin") . field ("fut1" , & self . fut1) . field ("fut2" , & self . fut2) . finish () } }
};
}
