// Generated macro for enable_fips (function)
macro_rules! Depcrateenable_fips {
() => {
// Module: crate
// Provides: {"enable_fips"}
// Dependencies: {}
# [cfg (not (any (CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] # [pyo3 :: pyfunction] fn enable_fips (providers : & mut LoadedProviders) -> CryptographyResult < () > { providers . fips = Some (provider :: Provider :: load (None , "fips") ?) ; cryptography_openssl :: fips :: enable () ? ; Ok (()) }
};
}
