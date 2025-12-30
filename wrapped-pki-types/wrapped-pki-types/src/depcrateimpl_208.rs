// Generated macro for impl_208 (impl)
macro_rules! Depcrateimpl_208 {
() => {
// Module: crate
// Provides: {"impl_208"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl EchConfigListBytes < 'static > { # [doc = " Convert an iterator over PEM items into an `EchConfigListBytes` and private key."] # [doc = ""] # [doc = " This handles the \"ECHConfig file\" format specified in"] # [doc = " <https://www.ietf.org/archive/id/draft-farrell-tls-pemesni-05.html#name-echconfig-file>"] # [doc = ""] # [doc = " Use it like:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(all(feature = \"alloc\", feature = \"std\"))] {"] # [doc = " # use rustls_pki_types::{EchConfigListBytes, pem::PemObject};"] # [doc = " let (config, key) = EchConfigListBytes::config_and_key_from_iter("] # [doc = "     PemObject::pem_file_iter(\"tests/data/ech.pem\").unwrap()"] # [doc = " ).unwrap();"] # [doc = " # }"] # [doc = " ```"] pub fn config_and_key_from_iter (iter : impl Iterator < Item = Result < (SectionKind , Vec < u8 >) , pem :: Error > > ,) -> Result < (Self , PrivatePkcs8KeyDer < 'static >) , pem :: Error > { let mut key = None ; let mut config = None ; for item in iter { let (kind , data) = item ? ; match kind { SectionKind :: PrivateKey => { key = PrivatePkcs8KeyDer :: from_pem (kind , data) ; } SectionKind :: EchConfigList => { config = Self :: from_pem (kind , data) ; } _ => continue , } ; if let (Some (_key) , Some (_config)) = (& key , & config) { return Ok ((config . take () . unwrap () , key . take () . unwrap ())) ; } } Err (pem :: Error :: NoItemsFound) } }
};
}
