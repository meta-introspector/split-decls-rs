// Generated macro for impl_633 (impl)
macro_rules! Depcrate_future_race_ok_vec_errorimpl_633 {
() => {
// Module: crate::future::race_ok::vec::error
// Provides: {"impl_633"}
// Dependencies: {}
impl < E : fmt :: Display > fmt :: Debug for AggregateError < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "{self}:") ? ; for (i , err) in self . inner . iter () . enumerate () { writeln ! (f , "- Error {}: {err}" , i + 1) ? ; } Ok (()) } }
};
}
