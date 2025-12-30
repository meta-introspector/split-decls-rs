// Generated macro for get_default_or_install_from_crate_features (function)
macro_rules! Depcrate_crypto_providerget_default_or_install_from_crate_features {
() => {
// Module: crate::crypto_provider
// Provides: {"get_default_or_install_from_crate_features"}
// Dependencies: {}
pub (crate) fn get_default_or_install_from_crate_features () -> Option < Arc < CryptoProvider > > { if let Some (provider) = CryptoProvider :: get_default () { return Some (provider . clone ()) ; } let _ = provider_from_crate_features () ? . install_default () ; Some (CryptoProvider :: get_default () . unwrap () . clone ()) }
};
}
