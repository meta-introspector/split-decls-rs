// Generated macro for impl_18 (impl)
macro_rules! Depcrate_lower_camelimpl_18 {
() => {
// Module: crate::lower_camel
// Provides: {"impl_18"}
// Dependencies: {}
impl < T : AsRef < str > > fmt :: Display for AsLowerCamelCase < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut first = true ; transform (self . 0 . as_ref () , | s , f | { if first { first = false ; lowercase (s , f) } else { capitalize (s , f) } } , | _ | Ok (()) , f ,) } }
};
}
