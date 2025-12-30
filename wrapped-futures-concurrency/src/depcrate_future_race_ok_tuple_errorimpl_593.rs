// Generated macro for impl_593 (impl)
macro_rules! Depcrate_future_race_ok_tuple_errorimpl_593 {
() => {
// Module: crate::future::race_ok::tuple::error
// Provides: {"impl_593"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl < E : fmt :: Display , const N : usize > fmt :: Debug for AggregateError < E , N > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "{self}:") ? ; for (i , err) in self . inner . iter () . enumerate () { writeln ! (f , "- Error {}: {err}" , i + 1) ? ; } Ok (()) } }
};
}
