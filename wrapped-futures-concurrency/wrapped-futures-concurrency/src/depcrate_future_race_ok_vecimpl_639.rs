// Generated macro for impl_639 (impl)
macro_rules! Depcrate_future_race_ok_vecimpl_639 {
() => {
// Module: crate::future::race_ok::vec
// Provides: {"impl_639"}
// Dependencies: {}
impl < Fut , T , E > fmt :: Debug for RaceOk < Fut , T , E > where Fut : Future < Output = Result < T , E > > + fmt :: Debug , Fut :: Output : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . elems . iter ()) . finish () } }
};
}
