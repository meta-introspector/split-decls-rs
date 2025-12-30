// Generated macro for impl_594 (impl)
macro_rules! Depcrate_future_race_ok_tuple_errorimpl_594 {
() => {
// Module: crate::future::race_ok::tuple::error
// Provides: {"impl_594"}
// Dependencies: {}
# [cfg (feature = "std")] impl < E : Error , const N : usize > fmt :: Display for AggregateError < E , N > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} errors occured" , self . inner . len ()) } }
};
}
