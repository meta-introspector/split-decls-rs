// Generated macro for impl_634 (impl)
macro_rules! Depcrate_future_race_ok_vec_errorimpl_634 {
() => {
// Module: crate::future::race_ok::vec::error
// Provides: {"impl_634"}
// Dependencies: {}
impl < E : fmt :: Display > fmt :: Display for AggregateError < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} errors occurred" , self . inner . len ()) } }
};
}
