// Generated macro for impl_for_iri (macro)
macro_rules! Depcrate_convertimpl_for_iri {
() => {
// Module: crate::convert
// Provides: {"impl_for_iri"}
// Dependencies: {}
# [doc = " Implement conversions for an IRI string type."] macro_rules ! impl_for_iri { ($ borrowed : ident , $ owned : ident , $ owned_uri : ident) => { impl < S : Spec > fmt :: Display for MappedToUri <'_ , $ borrowed < S >> { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { write_percent_encoded (f , self . 0 . as_str ()) } } # [cfg (feature = "alloc")] impl < S : Spec > ToDedicatedString for MappedToUri <'_ , $ borrowed < S >> { type Target = $ owned_uri ; fn try_to_dedicated_string (& self) -> Result < Self :: Target , TryReserveError > { let s = self . try_to_string () ?; Ok (TryFrom :: try_from (s) . expect ("[validity] the IRI must be encoded into a valid URI")) } } impl <'a , S : Spec > From <&'a $ borrowed < S >> for MappedToUri <'a , $ borrowed < S >> { # [inline] fn from (iri : &'a $ borrowed < S >) -> Self { Self (iri) } } # [cfg (feature = "alloc")] impl <'a , S : Spec > From <&'a $ owned < S >> for MappedToUri <'a , $ borrowed < S >> { # [inline] fn from (iri : &'a $ owned < S >) -> Self { Self (iri . as_slice ()) } } } ; }
};
}
