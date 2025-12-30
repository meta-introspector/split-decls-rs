// Generated macro for get_ffdhe_params_from_openssl (function)
macro_rules! Depcrate_validate_ffdhe_paramsget_ffdhe_params_from_openssl {
() => {
// Module: crate::validate_ffdhe_params
// Provides: {"get_ffdhe_params_from_openssl"}
// Dependencies: {}
# [doc = " Get FFDHE parameters `(p, g)` for the given `ffdhe_group` from OpenSSL"] fn get_ffdhe_params_from_openssl (ffdhe_group : NamedGroup) -> (Vec < u8 > , Vec < u8 >) { let group = match ffdhe_group { NamedGroup :: FFDHE2048 => "group:ffdhe2048" , NamedGroup :: FFDHE3072 => "group:ffdhe3072" , NamedGroup :: FFDHE4096 => "group:ffdhe4096" , NamedGroup :: FFDHE6144 => "group:ffdhe6144" , NamedGroup :: FFDHE8192 => "group:ffdhe8192" , _ => panic ! ("not an ffdhe group: {ffdhe_group:?}") , } ; let openssl_output = std :: process :: Command :: new ("openssl") . args (["genpkey" , "-genparam" , "-algorithm" , "DH" , "-text" , "-pkeyopt" , group ,]) . output () . unwrap () ; parse_dh_params_pem (& openssl_output . stdout) }
};
}
