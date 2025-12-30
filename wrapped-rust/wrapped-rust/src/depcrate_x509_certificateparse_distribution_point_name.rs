// Generated macro for parse_distribution_point_name (function)
macro_rules! Depcrate_x509_certificateparse_distribution_point_name {
() => {
// Module: crate::x509::certificate
// Provides: {"parse_distribution_point_name"}
// Dependencies: {}
pub (crate) fn parse_distribution_point_name < 'p > (py : pyo3 :: Python < 'p > , dp : DistributionPointName < 'p , Asn1Read > ,) -> CryptographyResult < (pyo3 :: Bound < 'p , pyo3 :: PyAny > , pyo3 :: Bound < 'p , pyo3 :: PyAny >) > { Ok (match dp { DistributionPointName :: FullName (data) => (x509 :: parse_general_names (py , & data) ? , py . None () . into_bound (py) ,) , DistributionPointName :: NameRelativeToCRLIssuer (data) => { (py . None () . into_bound (py) , x509 :: parse_rdn (py , & data) ?) } }) }
};
}
