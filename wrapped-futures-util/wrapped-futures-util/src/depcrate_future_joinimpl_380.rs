// Generated macro for impl_380 (impl)
macro_rules! Depcrate_future_joinimpl_380 {
() => {
// Module: crate::future::join
// Provides: {"impl_380"}
// Dependencies: {}
impl < Fut1 , Fut2 > fmt :: Debug for Join < Fut1 , Fut2 > where Fut1 : Future + fmt :: Debug , Fut1 :: Output : fmt :: Debug , Fut2 : Future + fmt :: Debug , Fut2 :: Output : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Join") . field ("fut1" , & self . fut1) . field ("fut2" , & self . fut2) . finish () } }
};
}
