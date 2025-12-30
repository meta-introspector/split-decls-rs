// Generated macro for encode_distribution_point_reasons (function)
macro_rules! Depcrate_x509_certificateencode_distribution_point_reasons {
() => {
// Module: crate::x509::certificate
// Provides: {"encode_distribution_point_reasons"}
// Dependencies: {}
pub (crate) fn encode_distribution_point_reasons (py : pyo3 :: Python < '_ > , py_reasons : & pyo3 :: Bound < '_ , pyo3 :: PyAny > ,) -> pyo3 :: PyResult < asn1 :: OwnedBitString > { let reason_flag_mapping = types :: CRL_REASON_FLAGS . get (py) ? ; let mut bits = vec ! [0 , 0] ; for py_reason in py_reasons . try_iter () ? { let bit = reason_flag_mapping . get_item (py_reason ?) ? . extract :: < usize > () ? ; set_bit (& mut bits , bit , true) ; } if bits [1] == 0 { bits . truncate (1) ; } let unused_bits = bits . last () . unwrap () . trailing_zeros () as u8 ; Ok (asn1 :: OwnedBitString :: new (bits , unused_bits) . unwrap ()) }
};
}
