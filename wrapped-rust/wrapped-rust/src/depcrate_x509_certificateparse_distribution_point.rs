// Generated macro for parse_distribution_point (function)
macro_rules! Depcrate_x509_certificateparse_distribution_point {
() => {
// Module: crate::x509::certificate
// Provides: {"parse_distribution_point"}
// Dependencies: {}
fn parse_distribution_point < 'p > (py : pyo3 :: Python < 'p > , dp : DistributionPoint < 'p , Asn1Read > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let (full_name , relative_name) = match dp . distribution_point { Some (data) => parse_distribution_point_name (py , data) ? , None => (py . None () . into_bound (py) , py . None () . into_bound (py)) , } ; let reasons = parse_distribution_point_reasons (py , dp . reasons . as_ref ()) ? ; let crl_issuer = match dp . crl_issuer { Some (aci) => x509 :: parse_general_names (py , & aci) ? , None => py . None () . into_bound (py) , } ; Ok (types :: DISTRIBUTION_POINT . get (py) ? . call1 ((full_name , relative_name , reasons , crl_issuer)) ?) }
};
}
