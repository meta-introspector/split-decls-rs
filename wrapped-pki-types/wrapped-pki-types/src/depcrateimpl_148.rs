// Generated macro for impl_148 (impl)
macro_rules! Depcrateimpl_148 {
() => {
// Module: crate
// Provides: {"impl_148"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl PemObject for PrivateKeyDer < 'static > { fn from_pem (kind : SectionKind , value : Vec < u8 >) -> Option < Self > { match kind { SectionKind :: RsaPrivateKey => Some (Self :: Pkcs1 (value . into ())) , SectionKind :: EcPrivateKey => Some (Self :: Sec1 (value . into ())) , SectionKind :: PrivateKey => Some (Self :: Pkcs8 (value . into ())) , _ => None , } } }
};
}
