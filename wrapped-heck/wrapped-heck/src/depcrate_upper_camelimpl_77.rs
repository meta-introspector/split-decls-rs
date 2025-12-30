// Generated macro for impl_77 (impl)
macro_rules! Depcrate_upper_camelimpl_77 {
() => {
// Module: crate::upper_camel
// Provides: {"impl_77"}
// Dependencies: {}
impl < T : AsRef < str > > fmt :: Display for AsUpperCamelCase < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { transform (self . 0 . as_ref () , capitalize , | _ | Ok (()) , f) } }
};
}
