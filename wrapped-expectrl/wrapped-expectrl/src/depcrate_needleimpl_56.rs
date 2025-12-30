// Generated macro for impl_56 (impl)
macro_rules! Depcrate_needleimpl_56 {
() => {
// Module: crate::needle
// Provides: {"impl_56"}
// Dependencies: {}
impl Needle for NBytes { fn check (& self , buf : & [u8] , _ : bool) -> Result < Vec < Match > , Error > { match buf . len () >= self . count () { true => Ok (vec ! [Match :: new (0 , self . count ())]) , false => Ok (Vec :: new ()) , } } }
};
}
