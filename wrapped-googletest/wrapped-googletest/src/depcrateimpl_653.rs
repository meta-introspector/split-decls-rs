// Generated macro for impl_653 (impl)
macro_rules! Depcrateimpl_653 {
() => {
// Module: crate
// Provides: {"impl_653"}
// Dependencies: {}
impl < T > OrFail for Option < T > { type Output = T ; # [track_caller] fn or_fail (self) -> std :: result :: Result < T , TestAssertionFailure > { match self { Some (t) => Ok (t) , None => Err (TestAssertionFailure :: create (format ! ("called `Option::or_fail()` on a `Option::<{}>::None` value" , std :: any :: type_name ::< T > ()))) , } } }
};
}
