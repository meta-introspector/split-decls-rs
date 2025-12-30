// Generated macro for impl_1105 (impl)
macro_rules! Depcrate_valueimpl_1105 {
() => {
// Module: crate::value
// Provides: {"impl_1105"}
// Dependencies: {}
impl < S : ScalarValue > fmt :: Display for Value < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Null => write ! (f , "null") , Self :: Scalar (s) => fmt :: Display :: fmt (< & Scalar < _ > > :: from (s) , f) , Self :: List (list) => { write ! (f , "[") ? ; for (idx , item) in list . iter () . enumerate () { write ! (f , "{item}") ? ; if idx < list . len () - 1 { write ! (f , ", ") ? ; } } write ! (f , "]") ? ; Ok (()) } Self :: Object (obj) => { write ! (f , "{{") ? ; for (idx , (key , value)) in obj . iter () . enumerate () { write ! (f , "\"{key}\": {value}") ? ; if idx < obj . field_count () - 1 { write ! (f , ", ") ? ; } } write ! (f , "}}") ? ; Ok (()) } } } }
};
}
