// Generated macro for impl_438 (impl)
macro_rules! Depcrate_future_join_arrayimpl_438 {
() => {
// Module: crate::future::join::array
// Provides: {"impl_438"}
// Dependencies: {}
impl < Fut , const N : usize > fmt :: Debug for Join < Fut , N > where Fut : Future + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . state . iter ()) . finish () } }
};
}
