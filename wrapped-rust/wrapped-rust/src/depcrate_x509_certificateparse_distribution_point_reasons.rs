// Generated macro for parse_distribution_point_reasons (function)
macro_rules! Depcrate_x509_certificateparse_distribution_point_reasons {
() => {
// Module: crate::x509::certificate
// Provides: {"parse_distribution_point_reasons"}
// Dependencies: {}
pub (crate) fn parse_distribution_point_reasons < 'p > (py : pyo3 :: Python < 'p > , reasons : Option < & asn1 :: BitString < '_ > > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let reason_bit_mapping = types :: REASON_BIT_MAPPING . get (py) ? ; Ok (match reasons { Some (bs) => { let mut vec = Vec :: new () ; for i in 1 ..= 8 { if bs . has_bit_set (i) { vec . push (reason_bit_mapping . get_item (i) ?) ; } } pyo3 :: types :: PyFrozenSet :: new (py , & vec) ? . into_any () } None => py . None () . into_bound (py) , }) }
};
}
