// Generated macro for impl_564 (impl)
macro_rules! Depcrate_future_race_ok_array_errorimpl_564 {
() => {
// Module: crate::future::race_ok::array::error
// Provides: {"impl_564"}
// Dependencies: {}
impl < E : fmt :: Display , const N : usize > fmt :: Debug for AggregateError < E , N > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "{self}:") ? ; for (i , err) in self . inner . iter () . enumerate () { writeln ! (f , "- Error {}: {err}" , i + 1) ? ; } Ok (()) } }
};
}
