// Generated macro for impl_80 (impl)
macro_rules! Depcrate_asn1impl_80 {
() => {
// Module: crate::asn1
// Provides: {"impl_80"}
// Dependencies: {}
impl fmt :: Display for Asn1GeneralizedTimeRef { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { unsafe { let mem_bio = match MemBio :: new () { Err (_) => return f . write_str ("error") , Ok (m) => m , } ; let print_result = cvt (ffi :: ASN1_GENERALIZEDTIME_print (mem_bio . as_ptr () , self . as_ptr () ,)) ; match print_result { Err (_) => f . write_str ("error") , Ok (_) => f . write_str (str :: from_utf8_unchecked (mem_bio . get_buf ())) , } } } }
};
}
