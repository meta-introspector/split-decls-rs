// Generated macro for find_in_pem (function)
macro_rules! Depcrate_x509_commonfind_in_pem {
() => {
// Module: crate::x509::common
// Provides: {"find_in_pem"}
// Dependencies: {}
# [doc = " Parse all sections in a PEM file and return the first matching section."] # [doc = " If no matching sections are found, return an error."] pub (crate) fn find_in_pem (data : & [u8] , filter_fn : fn (& pem :: Pem) -> bool , no_match_err : & 'static str ,) -> Result < pem :: Pem , CryptographyError > { let all_sections = pem :: parse_many (data) ? ; if all_sections . is_empty () { return Err (CryptographyError :: from (pem :: PemError :: MalformedFraming)) ; } all_sections . into_iter () . find (filter_fn) . ok_or_else (| | { CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err (no_match_err)) }) }
};
}
