// Generated macro for impl_206 (impl)
macro_rules! Depcrate_ordimpl_206 {
() => {
// Module: crate::ord
// Provides: {"impl_206"}
// Dependencies: {}
impl < T > DerOrd for T where T : EncodeValue + ValueOrd + Tagged , { fn der_cmp (& self , other : & Self) -> Result < Ordering > { match (self . header () , other . header ()) { (Ok (this) , Ok (that)) => { let cmp_result = this . der_cmp (& that) ; match cmp_result { Err (err) => Err (err) , Ok (Ordering :: Equal) => self . value_cmp (other) , Ok (ordering) => Ok (ordering) , } } (Err (err) , _) => Err (err) , (_ , Err (err)) => Err (err) , } } }
};
}
