// Generated macro for impl_1547 (impl)
macro_rules! Depcrate_x509impl_1547 {
() => {
// Module: crate::x509
// Provides: {"impl_1547"}
// Dependencies: {}
impl fmt :: Debug for X509 { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let serial = match & self . serial_number () . to_bn () { Ok (bn) => match bn . to_hex_str () { Ok (hex) => hex . to_string () , Err (_) => "" . to_string () , } , Err (_) => "" . to_string () , } ; let mut debug_struct = formatter . debug_struct ("X509") ; debug_struct . field ("serial_number" , & serial) ; debug_struct . field ("signature_algorithm" , & self . signature_algorithm () . object ()) ; debug_struct . field ("issuer" , & self . issuer_name ()) ; debug_struct . field ("subject" , & self . subject_name ()) ; if let Some (subject_alt_names) = & self . subject_alt_names () { debug_struct . field ("subject_alt_names" , subject_alt_names) ; } debug_struct . field ("not_before" , & self . not_before ()) ; debug_struct . field ("not_after" , & self . not_after ()) ; if let Ok (public_key) = & self . public_key () { debug_struct . field ("public_key" , public_key) ; } ; debug_struct . finish () } }
};
}
