// Generated macro for impl_913 (impl)
macro_rules! Depcrate_ssl_bioimpl_913 {
() => {
// Module: crate::ssl::bio
// Provides: {"impl_913"}
// Dependencies: {}
impl BioMethod { fn new < S : Read + Write > () -> Result < BioMethod , ErrorStack > { BIO_METHOD :: new :: < S > () . map (BioMethod) } }
};
}
