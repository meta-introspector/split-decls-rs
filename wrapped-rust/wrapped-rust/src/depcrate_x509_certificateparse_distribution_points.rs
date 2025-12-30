// Generated macro for parse_distribution_points (function)
macro_rules! Depcrate_x509_certificateparse_distribution_points {
() => {
// Module: crate::x509::certificate
// Provides: {"parse_distribution_points"}
// Dependencies: {}
pub (crate) fn parse_distribution_points < 'p > (py : pyo3 :: Python < 'p > , ext : & Extension < '_ > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let dps = ext . value :: < asn1 :: SequenceOf < '_ , DistributionPoint < '_ , Asn1Read > > > () ? ; let py_dps = pyo3 :: types :: PyList :: empty (py) ; for dp in dps { let py_dp = parse_distribution_point (py , dp) ? ; py_dps . append (py_dp) ? ; } Ok (py_dps . into_any ()) }
};
}
