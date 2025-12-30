// Generated macro for impl_503 (impl)
macro_rules! Depcrate_future_race_arrayimpl_503 {
() => {
// Module: crate::future::race::array
// Provides: {"impl_503"}
// Dependencies: {}
impl < Fut , const N : usize > fmt :: Debug for Race < Fut , N > where Fut : Future + fmt :: Debug , Fut :: Output : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . futures . iter ()) . finish () } }
};
}
