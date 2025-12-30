// Generated macro for parse_cp (function)
macro_rules! Depcrate_x509_certificateparse_cp {
() => {
// Module: crate::x509::certificate
// Provides: {"parse_cp"}
// Dependencies: {}
fn parse_cp < 'p > (py : pyo3 :: Python < 'p > , ext : & Extension < '_ > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let cp = ext . value :: < asn1 :: SequenceOf < '_ , PolicyInformation < '_ , Asn1Read > > > () ? ; let certificate_policies = pyo3 :: types :: PyList :: empty (py) ; for policyinfo in cp { let pi_oid = oid_to_py_oid (py , & policyinfo . policy_identifier) ? ; let py_pqis = match policyinfo . policy_qualifiers { Some (policy_qualifiers) => parse_policy_qualifiers (py , & policy_qualifiers) ? , None => py . None () . into_bound (py) , } ; let pi = types :: POLICY_INFORMATION . get (py) ? . call1 ((pi_oid , py_pqis)) ? ; certificate_policies . append (pi) ? ; } Ok (certificate_policies . into_any ()) }
};
}
