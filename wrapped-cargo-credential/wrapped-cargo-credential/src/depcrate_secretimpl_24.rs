// Generated macro for impl_24 (impl)
macro_rules! Depcrate_secretimpl_24 {
() => {
// Module: crate::secret
// Provides: {"impl_24"}
// Dependencies: {}
impl < T : ToOwned + ? Sized > Secret < & T > { # [doc = " Converts a `Secret` containing a borrowed type to a `Secret` containing the"] # [doc = " corresponding owned type."] # [doc = " ```"] # [doc = " # use cargo_credential::Secret;"] # [doc = " let borrowed: Secret<&str> = Secret::from(\"token\");"] # [doc = " let owned: Secret<String> = borrowed.to_owned();"] # [doc = " ```"] pub fn to_owned (& self) -> Secret < < T as ToOwned > :: Owned > { Secret :: from (self . inner . to_owned ()) } }
};
}
