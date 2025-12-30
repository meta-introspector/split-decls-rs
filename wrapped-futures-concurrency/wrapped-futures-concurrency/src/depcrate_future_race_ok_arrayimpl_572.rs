// Generated macro for impl_572 (impl)
macro_rules! Depcrate_future_race_ok_arrayimpl_572 {
() => {
// Module: crate::future::race_ok::array
// Provides: {"impl_572"}
// Dependencies: {}
impl < Fut , T , E , const N : usize > fmt :: Debug for RaceOk < Fut , T , E , N > where Fut : Future < Output = Result < T , E > > + fmt :: Debug , Fut :: Output : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . futures . iter ()) . finish () } }
};
}
