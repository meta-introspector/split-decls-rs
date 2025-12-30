// Generated macro for _initialize_providers (function)
macro_rules! Depcrate_initialize_providers {
() => {
// Module: crate
// Provides: {"_initialize_providers"}
// Dependencies: {}
# [cfg (not (any (CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] fn _initialize_providers (py : pyo3 :: Python < '_ >) -> CryptographyResult < LoadedProviders > { let load_legacy = ! cfg ! (CRYPTOGRAPHY_BUILD_OPENSSL_NO_LEGACY) && ! env :: var ("CRYPTOGRAPHY_OPENSSL_NO_LEGACY") . is_ok_and (| v | ! v . is_empty () && v != "0") ; let legacy = if load_legacy { let legacy_result = provider :: Provider :: load (None , "legacy") ; if legacy_result . is_err () { let message = c"OpenSSL 3's legacy provider failed to load. Legacy algorithms will not be available. If you need those algorithms, check your OpenSSL configuration." ; let warning_cls = pyo3 :: exceptions :: PyWarning :: type_object (py) . into_any () ; pyo3 :: PyErr :: warn (py , & warning_cls , message , 1) ? ; None } else { Some (legacy_result ?) } } else { None } ; let _default = provider :: Provider :: load (None , "default") ? ; Ok (LoadedProviders { legacy , _default , fips : None , }) }
};
}
