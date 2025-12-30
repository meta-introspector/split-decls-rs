// Generated macro for impl_53 (impl)
macro_rules! Depcrate_needleimpl_53 {
() => {
// Module: crate::needle
// Provides: {"impl_53"}
// Dependencies: {}
impl Needle for Eof { fn check (& self , buf : & [u8] , eof : bool) -> Result < Vec < Match > , Error > { match eof { true => Ok (vec ! [Match :: new (0 , buf . len ())]) , false => Ok (Vec :: new ()) , } } }
};
}
