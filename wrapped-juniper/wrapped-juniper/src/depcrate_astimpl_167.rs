// Generated macro for impl_167 (impl)
macro_rules! Depcrate_astimpl_167 {
() => {
// Module: crate::ast
// Provides: {"impl_167"}
// Dependencies: {}
impl < S : ScalarValue > fmt :: Display for InputValue < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Null => write ! (f , "null") , Self :: Scalar (s) => fmt :: Display :: fmt (< & Scalar < _ > > :: from (s) , f) , Self :: Enum (v) => write ! (f , "{v}") , Self :: Variable (v) => write ! (f , "${v}") , Self :: List (v) => { write ! (f , "[") ? ; for (i , spanning) in v . iter () . enumerate () { spanning . item . fmt (f) ? ; if i < v . len () - 1 { write ! (f , ", ") ? ; } } write ! (f , "]") } Self :: Object (o) => { write ! (f , "{{") ? ; for (i , (k , v)) in o . iter () . enumerate () { write ! (f , "{}: " , k . item) ? ; v . item . fmt (f) ? ; if i < o . len () - 1 { write ! (f , ", ") ? ; } } write ! (f , "}}") } } } }
};
}
