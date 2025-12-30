// Generated macro for impl_652 (impl)
macro_rules! Depcrateimpl_652 {
() => {
// Module: crate
// Provides: {"impl_652"}
// Dependencies: {}
impl < T , E : std :: fmt :: Debug > OrFail for std :: result :: Result < T , E > { type Output = T ; # [track_caller] fn or_fail (self) -> std :: result :: Result < T , TestAssertionFailure > { match self { Ok (t) => Ok (t) , Err (e) => Err (TestAssertionFailure :: create (format ! ("{e:?}"))) , } } }
};
}
