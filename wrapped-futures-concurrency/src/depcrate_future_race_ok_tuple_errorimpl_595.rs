// Generated macro for impl_595 (impl)
macro_rules! Depcrate_future_race_ok_tuple_errorimpl_595 {
() => {
// Module: crate::future::race_ok::tuple::error
// Provides: {"impl_595"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl < E : fmt :: Display , const N : usize > fmt :: Display for AggregateError < E , N > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} errors occured" , self . inner . len ()) } }
};
}
