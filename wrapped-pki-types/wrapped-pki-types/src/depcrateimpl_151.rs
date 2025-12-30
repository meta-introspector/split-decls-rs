// Generated macro for impl_151 (impl)
macro_rules! Depcrateimpl_151 {
() => {
// Module: crate
// Provides: {"impl_151"}
// Dependencies: {}
impl < 'a > From < PrivatePkcs8KeyDer < 'a > > for PrivateKeyDer < 'a > { fn from (key : PrivatePkcs8KeyDer < 'a >) -> Self { Self :: Pkcs8 (key) } }
};
}
