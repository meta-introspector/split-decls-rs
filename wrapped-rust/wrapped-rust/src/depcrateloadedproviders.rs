// Generated macro for LoadedProviders (struct)
macro_rules! DepcrateLoadedProviders {
() => {
// Module: crate
// Provides: {"LoadedProviders"}
// Dependencies: {}
# [cfg (not (any (CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] # [pyo3 :: pyclass (module = "cryptography.hazmat.bindings._rust")] struct LoadedProviders { legacy : Option < provider :: Provider > , _default : provider :: Provider , fips : Option < provider :: Provider > , }
};
}
