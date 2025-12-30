// Generated macro for impl_216 (impl)
macro_rules! Depcrateimpl_216 {
() => {
// Module: crate
// Provides: {"impl_216"}
// Dependencies: {}
impl < 'a > From < Cow < 'a , str > > for Value { # [inline] fn from (v : Cow < 'a , str >) -> Self { Self :: String (Utf8String :: from (v)) } }
};
}
