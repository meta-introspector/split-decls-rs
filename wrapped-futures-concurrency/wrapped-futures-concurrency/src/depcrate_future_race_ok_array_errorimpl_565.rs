// Generated macro for impl_565 (impl)
macro_rules! Depcrate_future_race_ok_array_errorimpl_565 {
() => {
// Module: crate::future::race_ok::array::error
// Provides: {"impl_565"}
// Dependencies: {}
impl < E : fmt :: Display , const N : usize > fmt :: Display for AggregateError < E , N > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} errors occured" , self . inner . len ()) } }
};
}
