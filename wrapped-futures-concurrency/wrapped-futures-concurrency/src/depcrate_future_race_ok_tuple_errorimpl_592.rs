// Generated macro for impl_592 (impl)
macro_rules! Depcrate_future_race_ok_tuple_errorimpl_592 {
() => {
// Module: crate::future::race_ok::tuple::error
// Provides: {"impl_592"}
// Dependencies: {}
# [cfg (feature = "std")] impl < E : Error , const N : usize > fmt :: Debug for AggregateError < E , N > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "{self}:") ? ; for (i , err) in self . inner . iter () . enumerate () { writeln ! (f , "- Error {}: {err}" , i + 1) ? ; let mut source = err . source () ; while let Some (err) = source { writeln ! (f , "  ↳ Caused by: {err}") ? ; source = err . source () ; } } Ok (()) } }
};
}
