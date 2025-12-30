// Generated macro for impl_125 (impl)
macro_rules! Depcrate_asn1impl_125 {
() => {
// Module: crate::asn1
// Provides: {"impl_125"}
// Dependencies: {}
impl fmt :: Display for Asn1ObjectRef { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { unsafe { let mut buf = [0 ; 80] ; let len = ffi :: OBJ_obj2txt (buf . as_mut_ptr () as * mut _ , buf . len () as c_int , self . as_ptr () , 0 ,) ; match str :: from_utf8 (& buf [.. len as usize]) { Err (_) => fmt . write_str ("error") , Ok (s) => fmt . write_str (s) , } } } }
};
}
