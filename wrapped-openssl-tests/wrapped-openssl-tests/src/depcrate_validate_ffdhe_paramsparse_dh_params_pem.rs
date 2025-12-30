// Generated macro for parse_dh_params_pem (function)
macro_rules! Depcrate_validate_ffdhe_paramsparse_dh_params_pem {
() => {
// Module: crate::validate_ffdhe_params
// Provides: {"parse_dh_params_pem"}
// Dependencies: {}
# [doc = " Parse PEM-encoded DH parameters, returning `(p, g)`"] fn parse_dh_params_pem (data : & [u8]) -> (Vec < u8 > , Vec < u8 >) { let output_str = str :: from_utf8 (data) . unwrap () ; let output_str_lines = output_str . lines () . collect :: < Vec < _ > > () ; assert_eq ! (output_str_lines [0] , "-----BEGIN DH PARAMETERS-----") ; let last_line = output_str_lines . iter () . enumerate () . find (| (_i , l) | * * l == "-----END DH PARAMETERS-----") . unwrap () . 0 ; let stripped = & output_str_lines [1 .. last_line] ; let base64_encoded = stripped . iter () . fold (String :: new () , | acc , l | acc + l) ; let base64_decoded = BASE64_STANDARD . decode (base64_encoded) . unwrap () ; let res : asn1 :: ParseResult < _ > = asn1 :: parse (& base64_decoded , | d | { d . read_element :: < asn1 :: Sequence < '_ > > () ? . parse (| d | { let p = d . read_element :: < asn1 :: BigUint < '_ > > () ? ; let g = d . read_element :: < asn1 :: BigUint < '_ > > () ? ; Ok ((p , g)) }) }) ; let res = res . unwrap () ; (res . 0 . as_bytes () . to_vec () , res . 1 . as_bytes () . to_vec ()) }
};
}
