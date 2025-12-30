// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl SelectedProvider { fn from_env () -> Self { match env :: var ("BOGO_SHIM_PROVIDER") . ok () . as_deref () { None | Some ("aws-lc-rs") => Self :: AwsLcRs , # [cfg (feature = "fips")] Some ("aws-lc-rs-fips") => Self :: AwsLcRsFips , Some ("ring") => Self :: Ring , Some (other) => panic ! ("unrecognized value for BOGO_SHIM_PROVIDER: {other:?}") , } } fn provider (& self) -> CryptoProvider { match self { Self :: AwsLcRs | Self :: AwsLcRsFips => { CryptoProvider { kx_groups : Cow :: Borrowed (aws_lc_rs :: ALL_KX_GROUPS) , tls12_cipher_suites : Cow :: Borrowed (aws_lc_rs :: ALL_TLS12_CIPHER_SUITES) , tls13_cipher_suites : Cow :: Borrowed (aws_lc_rs :: ALL_TLS13_CIPHER_SUITES) , .. aws_lc_rs :: DEFAULT_PROVIDER } } Self :: Ring => ring :: DEFAULT_PROVIDER , } } fn supports_ech (& self) -> bool { match * self { Self :: AwsLcRs | Self :: AwsLcRsFips => true , Self :: Ring => false , } } }
};
}
